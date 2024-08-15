<script lang="ts">
  import NeoNes from '$lib/components/NeoNES.svelte';

  let rom: Uint8Array;
  let files: FileList;

  const handleRom = async (files: FileList) => {
    rom = new Uint8Array(await files[0].arrayBuffer());
  }

  $: if (files) handleRom(files);
</script>

<div style="display: flex; flex-direction: column; align-items: center; gap: 50px">
  <input type="file" id="file-upload" accept=".nes" bind:files />
  <NeoNes bind:rom />
</div>
