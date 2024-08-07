use std::f32::consts::PI;

pub struct Filter {
  b0: f32,
  b1: f32,
  a1: f32,
  prev_x: f32,
  prev_y: f32,
}

pub enum FilterKind {
  LowPass,
  HighPass,
}

impl Filter {
  pub fn new(sample_rate: f32, cutoff: f32, kind: FilterKind) -> Self {
    let c = sample_rate / PI / cutoff;
    let a0i = 1.0 / (1.0 + c);

    let (b0, b1, a1, prev_x, prev_y) = match kind {
      FilterKind::LowPass => (a0i, a0i, (1.0 - c) * a0i, 0.0, 0.0),
      FilterKind::HighPass => (c * a0i, -c * a0i, (1.0 - c) * a0i, 0.0, 0.0),
    };

    Filter {
      b0,
      b1,
      a1,
      prev_x,
      prev_y
    }
  }


  pub fn process(&mut self, signal: f32) -> f32 {
    let y = self.b0 * signal + self.b1 * self.prev_x - self.a1 * self.prev_y;
    self.prev_y = y;
    self.prev_x = signal;
    y
  }
}
