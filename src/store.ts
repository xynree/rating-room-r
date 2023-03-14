import { invoke } from '@tauri-apps/api'
import { derived, writable } from 'svelte/store'

export enum SortBy {
  date = 'date',
  name = 'name',
  rating = 'rating',
}

export enum SortDir {
  ascending,
  descending,
}

let initialStore: ItemStore = {
  items: [],
  categories: new Set(),
  filters: {
    categories: new Set(),
    ratings: new Set(),
  },
  sort: {
    by: SortBy.rating,
    dir: SortDir.ascending,
  },
}

export enum FilterType {
  categories,
  ratings,
}

function createInitialStore() {
  const { subscribe, set, update } = writable(initialStore)

  // refresh db

  async function refresh(){
      await invoke("get_items").then((items)=>{
        update((store)=>({
          ...store, items:items as FullItem[]
        }))
      })
  }

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
    refresh,
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

function sortItemsStore($itemsStore: ItemStore, itemsView: FullItem[]) {
  let sortDir = $itemsStore.sort.dir
  let sortBy = $itemsStore.sort.by

  switch (sortBy) {
    case SortBy.date:
      if (sortDir == SortDir.ascending) {
        itemsView = itemsView.sort(
          (a, b) => Date.parse(a.date) - Date.parse(b.date)
        )
      } else {
        itemsView = itemsView.sort(
          (a, b) => Date.parse(b.date) - Date.parse(a.date)
        )
      }
      break
    case SortBy.name:
      if (sortDir == SortDir.ascending) {
        itemsView = itemsView.sort((a, b) => (a.name < b.name ? -1 : 1))
      } else {
        itemsView = itemsView.sort((a, b) => (a.name > b.name ? -1 : 1))
      }
      break
    case SortBy.rating:
      if (sortDir == SortDir.ascending) {
        itemsView = itemsView.sort((a, b) =>
          a.rating.rating < b.rating.rating ? -1 : 1
        )
      } else {
        itemsView = itemsView.sort((a, b) =>
          a.rating.rating > b.rating.rating ? -1 : 1
        )
      }
      break
  }

  return itemsView
}

function refreshView($itemsStore: ItemStore) {
  let itemsView = filterItemsStore($itemsStore)
  itemsView = sortItemsStore($itemsStore, itemsView)
  return itemsView
}

export const itemsStore = createInitialStore()
export const itemView = derived(itemsStore, refreshView)
