type Item = {
  item_id: number
  name: string
  description: string
  comments: string
  img_path: string
  date: string
}

type FullItem = {
  item_id: number
  name: string
  description: string
  comments: string
  img_path: string
  date: string
  categories: Category[]
  rating: Rating
  traits: Trait[]
}

type Category = {
  category_id: number
  name: string
  description: string
}

type Rating = {
  date: string | number | Date
  ratingId: number
  rating: number
}

type Trait = {
  trait_id: number
  name: string
  range_low: string
  range_high: string
}
