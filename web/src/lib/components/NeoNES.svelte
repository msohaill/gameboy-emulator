<script lang="ts">
  import init, { NeoNES, wasm_memory } from 'neones-web';
  import audioWorker from './audio-worklet?worker&url';
  import { Application, BufferImageSource, Sprite, Texture } from 'pixi.js';
  import { onMount } from 'svelte';

  export let rom: Uint8Array;

  let nes: NeoNES;
  let canvas: HTMLCanvasElement;
  let app: Application;
  let source: BufferImageSource;

  const KEYS = ["KeyW", "KeyA", "KeyS", "KeyD", "KeyN", "KeyM", "Enter", "Space"];
  const noise = new Uint8Array(Array(256 * 240 * 4));

  onMount(async () => {
    await init();
    app = new Application();
    app.init({ width: 256, height: 240, canvas, resolution: 2.5 });

    source = new BufferImageSource({ resource: noise, width: 256, height: 240, scaleMode: 'nearest' });
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
      // generateNoise();
      // source.update();
    }
  };

  const setUpAudio = async () => {
    if (!window.AudioContext) {
      return;
    }

    const context = new window.AudioContext({ sampleRate: 44100 });
    await context.audioWorklet.addModule(audioWorker);
    const worklet = new AudioWorkletNode(context, 'nes-audio-processor', {
      numberOfInputs: 0,
      outputChannelCount: [1],
    });

    worklet.port.onmessage = (event) => {
      let samples = new Float32Array(event.data.len);
      nes.signal(samples);
      worklet.port.postMessage({ samples });
    }

    worklet.connect(context.destination);
  }

  const push = (e: KeyboardEvent) => {
    if (nes && KEYS.includes(e.code)) nes.push(e.code);
  }

  const release = (e: KeyboardEvent) => {
    if (nes && KEYS.includes(e.code)) nes.release(e.code);
  }

  $: if(rom.length) (async () => {
    nes = new NeoNES(new Uint8Array(rom));
    source.resource = getFrame();
    source.update();
    await setUpAudio();
  })();
</script>

<svelte:window on:keydown={push} on:keyup={release} />
<canvas bind:this={canvas} />
