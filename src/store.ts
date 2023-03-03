import { filterSortItems } from "service/filterSort";
import { derived, writable } from "svelte/store";

let initialStore: ItemStore = {
	items: [],
	categories: new Set(),
	filters: {
		categories: new Set(),
		ratings: new Set(),
	},
};

export const itemsStore = writable(initialStore);
export const itemView = derived(itemsStore, ($itemsStore) =>
	filterSortItems($itemsStore),
);
