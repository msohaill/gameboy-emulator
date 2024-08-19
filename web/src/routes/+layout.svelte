<script lang="ts">
  import { fly } from 'svelte/transition';
  import { dev, browser } from '$app/environment';
  import '../app.css';
  import { onMount } from 'svelte';

  const isReducedMotion = browser && matchMedia('(prefers-reduced-motion: reduce)').matches;
  let visible = false;
  onMount(() => (visible = true));
</script>

<svelte:head>
  {#if !dev}
    <!-- Global site tag (gtag.js) - Google Analytics -->
  {/if}
</svelte:head>

{#if isReducedMotion}
  <main>
    <slot />
  </main>
{:else if visible}
  <main in:fly={{ y: 50, duration: 350, delay: 350 }}>
    <slot />
  </main>
{/if}
