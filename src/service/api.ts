import { invoke } from '@tauri-apps/api'
import { itemsStore } from 'store'

// Item
export async function getItem(id: number): Promise<Item> {
  return (await invoke('get_item', { id }).catch(() => null)) as Promise<Item>
}

export async function getItems(): Promise<FullItem[]> {
  return await invoke('get_items')
}

export async function deleteItem(id: number) {
  await invoke('delete_item', { id })
}

export async function updateItem({
  item,
  categories,
}: {
  item: FullItem
  categories: Category[] | undefined
}) {
  await invoke('update_item', {
    item,
    categories,
  })
}

export async function addItem(item:FullItem): Promise<number | undefined> {
  if (item.name === '' || !item.categories.length) {
    return
  }
  const newItemId = (await invoke('create_item', {
    name: item.name,
    description: item.description,
    comments: item.comments,
    imgPath: item.img_path || 'default.png',
  })) as number

  await invoke('add_categories_to_item', {
    itemId: newItemId,
    categories: item.categories,
  })

  await invoke('create_rating', {
    rating: item.rating.rating,
    itemId: newItemId,
  })

  itemsStore.refresh()

  return newItemId
}

//Categories

export async function createCategory({
  name,
  description,
}: {
  name: string
  description: string
}): Promise<number> {
  return await invoke('create_category', {
    name,
    description,
  })
}

export async function getCategories(): Promise<Category[] | []> {
  return invoke('get_categories')
    .then((c) => c as Category[])
    .catch((error) => [])
}

//Ratings
export async function createRating({
  rating,
  itemId,
}: {
  rating: number
  itemId: number
}) {
  await invoke('create_rating', {
    rating,
    itemId,
  })
}
