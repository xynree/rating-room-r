<script lang="ts">
  import { onMount } from "svelte"
  import { goto } from "$app/navigation"
  import { page } from "$app/stores"
  import { invoke } from "@tauri-apps/api"
  import { getItem } from "service/db"
  import { imgURL, saveFile } from "service/file"
  import RatingSelect from "$lib/RatingSelect.svelte"
  import CategorySelect from "$lib/CategorySelect.svelte"

  let id = Number($page.params.id)
  let imgUrl: string = ""
  let item: Item
  let ratings: Rating[]
  let categories: Category[] = []
  let allCategories: Category[] = []

  let tempURL: string = ""

  type EditState = {
    item: Item
    rating: number
    categories: Category[]
  }

  let editState: EditState = {
    item: {
      item_id: 0,
      name: "",
      description: "",
      comments: "",
      img_path: "",
      date: "",
    },
    rating: 0,
    categories: [],
  }

  onMount(async () => {
    item = await getItem(id)
    imgUrl = await imgURL(item.img_path)
    ratings = await invoke("get_ratings", { itemId: id })
    categories = await invoke("get_categories_for_item", { id })
    allCategories = await invoke("get_categories")
    editState = {
      item: { ...item },
      rating: ratings[0].rating,
      categories: [...categories],
    }
  })

  function handleRating(e) {
    editState.rating = e.detail.rating
  }

  function handleCategory(e) {
    editState.categories = e.detail.categories
  }

  async function saveImg() {
    const file = document.getElementById("imageInput") as HTMLInputElement
    if (file && file.files) {
      const img_path = await saveFile(file.files[0])
      editState = { ...editState, item: { ...editState.item, img_path } }
      console.log(editState)
    }
  }

  async function update() {
    const file = document.getElementById("imageInput") as HTMLInputElement
    if (file && file.files) {
      const blob = file.files[0]
      const url = window.URL.createObjectURL(blob)
      window.URL.revokeObjectURL(imgUrl)
      imgUrl = url
    }
  }

  async function saveItem() {
    window.URL.revokeObjectURL(imgUrl)
    await saveImg()
    await invoke("update_item", { item: editState.item, categories: editState.categories })
    await invoke("create_rating", { rating: editState.rating, itemId: id })
    goto(`/items/${editState.item.item_id}`)
  }
</script>

<div class="flex gap-12 justify-center items-center my-24 w-screen">
  <a href="/" class="transition-all hover:text-gray-600">☜ go back </a>
  <div class="flex flex-col">
    <img alt="drawing of item" id="img" src={imgUrl} width={300} class="bg-gray-500 rounded-2xl" />
    <input type="file" accept="image/*" id="imageInput" on:change={update} />
  </div>

  <div class="flex flex-col gap-4">
    <div>
      <p class="tag">name</p>
      <input bind:value={editState.item.name} />
    </div>
    <div>
      <p class="tag">description</p>
      <input bind:value={editState.item.description} placeholder="description" />
    </div>
    {#if ratings}
      <RatingSelect on:rating={handleRating} rating={ratings[0].rating} />
    {/if}
    <CategorySelect on:categories={handleCategory} {categories} {allCategories} />
    <div>
      <p class="tag">comments</p>
      <input bind:value={editState.item.comments} placeholder="Comments" />
    </div>
    <button on:click={saveItem} class="badge">Save Item</button>
  </div>
</div>

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
