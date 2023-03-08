<script lang="ts">
  import { getItems, getTable } from '../service/db'
  import Filter from '$lib/Filter.svelte'
  import { onMount } from 'svelte'
  import { imgURL } from '../service/file'
  import { itemsStore, itemView } from 'store'
  import Sort from '$lib/Sort.svelte'
  import { isRegistered, register } from '@tauri-apps/api/globalShortcut'
  import { goto } from '$app/navigation'

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

<nav class="flex justify-between w-screen p-6 text-sm">
  <a href="/">my collection ({$itemsStore.items.length} items)</a>
  <Sort />
  <Filter />
  <a href="/items/add_item" class="underline">+ add item</a>
</nav>
<div class="grid grid-cols-[repeat(auto-fit,_minmax(150px,_1fr))] gap-1 m-8">
  {#if $itemsStore.items.length}
    {#each $itemView as { item_id, img_path }}
      {#await imgURL(img_path) then url}
        <a href={`items/${item_id}`}>
          <img
            alt="drawing of item"
            src={url}
            class="object-cover w-full rounded-sm hover:outline hover:outline-1"
          />
        </a>
      {/await}
    {/each}
  {:else}
    <div class="m-auto mt-20 text-center">
      <h1 class="font-bold text-lg">No Items Found</h1>
      <p class="pt-2">
        <a href="/items/add_item" class="underline">+ add item</a>
        to start your collection
      </p>
    </div>
  {/if}
</div>

<style lang="postcss">
  button {
    @apply border border-red-500 text-red-500 px-2 py-1 rounded-sm;
  }
  input {
    @apply bg-red-100;
  }

  a {
    @apply text-zinc-800 hover:text-black;
  }
</style>
