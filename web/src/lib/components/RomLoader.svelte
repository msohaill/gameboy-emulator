<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { Buffer } from 'buffer';

  export let rom: { name: string; bytes: Uint8Array };
  let files: FileList;

  const freeware = import.meta.glob<boolean, string, string>('$static/roms/*', {
    query: '?hex',
    import: 'default',
  });
  const dispatch = createEventDispatcher();

  const romName = (name: string) => name.split(/[\\/]/).reverse()[0].slice(0, -4);

  const useFreeRom = async (selected: string) => {
    rom = {
      name: romName(selected),
      bytes: Uint8Array.from(Buffer.from(await freeware[selected](), 'hex')),
    };
    notify();
  };

  const handleRom = async (files: FileList) => {
    rom = { name: files[0].name, bytes: new Uint8Array(await files[0].arrayBuffer()) };
    notify();
  };

  const notify = () => {
    dispatch('selected');
  };

  $: if (files) handleRom(files);
</script>

<div
  class="flex flex-col w-3/5 lg:w-1/3 items-center border-white border-[0.5px] p-2 bg-[#0e0e16] text-center"
>
  <h1 class="text-md italic font-bold mb-4">Select a ROM</h1>

  <input type="file" id="file-upload" accept=".nes" hidden bind:files />
  <label class="rom mb-4" for="file-upload">Load your own ROM ...</label>

  <h2 class="text-md italic font-bold mb-4">Free-to-play ROMs</h2>

  {#each Object.keys(freeware) as freeRom}
    <button class="rom" on:click={() => useFreeRom(freeRom)}>{romName(freeRom)}</button>
  {/each}
</div>

<style lang="postcss">
  .rom {
    @apply py-1 px-4 text-sm w-full text-center font-thin hover:bg-[#1e1e26] cursor-pointer;
  }
</style>
