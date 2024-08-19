<script lang="ts">
  import init, { NeoNES, wasm_memory } from 'neones-web';
  import audioWorker from '../audio-worklet?worker&url';
  import { Application, BufferImageSource, Sprite, Texture } from 'pixi.js';
  import { onDestroy, onMount } from 'svelte';
  import { fly } from 'svelte/transition';

  export let rom: Uint8Array;
  export let paused: boolean = false;
  export let muted: boolean = false;

  const KEYS = ['KeyW', 'KeyA', 'KeyS', 'KeyD', 'KeyN', 'KeyM', 'Enter', 'Space'];
  const width = 256;
  const height = 240;

  let nes: NeoNES;
  let canvas: HTMLCanvasElement;
  let worklet: AudioWorkletNode;
  let source: BufferImageSource;
  let context: AudioContext;
  let frameId: number;
  let mounted = false;
  let visible = true;

  onDestroy(() => {
    cancelAnimationFrame(frameId);
    worklet.disconnect();
    context.close();
  });

  const safeExecute = (executable: () => void) => {
    try {
      executable();
      return false;
    } catch (e) {
      console.error(e);
      return true;
    }
  };

  onMount(async () => {
    await init();
    await setUpAudio();

    if (safeExecute(() => startUp())) {
      return;
    }

    const app = new Application();
    app.init({ width, height, canvas, resolution: 2.5 });
    app.stage.addChild(Sprite.from(new Texture({ source })));
    document.onvisibilitychange = () => (visible = !document.hidden);

    mounted = true;
    paused = false;
  });

  const pause = () => {
    cancelAnimationFrame(frameId);
    mute();
  };

  const play = () => {
    unmute();
    render();
  };

  const mute = () => {
    context.suspend();
  };

  const unmute = () => {
    context.resume();
  };

  const getFrame = () => new Uint8Array(wasm_memory().buffer, nes.frame(), 4 * 256 * 240);

  const render = () => {
    frameId = requestAnimationFrame(render);

    if (paused || !visible) pause();

    if (safeExecute(() => nes.step())) {
      cancelAnimationFrame(frameId);
    }

    source.resource = getFrame();
    source.update();
  };

  const setUpAudio = async () => {
    if (!window.AudioContext) {
      return;
    }

    context = new window.AudioContext({ sampleRate: 44100 });
    await context.audioWorklet.addModule(audioWorker);
    worklet = new AudioWorkletNode(context, 'nes-audio-processor', {
      numberOfInputs: 0,
      outputChannelCount: [1],
    });
    worklet.connect(context.destination);
  };

  const startUp = () => {
    nes = new NeoNES(new Uint8Array(rom));
    source = new BufferImageSource({ resource: getFrame(), width, height, scaleMode: 'nearest' });

    worklet.port.onmessage = event => {
      let samples = new Float32Array(event.data.len);
      nes.signal(samples);

      if (!muted) worklet.port.postMessage({ samples });
    };
  };

  const handleButton = (e: KeyboardEvent, pushed: boolean) => {
    if (nes && !paused && KEYS.includes(e.code)) {
      e.preventDefault();
      if (pushed) nes.push(e.code);
      else nes.release(e.code);
    }
  }

  $: if (!paused && visible && mounted) play();
</script>

<svelte:window
  on:keydown={e => handleButton(e, true)}
  on:keyup={e => handleButton(e, false)}
/>
<div in:fly={{ x: 50, duration: 350, delay: 350 }} out:fly={{ x: -50, duration: 350, delay: 0 }}>
  <canvas bind:this={canvas} />
</div>
