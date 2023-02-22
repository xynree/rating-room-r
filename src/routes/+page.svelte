<script lang="ts">
  import { getItem, getTable } from "../service/db"
  import { onMount } from "svelte"
  import { invoke } from "@tauri-apps/api"
  import { writeBinaryFile, BaseDirectory } from "@tauri-apps/api/fs"

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

  async function saveFile(e: any) {
    const file = document.getElementById("imageInput") as HTMLInputElement
    const saveFile = file.files && file.files[0]
    if (saveFile) {
      let binaryInput = await saveFile.arrayBuffer()
      console.log(binaryInput)
      const write = await writeBinaryFile(
        { path: saveFile.name, contents: binaryInput },
        { dir: BaseDirectory.AppData }
      )
      console.log("written", write)
    }
  }
</script>

<div class="flex flex-col items-start max-w-sm gap-6 m-3">
  <h1 class="font-bold">API Testing Demo</h1>
  <div>
    <input type="file" accept="image/*" id="imageInput" /><button on:click={saveFile}
      >Save File to Tauri :)</button
    >
  </div>
  <input type="number" id="itemId" bind:value={itemId} />
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
