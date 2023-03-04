<script lang="ts">
  import { getItems, getTable } from '../service/db'
  import { onMount } from 'svelte'
  import { imgURL } from '../service/file'
  import { itemsStore } from 'store'

  let categories: Category[] = []

  async function refresh() {}

  onMount(async () => {
    const allItems = await getItems()
    itemsStore.set(allItems)
    categories = (await getTable('get_categories')) as Category[]
  })
</script>

<nav
  class="flex justify-between items-center w-screen p-12 h-24 text-sm flex-0 font-medium"
>
  <a href="/">my collection ({$itemsStore.length} items)</a>
  <a href="/items/add_item" class="underline">+ add item</a>
</nav>
<div class="flex flex-wrap w-full gap-4 px-12">
  {#each $itemsStore as { name, item_id, img_path }}
    {#await imgURL(img_path) then url}
      <a href={`items/${item_id}`}>
        <div class="flex flex-col">
          <img
            alt="drawing of item"
            src={url}
            class="bg-gray-500 w-28 h-28 object-cover border border-gray-200 rounded-sm"
          />
        </div>
      </a>
    {/await}
  {/each}
</div>

<style lang="postcss">
  button {
    @apply border border-red-500 text-red-500 px-2 py-1 rounded-sm;
  }
  input {
    @apply bg-red-100;
  }
</style>
