export function filterSortItems(itemsStore: ItemStore): FullItem[] {
	let itemsView: FullItem[] = itemsStore.items;

	if (itemsStore.filters.categories.size > 0) {
		itemsView = itemsStore.items.filter((item) =>
			item.categories.some((c) => itemsStore.filters.categories.has(c.name)),
		);
	}

	if (itemsStore.filters.ratings.size > 0) {
		itemsView = itemsView.filter((item) =>
			itemsStore.filters.ratings.has(item.rating.rating),
		);
	}

	return itemsView;
}
