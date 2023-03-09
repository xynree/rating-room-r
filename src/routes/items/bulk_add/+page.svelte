<script lang="ts">
  import CategorySelect from '$lib/CategorySelect.svelte'
  import { invoke } from '@tauri-apps/api'
  import { getTable } from 'service/db'
  import { itemsStore } from 'store'
  import { onMount } from 'svelte'

  const defaultCell = {
    name: '',
    description: '',
    rating: 1,
    categories: ['Uncategorized'],
    comments: '',
  }

  onMount(async () => {
    const allCategories = (await getTable('get_categories')) as Category[]
    $itemsStore.categories = new Set(allCategories)
  })

  $: toggleCategoryMenu = false
  $: cells = Array(3).fill(defaultCell)
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
          <td><input value={name} placeholder="item name" /></td>
          <td><input value={description} placeholder="description" /></td>
          <td><input value={rating} placeholder="rating" /></td>
          <td>
            <!-- {#each categories as myCategory}
              <div class="flex gap-2 badge">
                <button value={JSON.stringify(myCategory)}>x</button
                >{myCategory.name}
              </div>
            {/each} -->
            <button
              on:click={() => (toggleCategoryMenu = !toggleCategoryMenu)}
              id="addCategory"
              class="badge"
            >
              + add category
            </button>
            {#if toggleCategoryMenu}
              <div
                class="flex absolute flex-col bg-white rounded-xl border border-black"
              >
                <div class="overflow-y-auto py-2 px-4 max-h-40">
                  <ul>
                    {#each [...$itemsStore.categories] as category}
                      <li value={JSON.stringify(category)}>
                        {category.name}
                      </li>{/each}
                  </ul>
                </div>
                <form class="p-2 border-t border-t-black">
                  <input
                    class="font-bold text-center"
                    placeholder="new category"
                  />
                </form>
              </div>
            {/if}
          </td>
          <td><input value={comments} placeholder="comments" /></td>
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
      on:click={() => (cells = [...cells, defaultCell])}
      class="hover:underline"
    >
      +Add a New Item
    </button>
    <button>Submit Items</button>
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
    @apply border p-1 rounded-sm mx-1;
  }

  button {
    @apply p-1 px-4 text-xs  text-gray-800 transition-all  hover:text-black bg-gray-100 rounded-full;
  }
</style>
