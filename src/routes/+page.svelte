<script lang="ts">
  import { getItem, getTable } from "../api/db"
  import { onMount } from "svelte"
  import { invoke } from "@tauri-apps/api"

  let items: Item[] = []
  let categoryName: string
  let categories: Category[] = []
  let item_id: number = 0
  let item: Item | null = null

  async function refresh() {
    items = (await getTable("get_items")) as Item[]
    categories = (await getTable("get_categories")) as Category[]
  }

  async function updateItem() {
    item = await getItem(item_id)
  }

  async function deleteCategory(id: number) {
    await invoke("delete_category", { id })
    refresh()
  }

  async function createCategory(e: any) {
    e.preventDefault()
    if (categoryName == "") return
    await invoke("create_category", {
      name: categoryName,
      description: "test description",
    })
    categoryName = ""
    refresh()
  }

  onMount(async () => {
    refresh()
  })
</script>

<div class="m-3 flex flex-col max-w-sm items-start">
  <h1 class="font-bold">API Testing Demo</h1>
  <input type="number" id="item_id" bind:value={item_id} />
  <button on:click={updateItem}>Get Item</button>
  {#if item}
    <p>{item.name} - {item.description} - {item.date}</p>
  {/if}
  {#if items.length == 0}
    <p>No Items</p>
  {:else}
    <h2 class="text-xl font-bold">Items</h2>
    {#each items as { name, description, date, comments }}
      <p>{name} - {description} - {date} - {comments}</p>
    {/each}
  {/if}
  <div class="font-bold">Categories</div>
  {#if categories}
    {#each categories as { id, name, description }}
      <div class="flex gap-4">
        <p>{name} - {id} - {description}</p>
        <button on:click={() => deleteCategory(id)}>Delete</button>
      </div>
    {/each}
    <form on:submit={createCategory}>
      <p>Create Category</p>
      <input bind:value={categoryName} />
    </form>
  {/if}
</div>

<style lang="postcss">
  button {
    @apply border border-red-500 text-red-500 px-2 py-1 rounded-sm;
  }
  input {
    @apply bg-red-100;
  }
</style>
