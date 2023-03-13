<script lang="ts">
  import { goto } from '$app/navigation'
  import CategorySelect from '$lib/CategorySelect.svelte'
  import RatingSelect from '$lib/RatingSelect.svelte'
  import { addItem, getTable } from 'service/db'
  import { itemsStore } from 'store'
  import { onMount } from 'svelte'

  let showError: boolean = false
  function toggleError() {
    showError = true
    setTimeout(() => (showError = false), 2000)
  }

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

  function formErrors() {
    return cells.every((c) => c.name === '' || !c.categories.length)
  }

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
    cells.forEach(async (c) => await createItem(c))
    goto('/')
  }

  async function createItem(item) {
    item = { ...item, img_path: '' }

    await addItem(item)
  }
</script>

<div
  class="w-screen flex-col gap-8 items-center justify-center m-auto text-sm text-center"
>
  <div class="my-2 text-xl font-bold">Bulk Add Items</div>
  <p
    class={`py-2 transition-all ${
      showError ? 'opacity-100' : 'opacity-0'
    } text-red-700 font-light text-xs`}
  >
    Missing name or categories
  </p>

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
              autocomplete="off"
              class:ring-2={!name}
              class:ring-red-500={!name}
              on:change={(e) => updateItem(e, i)}
            /></td
          >
          <td>
            <input
              value={description}
              placeholder="description"
              id="description"
              on:change={(e) => updateItem(e, i)}
            />
          </td>
          <td>
            <RatingSelect bind:defaultRating={rating} />
          </td>
          <td>
            <CategorySelect bind:categories />
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
    <button
      on:click={() => {
        formErrors() ? toggleError() : submitItems()
      }}>Submit Items</button
    >
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
