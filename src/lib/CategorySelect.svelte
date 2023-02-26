<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'

  export let categories: Category[] = []
  export let allCategories: Category[]

  import { createEventDispatcher } from 'svelte'
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
  <button on:click={toggleCategoryMenu} id="addCategory" class="badge">
    + add category
  </button>
  {#if showCategoryMenu}
    <div class="flex absolute flex-col bg-white rounded-xl border border-black">
      <div class="overflow-y-auto py-2 px-4 max-h-24">
        <ul>
          {#each allCategories as category}
            <li
              class="flex flex-col hover:bg-neutral-300"
              value={JSON.stringify(category)}
            >
              <button on:click={addCategory} value={JSON.stringify(category)}>
                {category.name}
              </button>
            </li>{/each}
        </ul>
      </div>
      <form
        on:submit|preventDefault={addNewCategory}
        class="p-2 border-t border-t-black"
      >
        <input
          class="font-bold text-center"
          bind:value={newCategory}
          placeholder="new category"
        />
      </form>
    </div>
  {/if}
  <!-- <select class="badge" on:change={addCategory}> -->
  <!--   <option>add category</option> -->
  <!-- </select> -->
</div>

<style lang="postcss">
  .badge {
    @apply rounded-full bg-neutral-100 text-xs px-3 py-1 hover:bg-slate-300 transition-all;
  }
</style>
