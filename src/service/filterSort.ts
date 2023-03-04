export function filterSortItems(itemsStore: ItemStore): FullItem[] {
	let itemsView: FullItem[] = itemsStore.items;

	if (itemsStore.filters.categories.size) {
		itemsView = itemsStore.items.filter((item) =>
			item.categories.some((c) => itemsStore.filters.categories.has(c.name)),
		);
	}

	if (itemsStore.filters.ratings.size) {
		itemsView = itemsView.filter(({rating}) =>
			itemsStore.filters.ratings.has(rating.rating),
		);
	}

	return itemsView;
}
