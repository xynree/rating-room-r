<script lang="ts">
  import { onDestroy } from 'svelte'
  import { imgURL } from 'service/file'
  import RatingSelect from '$lib/RatingSelect.svelte'
  import CategorySelect from '$lib/CategorySelect.svelte'
  import { createEventDispatcher } from 'svelte'

  let imgUrl: string = ''

  const dispatch = createEventDispatcher()

  export let editState: FullItem = {
    item_id: 0,
    name: '',
    description: '',
    comments: '',
    img_path: '',
    date: '',
    categories: [],
    rating: {
      date: '',
      ratingId: 0,
      rating: 0,
    },
  }

  $: if (editState.img_path && imgUrl === '') {
    imgURL(editState.img_path).then((url) => (imgUrl = url))
  }

  onDestroy(() => {
    window.URL.revokeObjectURL(imgUrl)
  })

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
      <input class="input" bind:value={editState.name} />
    </div>
    <div>
      <p class="tag">description</p>
      <input
        class="input"
        bind:value={editState.description}
        placeholder="description"
      />
    </div>
    <RatingSelect bind:defaultRating={editState.rating.rating} />
    <CategorySelect bind:categories={editState.categories} />
    <div>
      <p class="tag">comments</p>
      <input
        class="input"
        bind:value={editState.comments}
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
