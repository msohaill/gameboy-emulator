<script lang="ts">
  import init, { NeoNES, wasm_memory } from 'neones-web';
  import audioWorker from '../audio-worklet?worker&url';
  import { Application, BufferImageSource, Sprite, Texture } from 'pixi.js';
  import { onMount } from 'svelte';

  export let rom: Uint8Array;

  const KEYS = ["KeyW", "KeyA", "KeyS", "KeyD", "KeyN", "KeyM", "Enter", "Space"];
  const noise = new Uint8Array(Array(256 * 240 * 4));
  const width = 256;
  const height = 240;
  const source = new BufferImageSource({ resource: noise, width, height, scaleMode: 'nearest' });;

  let nes: NeoNES;
  let canvas: HTMLCanvasElement;
  let worklet: AudioWorkletNode;
  let context: AudioContext;

  onMount(async () => {
    await init();

    const app = new Application();
    app.init({ width, height, canvas, resolution: 2.5 });
    app.stage.addChild(Sprite.from(new Texture({ source })));

    render();
  });

  const generateNoise = () => {
    for (let i = 0; i < 256 * 240; i++) {
      let n = Math.random();
      noise[4 * i + 0] = n * 60;
      noise[4 * i + 1] = n * 40;
      noise[4 * i + 2] = n * 60;
    }
  }

  const getFrame = () => new Uint8Array(wasm_memory().buffer, nes.frame(), 4 * 256 * 240);

  const render = () => {
    requestAnimationFrame(render);

    if (nes) {
      nes.step();
      source.resource = getFrame();
      source.update();
    } else {
      generateNoise();
      source.update();
    }
  };

  const setUpAudio = async () => {
    if (!window.AudioContext || context) {
      return;
    }

    context = new window.AudioContext({ sampleRate: 44100 });
    await context.audioWorklet.addModule(audioWorker);
    worklet = new AudioWorkletNode(context, 'nes-audio-processor', {
      numberOfInputs: 0,
      outputChannelCount: [1],
    });
    worklet.connect(context.destination);
  }

  const startUp = async () => {
    nes = new NeoNES(new Uint8Array(rom));
    source.resource = getFrame();
    source.update();

    await setUpAudio();
    worklet.port.onmessage = (event) => {
      let samples = new Float32Array(event.data.len);
      nes.signal(samples);
      worklet.port.postMessage({ samples });
    }
  }

  const push = (e: KeyboardEvent) => {
    if (nes && KEYS.includes(e.code)) nes.push(e.code);
  }

  const release = (e: KeyboardEvent) => {
    if (nes && KEYS.includes(e.code)) nes.release(e.code);
  }

  $: if (rom) startUp();
</script>

<svelte:window on:keydown={push} on:keyup={release} />
<div>
  <canvas bind:this={canvas} />
</div>
