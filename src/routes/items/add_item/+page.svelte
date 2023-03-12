<script lang="ts">
  import { goto } from '$app/navigation'
  import { saveFile } from 'service/file'
  import ItemForm from '$lib/ItemForm.svelte'
  import { itemsStore } from 'store'
  import { addItem } from 'service/db'

  let imgUrl: string = ''
  let imgBlob: Blob | File | null = null

  if ($itemsStore.items.length == 0) {
    itemsStore.refresh()
  }

  function updateBlob(e: { detail: Blob | File | null }) {
    imgBlob = e.detail
  }

  async function createItem(e: { detail: FullItem }) {
    let editState = e.detail
    if (editState.name === '' || !editState.categories.length) {
      return
    }
    window.URL.revokeObjectURL(imgUrl)

    if (imgBlob) {
      const img_path = await saveFile(imgBlob)
      editState = { ...editState, img_path }
      imgBlob = null
    }

    const newItemId = await addItem(editState)
    goto(`/items/${newItemId}`)
  }
</script>

<ItemForm on:sendItem={createItem} on:updateBlob={updateBlob} />
