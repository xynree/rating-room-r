<script lang="ts">
  import { getItems, getTable } from '../service/db'
  import Filter from '$lib/Filter.svelte'
  import { onMount } from 'svelte'
  import { imgURL } from '../service/file'
  import { itemsStore, itemView } from 'store'

  let items: FullItem[] = []
  let categories: Category[] = []

  async function refresh() {
    items = (await getTable('get_items')) as FullItem[]
    categories = (await getTable('get_categories')) as Category[]
  }

  onMount(async () => {
    const allItems = await getItems()
    $itemsStore.items = allItems
    const allCategories = (await getTable('get_categories')) as Category[]
    $itemsStore.categories = new Set(allCategories)
    refresh()
  })
</script>

<nav class="flex justify-between w-screen p-6 text-sm font-medium shadow-sm">
  <a href="/">my collection ({$itemsStore.items.length} items)</a>
  <Filter />
  <a href="/items/add_item" class="underline">+ add item</a>
</nav>
<div class="flex flex-col gap-6 m-3 w-full">
  <div class="flex flex-wrap gap-2">
    {#each $itemView as { name, item_id, img_path }}
      {#await imgURL(img_path) then url}
        <a href={`items/${item_id}`}>
          <div class="flex flex-col">
            <img
              alt="drawing of item"
              src={url}
              width={150}
              class="bg-gray-500 object-cover rounded-sm hover:outline hover:outline-1 "
            />
          </div>
        </a>
      {/await}
    {/each}
  </div>
</div>

<style lang="postcss">
  button {
    @apply border border-red-500 text-red-500 px-2 py-1 rounded-sm;
  }
  input {
    @apply bg-red-100;
  }
</style>
