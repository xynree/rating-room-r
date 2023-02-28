<script lang="ts">
  import { onDestroy, onMount } from 'svelte'
  import { goto } from '$app/navigation'
  import { page } from '$app/stores'
  import { invoke } from '@tauri-apps/api'
  import { getItem } from 'service/db'
  import { deleteImgFromPath, saveFile } from 'service/file'
  import ItemForm from '$lib/ItemForm.svelte'

  $: id = Number($page.params.id)
  let imgUrl: string = ''
  let item: Item
  let ratings: Rating[]
  let categories: Category[] = []
  let editState: EditState

  onMount(async () => {
    item = await getItem(id)
    ratings = await invoke('get_ratings', { itemId: id })
    categories = await invoke('get_categories_for_item', { id })
    editState = {
      item,
      rating: ratings[0].rating,
      categories,
    }
  })

  async function saveItem(e: { detail: EditState }) {
    let editedItem = e.detail
    window.URL.revokeObjectURL(imgUrl)
    const file = document.getElementById('imageInput') as HTMLInputElement
    if (file && file.files && file.files[0]) {
      const img_path = await saveFile(file.files[0])
      editedItem = { ...editedItem, item: { ...editedItem.item, img_path } }
      if (item.img_path) {
        deleteImgFromPath(item.img_path)
      }
    }
    await invoke('update_item', {
      item: editedItem.item,
      categories: editState.categories,
    })
    await invoke('create_rating', { rating: editedItem.rating, itemId: id })
    goto(`/items/${editedItem.item.item_id}`)
  }
</script>

{#if item}
  <ItemForm
    {editState}
    defaultRating={ratings && ratings[0].rating}
    on:sendItem={saveItem}
  />
{/if}
