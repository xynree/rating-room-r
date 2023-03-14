<script lang="ts">
  import { goto } from '$app/navigation'
  import { page } from '$app/stores'
  import { itemsStore, itemView } from 'store'
  import { deleteImgFromPath, saveFile } from 'service/file'
  import ItemForm from '$lib/ItemForm.svelte'
  import { createRating, updateItem } from 'service/api'

  let imgUrl: string = ''
  let imgBlob: Blob | File | null = null

  let item: FullItem | undefined
  let editState: FullItem | undefined

  $: id = Number($page.params.id)
  $: {
    if ($itemsStore.items.length == 0) {
      itemsStore.refresh()
    }
    item = $itemView.find((i) => i.item_id == id)
    editState = item
  }

  function updateBlob(e) {
    imgBlob = e.detail
  }

  async function saveItem(e: { detail: FullItem }) {
    let editedItem = e.detail
    if (editedItem.name === '') return
    window.URL.revokeObjectURL(imgUrl)

    if (imgBlob) {
      const img_path = await saveFile(imgBlob)
      editedItem = { ...editedItem, img_path }
      if (item?.img_path) {
        deleteImgFromPath(item.img_path)
      }
      imgBlob = null
    }

    await updateItem({
      item: editedItem,
      categories: editState?.categories,
    })

    await createRating({ rating: editedItem.rating.rating, itemId: id })

    itemsStore.refresh()
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
