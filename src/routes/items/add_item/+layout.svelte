<script>
  import { itemsStore } from 'store'
</script>

<nav class="flex justify-between p-6 w-screen text-sm">
  <a href="/">my collection ({$itemsStore.items.length} items)</a>
</nav>

<slot />

<style lang="postcss">
  a {
    @apply hover:text-gray-800 transition-all text-sm;
  }
</style>
