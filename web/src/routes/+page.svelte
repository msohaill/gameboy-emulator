<script lang="ts">
  import Controls from '$lib/components/Controls.svelte';
  import Modal from '$lib/components/Modal.svelte';
  import NeoNes from '$lib/components/NeoNES.svelte';
  import RomLoader from '$lib/components/RomLoader.svelte';
  import { Github, Pause, Play, X, Gamepad, RefreshCcw, VolumeX, Music } from 'lucide-svelte';

  let rom: { name: string; bytes: Uint8Array };
  let paused = false;
  let muted = false;
  let changingRom = false;
  let checkingControls = false;

  $: paused = changingRom || checkingControls;
</script>

<svelte:head>
  <title>NeoNES</title>
  <meta name="description" content="An NES emulator written in Rust!" />
  <meta property="og:title" content="NeoNES" />
  <meta property="og:description" content="An NES emulator written in Rust!" />
  <meta name="twitter:card" content="summary_large_image" />
</svelte:head>

<div class="flex flex-col items-center h-full">
  <button
    class="mx-auto text-center text-xl font-semibold italic"
    on:click={() => window.location.reload()}
  >
    <h1>
      <span class="font-[Neo] text-2xl">neoNES</span>
      &mdash; An emulator in Rust &nbsp;<span class="not-italic">🕹️</span>
    </h1>
  </button>
  {#if rom}
    <div class="mt-7 flex flex-col">
      {#key rom.name}
        <NeoNes bind:rom={rom.bytes} bind:paused bind:muted />
      {/key}
      <div class="flex justify-center items-center gap-3 mt-4">
        <button on:click={() => (checkingControls = true)}><Gamepad /></button>
        <button on:click={() => (muted = !muted)}>
          {#if muted}
            <Music />
          {:else}
            <VolumeX />
          {/if}
        </button>
        <button on:click={() => (paused = !paused)}>
          {#if paused}
            <Play />
          {:else}
            <Pause />
          {/if}
        </button>
        <button
          class="hover:bg-[#1e1e26] p-2 rounded italic text-sm"
          on:click={() => (changingRom = true)}
          >Change ROM &nbsp;<RefreshCcw class="inline-block" size="16" /></button
        >
      </div>
      <Modal bind:visible={changingRom}>
        <RomLoader bind:rom on:selected={() => (changingRom = false)} />
      </Modal>
      <Modal bind:visible={checkingControls}>
        <Controls />
      </Modal>
    </div>
  {:else}
    <div class="mt-16 flex flex-col gap-20 lg:flex-row justify-between items-center w-full">
      <div class="flex flex-col gap-5 italic w-full lg:w-5/12">
        <p>Welcome! 👋</p>
        <p>
          neoNES is an emulator for the Nintendo Entertainment System ( <a
            class="link"
            target="_blank"
            rel="noopener noreferrer"
            href="https://en.wikipedia.org/wiki/Nintendo_Entertainment_System"
          >
            NES</a
          >
          ) written entirely in
          <a
            class="link"
            target="_blank"
            rel="noopener noreferrer"
            href="https://www.rust-lang.org/">Rust</a
          >
          and compiled over to
          <a class="link" target="_blank" rel="noopener noreferrer" href="https://webassembly.org/"
            >WebAssembly</a
          >
          using
          <a
            class="link"
            target="_blank"
            rel="noopener noreferrer"
            href="https://rustwasm.github.io/wasm-pack/">wasm-pack</a
          >.
        </p>
        <p>
          Visit the Github repository below if you are interested in the code! There are some
          freeware NES ROMs to get you started, but I'm sure you'll be able to scour the internet
          for some of your favourite games like Super Mario Brothers, Metroid, Pac-Man, and more!
        </p>
        <p>To play, simply use the following key bindings:</p>
        <Controls />
      </div>
      <RomLoader bind:rom />
    </div>
  {/if}
  <a
    class="self-end mt-10"
    target="_blank"
    rel="noopener noreferrer"
    href="https://github.com/msohaill/neones"
  >
    <Github />
  </a>
</div>
