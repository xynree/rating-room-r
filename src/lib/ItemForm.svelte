<script lang="ts">
  import { afterUpdate, onDestroy, onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api'
  import { imgURL } from 'service/file'
  import RatingSelect from '$lib/RatingSelect.svelte'
  import CategorySelect from '$lib/CategorySelect.svelte'
  import { createEventDispatcher } from 'svelte'
  import { goto } from '$app/navigation'

  export let defaultRating = 0
  let imgUrl: string = ''
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
    />
    <div>
      <p class="tag">comments</p>
      <input bind:value={editState.item.comments} placeholder="Comments" />
    </div>
    <div class="flex gap-1 py-6">
      <button on:click={() => goto('/')}>cancel</button>
      <button on:click={() => dispatch('sendItem', editState)}>update</button>
    </div>
  </div>
</div>

<style lang="postcss">
  .tag {
    @apply font-bold;
  }

  button {
    @apply border-black border-2 rounded-full px-4 text-sm transition-all;

    &:hover {
      @apply bg-gray-100;
    }

    &:nth-child(2) {
      @apply bg-black text-white;

      &:hover {
        @apply bg-gray-900;
      }
    }
  }

  .badge {
    @apply rounded-full bg-zinc-100 text-xs px-3 py-1 transition-all;

    &:hover {
      @apply bg-zinc-300;
    }
  }

  a {
    @apply text-sm p-6;
  }
  input:not(#imageInput) {
    @apply border-2 border-black rounded-lg px-1 my-1 bg-stone-100;
    &:focus {
      @apply outline-none ring ring-gray-800 border-white;
    }
  }
</style>
