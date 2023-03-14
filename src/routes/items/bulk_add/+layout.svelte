<script>
  import { itemsStore } from 'store'
</script>

<nav class="flex justify-between p-6 w-screen text-sm">
  <a href="/">my collection ({$itemsStore.items.length} items)</a>
  <a href="/items/add_item">single add</a>
</nav>

<slot />

<style lang="postcss">
  a {
    @apply hover:text-gray-800 transition-all text-sm hover:underline;
  }
</style>
