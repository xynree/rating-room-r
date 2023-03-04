<script lang="ts">
  import { onMount } from 'svelte'
  import { goto } from '$app/navigation'
  import { invoke } from '@tauri-apps/api'
  import { saveFile } from 'service/file'
  import ItemForm from '$lib/ItemForm.svelte'

  let imgUrl: string = ''
  let allCategories: Category[] = []

  onMount(async () => {
    allCategories = await invoke('get_categories')
  })

  async function createItem(e: { detail: EditState }) {
    console.log('running Create Item...')
    let editState = e.detail
    window.URL.revokeObjectURL(imgUrl)
    const file = document.getElementById('imageInput') as HTMLInputElement
    if (file && file.files && file.files[0]) {
      const img_path = await saveFile(file.files[0])
      editState = { ...editState, item: { ...editState.item, img_path } }
    }

    const newItemId = await invoke('create_item', {
      name: editState.item.name,
      description: editState.item.description,
      comments: editState.item.comments,
      imgPath: editState.item.img_path,
    })
    console.log(newItemId)
    await invoke('add_categories_to_item', {
      itemId: newItemId,
      categories: editState.categories,
    })
    await invoke('create_rating', {
      rating: editState.rating,
      itemId: newItemId,
    })

    goto(`/items/${newItemId}`)
  }
</script>

<ItemForm on:sendItem={createItem} />
