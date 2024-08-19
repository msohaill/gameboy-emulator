class NESAudioProcessor extends AudioWorkletProcessor {
  private samples: number[] = [];

  constructor() {
    super();
    this.port.onmessage = event => this.samples.push(...event.data.samples);
  }

  override process(
    inputs: Float32Array[][],
    outputs: Float32Array[][],
    parameters: Record<string, Float32Array>,
  ) {
    const out = outputs[0][0];
    this.port.postMessage({ len: out.length });

    for (let i = 0; i < out.length; i++) {
      let sample = this.samples.shift();

      if (sample == undefined) out[i] = 0;
      else out[i] = sample;
    }

    return true;
  }
}

registerProcessor('nes-audio-processor', NESAudioProcessor);
