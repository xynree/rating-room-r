<script lang="ts">
  import { goto } from '$app/navigation'
  import CategorySelect from '$lib/CategorySelect.svelte'
  import { invoke } from '@tauri-apps/api'
  import { getTable } from 'service/db'
  import { itemsStore } from 'store'
  import { onMount } from 'svelte'

  const defaultCell = {
    name: '',
    description: '',
    rating: 1,
    categories: [] as Category[],
    comments: '',
  }

  onMount(async () => {
    const allCategories = (await getTable('get_categories')) as Category[]
    $itemsStore.categories = new Set(allCategories)
  })

  $: cells = [
    {
      ...defaultCell,
    },
  ]

  function toggleMenu(i: number) {
    const menu = document.getElementById(`menu_${i}`)
    if (menu) {
      menu.classList.contains('invisible')
        ? menu.classList.remove('invisible')
        : menu.classList.add('invisible')
    }
  }

  function updateItem(e, i: number) {
    const val = e.target?.id as keyof typeof defaultCell
    if (val) {
      cells[i][val] = e.target.value
    }
  }

  function addCategory(i: number, category: Category) {
    cells[i].categories = [...cells[i].categories, category]
    document.getElementById(`menu_${i}`)?.classList.add('invisible')
  }

  function removeCategory(i: number, category: Category) {
    cells[i].categories = cells[i].categories.filter(
      (c) => c.category_id !== category.category_id
    )
  }

  function submitItems() {
    const res = cells.forEach(async (c) => await createItem(c))
    goto('/')
  }

  async function createItem(item) {
    item = { ...item, img_path: '' }

    if (item.name === '' || !item.categories.length) {
      return
    }
    const newItemId = await invoke('create_item', {
      name: item.name,
      description: item.description,
      comments: item.comments,
      imgPath: 'default.png',
    })
    await invoke('add_categories_to_item', {
      itemId: newItemId,
      categories: item.categories,
    })
    await invoke('create_rating', {
      rating: Number(item.rating),
      itemId: newItemId,
    })

    // itemsStore.fetch()
    await invoke('get_items').then(
      (items) => ($itemsStore.items = items as FullItem[])
    )
  }
</script>

<div
  class="w-screen flex-col gap-8 items-center justify-center m-auto text-sm text-center"
>
  <div class="my-2 text-xl font-bold">Bulk Add Items</div>
  <table>
    <thead>
      <tr>
        <td>Item Name</td>
        <td>Description</td>
        <td>Rating (1-5)</td>
        <td>Categories</td>
        <td>Comments</td>
        <td />
      </tr>
    </thead>
    <tbody>
      {#each cells as { name, description, rating, categories, comments }, i}
        <tr>
          <td
            ><input
              value={name}
              placeholder="item name"
              id="name"
              on:change={(e) => updateItem(e, i)}
            /></td
          >
          <td
            ><input
              value={description}
              placeholder="description"
              id="description"
              on:change={(e) => updateItem(e, i)}
            /></td
          >
          <td
            ><input
              type="number"
              value={rating}
              max="5"
              min="1"
              placeholder="rating"
              id="rating"
              on:change={(e) => updateItem(e, i)}
            /></td
          >
          <td>
            <div class="flex flex-wrap">
              {#each categories as category}
                <div class="flex gap-1 px-3 bg-gray-100 rounded-full text-xs">
                  {category.name}
                  <span
                    class="cursor-pointer"
                    on:click={() => removeCategory(i, category)}>x</span
                  >
                </div>
              {/each}
              <div
                on:click={() => toggleMenu(i)}
                class="px-3 text-xs bg-gray-200 rounded-full cursor-pointer hover:bg-gray-300"
              >
                + add category
              </div>
            </div>

            <div
              class="flex absolute flex-col bg-white rounded-xl border border-black invisible"
              id={`menu_${i}`}
            >
              <div class="overflow-y-auto py-2 px-4 max-h-40">
                <ul>
                  {#each [...$itemsStore.categories] as category}
                    <div
                      class="cursor-pointer"
                      on:click={() => addCategory(i, category)}
                    >
                      {category.name}
                    </div>{/each}
                </ul>
              </div>
              <form class="p-2 border-t border-t-black">
                <input
                  class="font-bold text-center"
                  placeholder="new category"
                />
              </form>
            </div>
          </td>
          <td
            ><input
              value={comments}
              placeholder="comments"
              id="comments"
              on:change={(e) => updateItem(e, i)}
            /></td
          >
          <td
            ><button
              on:click={() => (cells = cells.filter((c, idx) => idx !== i))}
              >delete</button
            ></td
          >
        </tr>
      {/each}
    </tbody>
  </table>
  <div class="my-6">
    <button
      on:click={() => (cells = [...cells, { ...defaultCell }])}
      class="hover:underline"
    >
      +Add a New Item
    </button>
    <button on:click={submitItems}>Submit Items</button>
  </div>
</div>

<style lang="postcss">
  table {
    table-layout: fixed;
    width: 1000px;
    @apply outline outline-1 outline-black m-auto;
  }

  thead > * > td {
    @apply bg-gray-100;
  }

  tbody > * > td {
    @apply overflow-ellipsis whitespace-nowrap py-1;
  }

  input {
    @apply border p-1 rounded-sm mx-1 text-xs;
  }

  button {
    @apply p-1 px-4 text-xs  text-gray-800 transition-all  hover:text-black bg-gray-100 rounded-full;
  }
</style>
