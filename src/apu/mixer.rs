use std::{mem::MaybeUninit, sync::Arc};

use ringbuf::{
  storage::Owning,
  traits::{Observer, Producer as _, Split},
  wrap::caching::Caching,
  SharedRb,
  StaticRb,
};

use crate::cpu::CPU;

use super::filter::{Filter, FilterKind};

type Producer = Caching<Arc<SharedRb<Owning<[MaybeUninit<f32>; BUFFER_SIZE]>>>, true, false>;
pub type Consumer = Caching<Arc<SharedRb<Owning<[MaybeUninit<f32>; BUFFER_SIZE]>>>, false, true>;

pub struct Mixer {
  producer: Producer,
  consumer: Option<Consumer>,
  sampling: Sampling,
  filters: [Filter; 3],
}

struct Sampling {
  fraction: f32,
  average: f32,
  count: f32,
}

impl Sampling {
  fn new() -> Self {
    Sampling {
      fraction: 0.0,
      average: 0.0,
      count: 0.0,
    }
  }
}

pub const BUFFER_SIZE: usize = Mixer::BUFFER_SIZE;

impl Mixer {
  pub const OUTPUT_FREQ: f32 = 44100.0;
  pub const BUFFER_SIZE: usize = 4096;
  const SAMPLE_RATE: f32 = CPU::CLOCK_RATE / Mixer::OUTPUT_FREQ;

  pub fn new() -> Self {
    let buffer = StaticRb::<f32, BUFFER_SIZE>::default();
    let (producer, consumer) = buffer.split();

    Mixer {
      producer,
      consumer: Some(consumer),
      sampling: Sampling::new(),
      filters: [
        Filter::new(Mixer::OUTPUT_FREQ, 90.0, FilterKind::HighPass),
        Filter::new(Mixer::OUTPUT_FREQ, 440.0, FilterKind::HighPass),
        Filter::new(Mixer::OUTPUT_FREQ, 14000.0, FilterKind::LowPass)
      ],
    }
  }

  pub fn consume(&mut self, samples: &Vec<f32>) {
    let pitch_ratio = {
      let size = self.producer.occupied_len() as f32;
      let capacity = self.producer.capacity().get() as f32;
      ((capacity - 2.0 * size) / capacity).mul_add(0.001, 1.0)
    };

    let decim = Mixer::SAMPLE_RATE / pitch_ratio;

    for sample in samples {
      self.sampling.average = *sample;
      self.sampling.count = 1.0;

      while self.sampling.fraction <= 0.0 {
        let sample = self.filters.iter_mut()
          .fold(self.sampling.average / self.sampling.count, |s, filter| filter.process(s));
        if self.producer.try_push(sample).is_err() {
          std::thread::sleep(std::time::Duration::from_micros(10));
        }

        self.sampling.average = 0.0;
        self.sampling.count = 0.0;
        self.sampling.fraction += decim;
      }

      self.sampling.fraction -= 1.0;
    }
  }

  pub fn consumer(&mut self) -> Consumer {
    self.consumer.take().unwrap_or_else(|| panic!("Can only obtain audio consumer once."))
  }
}
