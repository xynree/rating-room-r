<script lang="ts">
  export let categories: Category[] = []
  export let allCategories: Category[]

  import { createEventDispatcher } from "svelte"
  const dispatch = createEventDispatcher()
  function addCategory(e) {
    let category = JSON.parse(e.target.value)

    if (categories.filter((cat) => cat.category_id === category.category_id).length != 0) {
      return
    }

    categories.push(JSON.parse(e.target.value))
    categories = categories
    console.log(categories)
    dispatch("categories", {
      categories: categories,
    })
  }

  function removeCategory(e) {
    let category = JSON.parse(e.target.value)
    categories = categories.filter((cat) => cat.category_id !== category.category_id)
    console.log(categories)
    dispatch("categories", {
      categories: categories,
    })
  }
</script>

<div>
  <p class="font-bold">categories</p>
  <div class="flex">
    {#each categories as category}
      <div class="flex gap-2 badge">
        <button value={JSON.stringify(category)} on:click={removeCategory}>x</button>{category.name}
      </div>
    {/each}
  </div>
  <select class="badge" on:change={addCategory}>
    <option>add category</option>
    {#each allCategories as category}<option value={JSON.stringify(category)}
        >{category.name}</option
      >{/each}
  </select>
</div>

<style lang="postcss">
  .badge {
    @apply rounded-full bg-slate-200 text-xs px-3 py-1 hover:bg-slate-300 transition-all;
  }
</style>
