<script lang="ts">
  import { afterUpdate, onDestroy, onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api'
  import { imgURL } from 'service/file'
  import RatingSelect from '$lib/RatingSelect.svelte'
  import CategorySelect from '$lib/CategorySelect.svelte'
  import { createEventDispatcher } from 'svelte'
  import { goto } from '$app/navigation'

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

  let imgUrl: string = ''
  let defaultRating = 0
  let allCategories: Category[] = []
  const dispatch = createEventDispatcher()

  $: invoke('get_categories').then((c) => {
    allCategories = c as Category[]
  })

  $: if (editState.item.img_path && imgUrl === '') {
    imgURL(editState.item.img_path).then((url) => (imgUrl = url))
  }

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
    <label class="absolute m-2 text-xs underline hover:cursor-pointer">
      <input
        class="hidden input"
        type="file"
        accept="image/*"
        id="imageInput"
        on:change={update}
      />
      switch image
    </label>
    <img
      alt="drawing of item"
      id="img"
      src={imgUrl}
      width={300}
      class="bg-gray-500 rounded-2xl"
    />
  </div>

  <div class="flex flex-col gap-4">
    <div>
      <p class="tag">name</p>
      <input class="input" bind:value={editState.item.name} />
    </div>
    <div>
      <p class="tag">description</p>
      <input
        class="input"
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
      <input
        class="input"
        bind:value={editState.item.comments}
        placeholder="Comments"
      />
    </div>
    <div class="flex flex-row gap-2">
      <button
        on:click={() => dispatch('cancel')}
        class="py-1 px-4 rounded-full border-black border-[1.5px]"
        >cancel</button
      >
      <button
        on:click={() => dispatch('sendItem', editState)}
        class="py-1 px-4 text-white bg-black rounded-full">update</button
      >
    </div>
  </div>
</div>

<style lang="postcss">
  .tag {
    @apply font-bold;
  }

  input {
    @apply border-black border-[1.5px] rounded-md bg-neutral-100 px-1 mt-2;
  }

  .badge {
    @apply rounded-full bg-neutral-100 text-xs px-3 py-1 hover:bg-slate-300 transition-all;
  }

  a {
    @apply text-sm p-6;
  }
</style>
