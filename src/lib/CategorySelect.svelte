<script lang="ts">
  import { createCategory } from 'service/api'
  import { itemsStore } from 'store'

  export let categories: Category[] = []

  let categoryName: string = ''
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
    const categoryId = await createCategory({
      name: categoryName,
      description: '',
    })

    let newCategory = {
      category_id: categoryId,
      name: categoryName,
      description: '',
    }

    categories = [...categories, newCategory]
    $itemsStore.categories.add(newCategory)
    showCategoryMenu = false
    categoryName = ''
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
  <div class="flex flex-wrap max-w-xs">
    {#each categories as category}
      <div class="flex gap-2 badge">
        <button value={JSON.stringify(category)} on:click={removeCategory}
          >x</button
        >{category.name}
      </div>
    {/each}
    <button
      on:click={toggleCategoryMenu}
      id="addCategory"
      class="badge {categories.length ? '' : 'ring-2 ring-red-500'}"
    >
      + add category
    </button>
  </div>
  {#if showCategoryMenu}
    <div class="flex absolute flex-col bg-white rounded-xl border border-black">
      <div class="overflow-y-auto py-2 px-4 max-h-40">
        <ul>
          {#each Array.from($itemsStore.categories) as category}
            <li
              class="flex flex-col hover:bg-zinc-300"
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
          bind:value={categoryName}
          placeholder="new category"
        />
      </form>
    </div>
  {/if}
</div>

<style lang="postcss">
  .badge {
    @apply m-0.5 w-fit rounded-full bg-zinc-200 text-xs px-3 py-1 hover:bg-zinc-300 transition-all;
  }
</style>
