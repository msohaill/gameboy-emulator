<script lang="ts">
  import { X } from 'lucide-svelte';

  export let visible = false;
</script>

{#if visible}
  <dialog class="modal">
    <button class="absolute top-7 right-7" on:click={() => (visible = false)}><X /></button>
    <slot />
  </dialog>
{/if}

<style lang="postcss">
  .modal {
    @apply w-screen h-screen bg-[rgba(14,14,22,0.80)] fixed top-0 left-0;
    @apply flex justify-center items-center;
  }
</style>
