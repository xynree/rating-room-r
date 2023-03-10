type ItemStore = {
  items: FullItem[]
  categories: Set<Category>
  filters: {
    categories: Set<string>
    ratings: Set<number>
  }
  sort: {
    by: SortBy
    dir: SortDir
  }
}
