<script lang="ts">
  import { getItem, getTable } from '../service/db'
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api'
  import { imgURL } from '../service/file'

  let items: Item[] = []
  let categoryName: string
  let categories: Category[] = []
  let itemId: number = 0
  let item: Item | null = null

  async function refresh() {
    items = (await getTable('get_items')) as Item[]
    categories = (await getTable('get_categories')) as Category[]
  }

  async function updateItem() {
    item = await getItem(itemId)
  }

  async function deleteCategory(id: number) {
    await invoke('delete_category', { id })
    refresh()
  }

  async function createCategory(e: any) {
    e.preventDefault()
    if (categoryName == '') return
    await invoke('create_category', {
      name: categoryName,
      description: 'test description',
    })
    categoryName = ''
    refresh()
  }

  onMount(async () => {
    refresh()
  })
</script>

<nav class="flex justify-between w-screen p-6 text-sm font-medium shadow-sm">
  <a href="/">my collection</a>
  <a href="/items/add_item" class="underline">+ add item</a>
</nav>
<div class="flex flex-col items-start gap-6 m-3">
  <div class="flex flex-wrap w-full gap-2">
    {#each items as { name, item_id, img_path }}
      {#await imgURL(img_path) then url}
        <a href={`items/${item_id}`}>
          <div class="flex flex-col">
            <img
              alt="drawing of item"
              src={url}
              class="bg-gray-500 w-24 h-24 object-cover border border-gray-200 rounded-sm"
            />
            <p class="w-24 text-xs">{name}</p>
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
