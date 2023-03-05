<script lang="ts">
  import { goto } from '$app/navigation'
  import { page } from '$app/stores'
  import { itemsStore, itemView } from 'store'
  import { invoke } from '@tauri-apps/api'
  import { deleteImgFromPath, saveFile } from 'service/file'
  import ItemForm from '$lib/ItemForm.svelte'

  let imgUrl: string = ''
  let imgBlob: Blob | File | null = null

<<<<<<< HEAD
  let imgUrl: string
  let item: FullItem | undefined
  let editState: FullItem | undefined

  $: id = Number($page.params.id)
  $: {
    item = $itemView.find((i) => i.item_id == id)
    editState = item
  }
=======
  let imgUrl: string = ''
  let imgBlob: Blob | File | null = null

  $: id = Number($page.params.id)
  $: item = $itemView.find((i) => i.item_id == id)
  $: editState = item
>>>>>>> a628636 (generalized image saving functions to take file or blob)

  function updateBlob(e) {
    console.log('blob updated!', e.detail)
    imgBlob = e.detail
  }

  async function saveItem(e: { detail: FullItem }) {
    let editedItem = e.detail
    window.URL.revokeObjectURL(imgUrl)
<<<<<<< HEAD
    const file = document.getElementById('imageInput') as HTMLInputElement
    if (file && file.files && file.files[0]) {
      const img_path = await saveFile(file.files[0])
      if (item?.img_path) {
        deleteImgFromPath(item.img_path)
      }
      // editedItem.img_path = img_path
      editedItem = { ...editedItem, img_path }
=======

    if (imgBlob) {
      const img_path = await saveFile(imgBlob)
      editedItem = { ...editedItem, img_path }
      if (item?.img_path) {
        deleteImgFromPath(item.img_path)
      }
      imgBlob = null
>>>>>>> a628636 (generalized image saving functions to take file or blob)
    }


    await invoke('update_item', {
      item: editedItem,
      categories: editState?.categories,
    })

    await invoke('create_rating', {
      rating: editedItem.rating.rating,
      itemId: id,
    })

    // itemsStore.fetch()
    await invoke('get_items').then((items) => ($itemsStore.items = items))
    goto(`/items/${id}`)
  }
</script>

{#if item}
  <ItemForm
    {editState}
    on:cancel={() => goto(`/items/${id}`)}
    on:sendItem={saveItem}
    on:updateBlob={updateBlob}
  />
{/if}
