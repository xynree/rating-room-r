<script>
  import '../app.css'
</script>

<main class="w-screen h-screen">
  <slot />
</main>

<style lang="postcss">
  a {
    @apply hover:text-gray-800 transition-all text-sm;
  }
</style>
