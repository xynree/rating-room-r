import { derived, writable } from 'svelte/store'

let initialStore: ItemStore = {
  items: [],
  categories: new Set(),
  filters: {
    categories: new Set(),
    ratings: new Set(),
  },
}

export enum FilterType {
  categories,
  ratings,
}

function createInitialStore() {
  const { subscribe, set, update } = writable(initialStore)

  // filter reset and toggle functions
  function toggle(filterType: FilterType, value: string | number) {
    update((store) => {
      let filterGroup = filterType
        ? store.filters.ratings
        : store.filters.categories

      filterGroup.has(value)
        ? filterGroup.delete(value)
        : filterGroup.add(value)

      return store
    })
  }

  function reset() {
    update((store) => ({
      ...store,
      filters: {
        categories: new Set(),
        ratings: new Set(),
      },
    }))
  }

  return {
    subscribe,
    set,
    update,
    filters: {
      reset,
      toggle,
    },
  }
}

function filterItemsStore($itemsStore: ItemStore) {
  let itemsView: FullItem[] = $itemsStore.items

  if ($itemsStore.filters.categories.size) {
    itemsView = $itemsStore.items.filter((item) =>
      item.categories.some((c) => $itemsStore.filters.categories.has(c.name))
    )
  }

  if ($itemsStore.filters.ratings.size) {
    itemsView = itemsView.filter(({ rating }) =>
      $itemsStore.filters.ratings.has(rating.rating)
    )
  }

  return itemsView
}

export const itemsStore = createInitialStore()
export const itemView = derived(itemsStore, filterItemsStore)
