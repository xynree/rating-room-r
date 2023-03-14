<script lang="ts">
  import { goto } from '$app/navigation'
  import CategorySelect from '$lib/CategorySelect.svelte'
  import RatingSelect from '$lib/RatingSelect.svelte'
  import { addItem, getCategories } from 'service/api'
  import { itemsStore } from 'store'
  import { onMount } from 'svelte'

  let showError: boolean = false
  function toggleError() {
    showError = true
    setTimeout(() => (showError = false), 2000)
  }

  const defaultCell: FullItem = {
    name: '',
    description: '',
    categories: [] as Category[],
    comments: '',
    item_id: 0,
    img_path: '',
    date: '',
    rating: {
      date: '',
      ratingId: 0,
      rating: 0,
    },
  }

  onMount(async () => {
    $itemsStore.categories = new Set(await getCategories())
  })

  $: cells = [
    {
      ...defaultCell,
    },
  ]

  function formErrors() {
    return cells.every((c) => c.name === '' || !c.categories.length)
  }

  function updateItem(e: Event, i: number) {
    const target = e.target as HTMLInputElement
    const val = target.id as keyof typeof defaultCell
    if (val) {
      cells[i][val] = target.value as never
    }
  }

  function submitItems() {
    cells.forEach(async (c) => await createItem(c))
    goto('/')
  }

  async function createItem(item: FullItem) {
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
            <RatingSelect bind:defaultRating={rating.rating} />
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
              on:click={() => (cells = cells.filter((_, idx) => idx !== i))}
              >delete</button
            ></td
          >
        </tr>
      {/each}
    </tbody>
  </table>
  <div class="my-6">
    <button
      on:click={() =>
        (cells = [
          ...cells,
          { ...defaultCell, rating: { ...defaultCell.rating } },
        ])}
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
