<script lang="ts">
  import HeroSection from '$lib/components/HeroSection.svelte';
  import Playground  from '$lib/components/Playground.svelte';
</script>

<svelte:head>
  <title>mdix-converter — JSON to DixScript Optimizer</title>
  <meta
    name="description"
    content="Paste JSON, TOML or YAML. Get optimised DixScript with automatic @ENUMS and @QUICKFUNCS induction. Runs 100% in your browser via WebAssembly."
  />
</svelte:head>

<HeroSection />
<Playground />
