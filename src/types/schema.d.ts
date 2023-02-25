type Item = {
  item_id: number
  name: string
  description: string
  comments: string
  img_path: string
  date: string
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
