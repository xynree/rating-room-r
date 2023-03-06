<script lang="ts">
  import { SortDir, SortBy, itemsStore } from 'store'
  let showSortSelect = false

  function toggleSortSelect() {
    showSortSelect = !showSortSelect
  }

  function changeSort(e: Event) {
    const target = e.target as HTMLButtonElement
    $itemsStore.sort.by = target.value as SortBy
  }

  function changeDir() {
    if ($itemsStore.sort.dir == SortDir.ascending) {
      $itemsStore.sort.dir = SortDir.descending
    } else {
      $itemsStore.sort.dir = SortDir.ascending
    }
  }
</script>

<div class="flex flex-row gap-1 ml-4">
  <div class=" {showSortSelect ? 'bg-zinc-100' : ''} rounded-full px-4 ">
    <button on:click={toggleSortSelect} class="relative pr-1">
      {$itemsStore.sort.by}
    </button>
    <button on:click={changeDir}>
      {$itemsStore.sort.dir == SortDir.ascending ? '⏶' : '⏷'}
    </button>
  </div>
  {#if showSortSelect}
    <div class="modal left-30">
      <div class="overflow-y-auto py-2 px-4 max-h-36 w-fit">
        <ul>
          {#each Object.keys(SortBy) as sort}
            <li>
              <button on:click={changeSort} value={sort}>
                {sort}
              </button>
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
    @apply flex flex-row gap-2 justify-between space-x-4 text-gray-700 hover:text-black;
  }
</style>
