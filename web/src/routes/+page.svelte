<script lang="ts">
  import init, { NES, wasm_memory } from 'neones-web';
  import audioWorker from '$lib/audio-worklet?worker&url';
  import * as pixi from 'pixi.js';
  import { onMount } from 'svelte';

  let nes: NES;
  let files: FileList;
  let canvas: HTMLCanvasElement;
  let app: pixi.Application;
  let source: pixi.BufferImageSource;
  const keys = ["KeyW", "KeyA", "KeyS", "KeyD", "KeyN", "KeyM", "Enter", "Space"];

  onMount(async () => {
    await init();
    app = new pixi.Application();
    app.init({ width: 256, height: 240, canvas, resolution: 2.5 });
  });

  const getFrame = () => new Uint8Array(wasm_memory().buffer, nes.frame(), 4 * 256 * 240);

  const render = () => {
    requestAnimationFrame(render);
    nes.step();

    source.resource = getFrame();
    source.update();
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

  const handleRom = async (files: FileList) => {
    let rom = await files[0].arrayBuffer();
    nes = new NES(new Uint8Array(rom));

    source = new pixi.BufferImageSource({ resource: getFrame(), width: 256, height: 240, scaleMode: 'nearest' });
    app.stage.addChild(pixi.Sprite.from(new pixi.Texture({ source })));

    await setUpAudio();
    render();
  }

  const push = (e: KeyboardEvent) => {
    if (nes && keys.includes(e.code)) nes.push(e.code);
  }

  const release = (e: KeyboardEvent) => {
    if (nes && keys.includes(e.code)) nes.release(e.code);
  }

  $: if (files) handleRom(files);
</script>

<svelte:window on:keydown={push} on:keyup={release} />
<input type="file" id="file-upload" accept=".nes" bind:files />
<canvas bind:this={canvas} />
