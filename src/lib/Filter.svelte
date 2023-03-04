<script lang="ts">
  import { itemsStore, itemView } from 'store'
  let showCategoryFilter = false
  let showRatingFilter = false

  $: console.log({$itemsStore, $itemView})

  function toggleRatingFilter() {
    showRatingFilter = !showRatingFilter
  }

  function toggleCategoryFilter() {
    showCategoryFilter = !showCategoryFilter
  }

  function handleRating(e) {
    let rating = Number(e.target.value)
    if ($itemsStore.filters.ratings.has(rating)) {
      $itemsStore.filters.ratings.delete(rating)
      $itemsStore = $itemsStore
    } else {
      if (rating > 0 && rating <= 5) {
        $itemsStore.filters.ratings.add(rating)
        $itemsStore = $itemsStore
      }
    }
  }

  function handleCategory(e) {
    let category = e.target.value
    if ($itemsStore.filters.categories.has(category)) {
      $itemsStore.filters.categories.delete(category)
      $itemsStore = $itemsStore
    } else {
      $itemsStore.filters.categories.add(category)
      $itemsStore = $itemsStore
    }

  }
</script>

<div class="flex flex-row gap-8">
  <button on:click={toggleRatingFilter}> rating </button>
  {#if showRatingFilter}
    <div
      on:click={toggleRatingFilter}
      class="absolute w-screen h-screen opacity-0"
    />
    <div
      class="flex fixed top-12 flex-col bg-white rounded-xl border border-black"
    >
      <div class="overflow-y-auto py-2 px-4 max-h-36 w-fit">
        <ul>
          {#each Array(5) as _, i}
            <li class="flex flex-row gap-2 justify-between space-x-4">
              <button on:click={handleRating} value={i + 1}>
                {#each Array(i + 1) as _}★{/each}
              </button>
              <span>
                {$itemsStore.filters.ratings.has(i + 1) ? '✓' : ''}
              </span>
            </li>{/each}
        </ul>
      </div>
    </div>
  {/if}

  <button on:click={toggleCategoryFilter}> category </button>
  {#if showCategoryFilter}
    <div
      on:click={toggleCategoryFilter}
      class="absolute w-screen h-screen opacity-0"
    />
    <div
      class="flex fixed top-12 flex-col bg-white rounded-xl border border-black"
    >
      <div class="overflow-y-auto py-2 px-4 max-h-36 w-fit">
        <ul>
          {#each Array.from($itemsStore.categories) as category}
            <li class="flex flex-row gap-2 justify-between space-x-4">
              <button on:click={handleCategory} value={category.name}>
                {category.name}
              </button>
              <span>
                {$itemsStore.filters.categories.has(category.name) ? '✓' : ''}
              </span>
            </li>{/each}
        </ul>
      </div>
    </div>
  {/if}
</div>
