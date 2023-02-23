<script>
  import "../app.css"
</script>

<main class="container">
  <nav class="flex justify-between w-screen p-6 text-sm font-medium shadow-sm">
    <a href="/">my collection (# items)</a>
    <a href="/items/add_item" class="underline">+ add item</a>
  </nav>
  <slot />
</main>

<style lang="postcss">
  a {
    @apply hover:text-gray-800 transition-all;
  }
</style>
