import { filterSortItems } from 'service/filterSort'
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
	function toggle(filterType: FilterType, value:string|number){
		update((store) => {
			let filterGroup = filterType
				? store.filters.ratings
				: store.filters.categories;

			filterGroup.has(value)? filterGroup.delete(value): filterGroup.add(value)

			return store
		})
	}

	function reset(){
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

export const itemsStore = createInitialStore()
export const itemView = derived(itemsStore, ($itemsStore) =>
  filterSortItems($itemsStore)
)
