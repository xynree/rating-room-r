<script lang="ts">
  import { getItem, getTable } from "../api/db"
  import { onMount } from "svelte"

  let items: Item[] = []
  let categories: Category[] = []
  let item_id: number = 0
  let item: Item | null = null

  onMount(async () => {
    items = await getTable("get_items")
    categories = await getTable("get_categories")
    console.log(items, categories)
  })

  async function updateItem() {
    item = await getItem(item_id)
  }
</script>

<div class="m-3 flex flex-col max-w-sm items-start">
  <h1 class="font-bold">API Testing Demo</h1>
  <input type="number" id="item_id" bind:value={item_id} />
  <button class="p-1 font-bold" on:click={updateItem}>Get Item</button>
  {#if item}
    <p>{item.name} - {item.description} - {item.date}</p>
  {/if}
  {#if items.length == 0}
    <p>No Items</p>
  {:else}
    <h2 class="text-xl font-bold">Items</h2>
    {#each items as item}
      <p>{item.name} - {item.description} - {item.date}</p>
    {/each}
  {/if} -->
  <div class="font-bold">Categories</div>
  {#if categories}
    {#each categories as { id, name, description }}
      <p>{name} - {id} - {description}</p>
    {/each}
  {/if}
</div>
