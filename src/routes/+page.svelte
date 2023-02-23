<script lang="ts">
  import { getItem, getTable } from "../service/db"
  import { onMount } from "svelte"
  import { invoke } from "@tauri-apps/api"

  let items: Item[] = []
  let categoryName: string
  let categories: Category[] = []
  let itemId: number = 0
  let item: Item | null = null

  async function refresh() {
    items = (await getTable("get_items")) as Item[]
    categories = (await getTable("get_categories")) as Category[]
  }

  async function updateItem() {
    item = await getItem(itemId)
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

<div class="flex flex-col items-start gap-6 m-3">
  <div class="flex flex-wrap w-full gap-4">
    {#each items as { name, item_id }}
      <a
        href={`/items/${item_id}`}
        class="w-24 h-24 p-2 text-xs text-center hover:bg-gray-200 outline">{name}</a
      >
    {/each}
  </div>
  <!-- {#if categories}
    {#each categories as { category_id, name, description }}
      <div class="flex gap-4">
        <p>{name} - {category_id} - {description}</p>
        <button on:click={() => deleteCategory(category_id)}>Delete</button>
      </div>
    {/each}
    <form on:submit={createCategory}>
      <p>Create Category</p>
      <input bind:value={categoryName} />
    </form>
  {/if} -->
</div>

<style lang="postcss">
  button {
    @apply border border-red-500 text-red-500 px-2 py-1 rounded-sm;
  }
  input {
    @apply bg-red-100;
  }
</style>
