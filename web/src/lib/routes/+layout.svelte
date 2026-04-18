<script lang="ts">
  import '../app.css';
  import Nav    from '$lib/components/Nav.svelte';
  import Footer from '$lib/components/Footer.svelte';
</script>

<div class="layout">
  <Nav />
  <main class="main-content"><slot /></main>
  <Footer />
</div>

<style>
  .layout       { display: flex; flex-direction: column; min-height: 100vh; }
  .main-content { flex: 1; display: flex; flex-direction: column; }
</style>
