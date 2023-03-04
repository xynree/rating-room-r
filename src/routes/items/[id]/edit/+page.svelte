<script lang="ts">
  import { onDestroy, onMount } from 'svelte'
  import { goto } from '$app/navigation'
  import { page } from '$app/stores'
  import { itemView } from 'store'
  import { invoke } from '@tauri-apps/api'
  import { getItem } from 'service/db'
  import { deleteImgFromPath, saveFile } from 'service/file'
  import ItemForm from '$lib/ItemForm.svelte'

  $: id = Number($page.params.id)
  let imgUrl: string = ''
  $: item = $itemView.find((i) => i.item_id == id)
  $: editState = item

  async function saveItem(e: { detail: FullItem }) {
    let editedItem = e.detail
    window.URL.revokeObjectURL(imgUrl)
    const file = document.getElementById('imageInput') as HTMLInputElement
    if (file && file.files && file.files[0]) {
      const img_path = await saveFile(file.files[0])
      editedItem = { ...editedItem, img_path }
      if (item?.img_path) {
        deleteImgFromPath(item.img_path)
      }
    }
    await invoke('update_item', {
      item: editedItem,
      categories: editState?.categories,
    })
    await invoke('create_rating', {
      rating: editedItem.rating.rating,
      itemId: id,
    })
    goto(`/items/${editedItem.item_id}`)
  }
</script>

{#if item}
  <ItemForm
    {editState}
    on:cancel={() => goto(`/items/${id}`)}
    on:sendItem={saveItem}
  />
{/if}
