<script lang="ts">
  import { onDestroy, onMount } from "svelte"
  import { goto } from "$app/navigation"
  import { page } from "$app/stores"
  import { invoke } from "@tauri-apps/api"
  import { getItem } from "service/db"
  import { deleteImgFromPath, saveFile } from "service/file"
  import ItemForm from "$lib/ItemForm.svelte"

  let id = Number($page.params.id)
  let imgUrl: string = ""
  let item: Item
  let ratings: Rating[]
  let categories: Category[] = []
  let allCategories: Category[] = []
  let editState: EditState

  onMount(async () => {
    item = await getItem(id)
    ratings = await invoke("get_ratings", { itemId: id })
    categories = await invoke("get_categories_for_item", { id })
    allCategories = await invoke("get_categories")
    editState = {
      item,
      rating: ratings[0].rating,
      categories,
    }
  })

  async function saveItem(e: { detail: EditState }) {
    let editedItem = e.detail
    console.log("saveItem run")
    window.URL.revokeObjectURL(imgUrl)
    const file = document.getElementById("imageInput") as HTMLInputElement
    if (file && file.files && file.files[0]) {
      const img_path = await saveFile(file.files[0])
      editedItem = { ...editedItem, item: { ...editedItem.item, img_path } }
      if (item.img_path) {
        deleteImgFromPath(item.img_path)
      }
    }
    await invoke("update_item", { item: editedItem.item, categories: editState.categories })
    await invoke("create_rating", { rating: editedItem.rating, itemId: id })
    goto(`/items/${editedItem.item.item_id}`)
  }
</script>

{#if item}
  <ItemForm {editState} on:sendItem={saveItem} />
{/if}

<style lang="postcss">
  .tag {
    @apply font-bold;
  }

  .badge {
    @apply rounded-full bg-slate-200 text-xs px-3 py-1 hover:bg-slate-300 transition-all;
  }

  a {
    @apply text-sm p-6;
  }
  input {
    @apply border-2 border-blue-400;
  }
</style>
