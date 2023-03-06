<script lang="ts">
  import { onMount } from 'svelte'
  import { goto } from '$app/navigation'
  import { invoke } from '@tauri-apps/api'
  import { saveFile } from 'service/file'
  import ItemForm from '$lib/ItemForm.svelte'
  import { itemsStore } from 'store'

  let imgUrl: string = ''
  let imgBlob: Blob | File | null = null

  if ($itemsStore.items.length == 0) {
    invoke('get_items').then(
      (items) => ($itemsStore.items = items as FullItem[])
    )
  }

  function updateBlob(e: { detail: Blob | File | null }) {
    imgBlob = e.detail
  }

  async function createItem(e: { detail: FullItem }) {
    let editState = e.detail
    window.URL.revokeObjectURL(imgUrl)

    if (imgBlob) {
      const img_path = await saveFile(imgBlob)
      editState = { ...editState, img_path }
      imgBlob = null
    }

    const newItemId = await invoke('create_item', {
      name: editState.name,
      description: editState.description,
      comments: editState.comments,
      imgPath: editState.img_path || '',
    })
    await invoke('add_categories_to_item', {
      itemId: newItemId,
      categories: editState.categories,
    })
    await invoke('create_rating', {
      rating: editState.rating.rating,
      itemId: newItemId,
    })

    // itemsStore.fetch()
    await invoke('get_items').then(
      (items) => ($itemsStore.items = items as FullItem[])
    )

    goto(`/items/${newItemId}`)
  }
</script>

<ItemForm on:sendItem={createItem} on:updateBlob={updateBlob} />
