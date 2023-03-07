<script lang="ts">
  import { FilterType, itemsStore } from 'store'
  let showCategoryFilter = false
  let showRatingFilter = false

  $: hasFilters =
    $itemsStore.filters.categories.size > 0 ||
    $itemsStore.filters.ratings.size > 0

  function toggleRatingFilter() {
    if (showCategoryFilter) showCategoryFilter = false
    showRatingFilter = !showRatingFilter
  }

  function toggleCategoryFilter() {
    if (showRatingFilter) showRatingFilter = false
    showCategoryFilter = !showCategoryFilter
  }
</script>

<div class="flex flex-row gap-1 ml-auto mr-8">
  {#if hasFilters}
    <button
      class="font-light bg-zinc-100 rounded-full px-4"
      on:click={itemsStore.filters.reset}
    >
      clear filters
    </button>
  {/if}
  <button
    on:click={toggleRatingFilter}
    class="{showRatingFilter
      ? 'bg-zinc-100'
      : ''} rounded-full px-4 relative flex gap-1 items-center"
  >
    rating <span class="text-[9px] mt-[0.5px]"
      >{showRatingFilter ? '▲' : '▼'}</span
    >
  </button>
  {#if showRatingFilter}
    <div class="modal right-48">
      <div class="overflow-y-auto py-2 px-4 max-h-36 w-fit">
        <ul>
          {#each Array(5) as _, i}
            <li>
              <button
                on:click={() =>
                  itemsStore.filters.toggle(FilterType.ratings, i + 1)}
                value={i + 1}
              >
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

  <button
    on:click={toggleCategoryFilter}
    class="{showCategoryFilter
      ? 'bg-zinc-100'
      : ''} rounded-full px-4 relative flex items-center gap-1"
  >
    category <span class="text-[9px] mt-[0.5px]"
      >{showCategoryFilter ? '▲' : '▼'}</span
    >
  </button>
  {#if showCategoryFilter}
    <div class="modal right-24">
      <div class="py-2 px-4 w-fit">
        <ul>
          {#each Array.from($itemsStore.categories) as category}
            <li>
              <button
                on:click={() =>
                  itemsStore.filters.toggle(
                    FilterType.categories,
                    category.name
                  )}
                value={category.name}
              >
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

<style lang="postcss">
  .modal {
    @apply flex absolute max-h-72 top-12 flex-col bg-white rounded-xl border border-black;
  }

  li {
    @apply flex flex-row gap-2 justify-between space-x-4 text-zinc-800 hover:text-black;
  }
</style>
