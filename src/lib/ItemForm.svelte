<script lang="ts">
  import { afterUpdate, onDestroy, onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api'
  import { imgURL } from 'service/file'
  import RatingSelect from '$lib/RatingSelect.svelte'
  import CategorySelect from '$lib/CategorySelect.svelte'
  import { createEventDispatcher } from 'svelte'

  let imgUrl: string = ''
  let defaultRating = 0
  let allCategories: Category[] = []

  const dispatch = createEventDispatcher()

  export let editState: EditState = {
    item: {
      item_id: 0,
      name: '',
      description: '',
      comments: '',
      img_path: '',
      date: '',
    },
    rating: 0,
    categories: [],
  }

  afterUpdate(async () => {
    if (editState.item.img_path && imgUrl === '') {
      imgUrl = await imgURL(editState.item.img_path)
    }
  })

  onMount(async () => {
    allCategories = await invoke('get_categories')
  })

  onDestroy(() => {
    window.URL.revokeObjectURL(imgUrl)
  })

  function handleRating(e) {
    editState.rating = e.detail.rating
  }

  function handleCategory(e) {
    editState.categories = e.detail.categories
    console.log(editState)
  }

  async function update() {
    const file = document.getElementById('imageInput') as HTMLInputElement
    if (file && file.files && file.files[0]) {
      const blob = file.files[0]
      const url = window.URL.createObjectURL(blob)
      window.URL.revokeObjectURL(imgUrl)
      imgUrl = url
    }
  }
</script>

<div class="flex gap-12 justify-center items-center my-24 w-screen">
  <a href="/" class="transition-all hover:text-gray-600">☜ go back </a>
  <div class="flex flex-col">
    <img
      alt="drawing of item"
      id="img"
      src={imgUrl}
      width={300}
      class="bg-gray-500 rounded-2xl"
    />
    <input type="file" accept="image/*" id="imageInput" on:change={update} />
  </div>

  <div class="flex flex-col gap-4">
    <div>
      <p class="tag">name</p>
      <input bind:value={editState.item.name} />
    </div>
    <div>
      <p class="tag">description</p>
      <input
        bind:value={editState.item.description}
        placeholder="description"
      />
    </div>
    <RatingSelect on:rating={handleRating} {defaultRating} />
    <CategorySelect
      on:categories={handleCategory}
      categories={editState.categories}
      {allCategories}
    />
    <div>
      <p class="tag">comments</p>
      <input bind:value={editState.item.comments} placeholder="Comments" />
    </div>
    <button on:click={() => dispatch('sendItem', editState)} class="badge"
      >Save Item</button
    >
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
