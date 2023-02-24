<script>
  import { page } from "$app/stores"

  const id = $page.params.id
  console.log(id)
</script>

<nav class="flex justify-between w-screen p-6 text-sm font-medium shadow-sm">
  <a href="/">my collection</a>
  <div class="flex gap-5">
    <button class="underline">delete item</button>
    <a href={`/items/${id}/edit`} class="underline">+ edit item</a>
  </div>
</nav>
<slot />

<style lang="postcss">
  a {
    @apply hover:text-gray-800 transition-all text-sm;
  }
</style>
