<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import { itemsStore } from 'store'

  export let categories: Category[] = []

  let newCategory: string = ''
  let showCategoryMenu = false

  function addCategory(e) {
    let category = JSON.parse(e.target.value)

    if (
      categories.filter((cat) => cat.category_id === category.category_id)
        .length != 0
    ) {
      showCategoryMenu = false
      return
    }

    categories = [...categories, JSON.parse(e.target.value)]
    showCategoryMenu = false
  }

  async function addNewCategory() {
    const categoryId: number = await invoke('create_category', {
      name: newCategory,
      description: '',
    })

    categories = [
      ...categories,
      {
        category_id: categoryId,
        name: newCategory,
        description: '',
      },
    ]
    showCategoryMenu = false
  }

  function removeCategory(e) {
    let category = JSON.parse(e.target.value)
    categories = categories.filter(
      (cat) => cat.category_id !== category.category_id
    )
  }

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
          {#each Array.from($itemsStore.categories) as category}
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
</div>

<style lang="postcss">
  .badge {
    @apply rounded-full bg-neutral-100 text-xs px-3 py-1 hover:bg-slate-300 transition-all;
  }
</style>
