<script lang="ts">
  import { invoke } from '@tauri-apps/api'
  export let categories: Category[] = []
  let allCategories: Category[] = []

  import { createEventDispatcher, onMount } from 'svelte'
  const dispatch = createEventDispatcher()
  let newCategory: string = ''
  function addCategory(e) {
    let category = JSON.parse(e.target.value)
    console.log(e)

    if (
      categories.filter((cat) => cat.category_id === category.category_id)
        .length != 0
    ) {
      showCategoryMenu = false
      return
    }

    categories.push(JSON.parse(e.target.value))
    categories = categories
    console.log(categories)
    dispatch('categories', {
      categories: categories,
    })
    showCategoryMenu = false
  }

  async function addNewCategory() {
    let cat_id: number = await invoke('create_category', {
      name: newCategory,
      description: '',
    })

    let newcat: Category = {
      category_id: cat_id,
      name: newCategory,
      description: '',
    }

    categories.push(newcat)
    categories = categories
    console.log(categories)
    dispatch('categories', {
      categories: categories,
    })
    showCategoryMenu = false
  }

  function removeCategory(e) {
    let category = JSON.parse(e.target.value)
    categories = categories.filter(
      (cat) => cat.category_id !== category.category_id
    )
    console.log(categories)
    dispatch('categories', {
      categories: categories,
    })
  }

  let showCategoryMenu
  function toggleCategoryMenu() {
    showCategoryMenu = !showCategoryMenu
  }
</script>

<div>
  <p class="font-bold">categories</p>
  <div class="flex">
    {#each categories as category}
      <div class="flex gap-2 badge">
        <button value={JSON.stringify(category)} on:click={removeCategory}
          >x</button
        >{category.name}
      </div>
    {/each}
  </div>
  <select class="badge" on:change={addCategory}>
    <option>add category</option>
    {#each allCategories as category}
      <option value={JSON.stringify(category)}>{category.name}</option>{/each}
  </select>
</div>

<style lang="postcss">
  .badge {
    @apply rounded-full bg-neutral-100 text-xs px-3 py-1 hover:bg-slate-300 transition-all;
  }
</style>
