<script>
  import "../app.css"
  import { page } from "$app/stores"

  const url = $page.route.id
</script>

<main class="container">
  <nav class="flex justify-between w-screen p-6 font-medium shadow-sm">
    <a href="/">my collection</a>
    <a href="/items/add_item" class="underline">+ add item</a>
  </nav>
  <slot />
</main>

<style lang="postcss">
  a {
    @apply hover:text-gray-800 transition-all text-sm;
  }
</style>
