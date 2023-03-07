<script lang="ts">
  import { onDestroy } from 'svelte'
  import { imgURL } from 'service/file'
  import RatingSelect from '$lib/RatingSelect.svelte'
  import CategorySelect from '$lib/CategorySelect.svelte'
  import { createEventDispatcher } from 'svelte'
  import DrawingPane from './DrawingPane.svelte'
  import { page } from '$app/stores'

  let showError: boolean = false
  function toggleError() {
    showError = true
    setTimeout(() => (showError = false), 2000)
  }
  let imgUrl: string = ''
  $: drawing = false

  let pageName =
    $page.route.id?.split('/').slice(-1)[0] === 'add_item'
      ? 'create item'
      : 'update'
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
      window.URL.revokeObjectURL(imgUrl)
      imgUrl = window.URL.createObjectURL(blob)
      dispatch('updateBlob', blob)
    }
  }

  function formErrors() {
    return editState.name === '' || !editState.categories.length
  }
</script>

<div class="flex gap-12 justify-center items-center my-24 w-screen">
  <a href="/" class="transition-all hover:text-gray-600">☜ go back </a>
  <div class="flex flex-col gap-2 relative">
    <div class="absolute -bottom-10 gap-2 items-center justify-center">
      <label>
        <input
          class="hidden input"
          type="file"
          accept="image/*"
          id="imageInput"
          on:change={update}
        />
        {#if !drawing}
          <span class="imgtool cursor-pointer">upload image </span>
        {/if}
      </label>
      <button
        class="imgtool cursor-pointer"
        on:click={() => (drawing = !drawing)}
      >
        {drawing ? 'upload image' : 'draw image'}
      </button>
    </div>
    {#if drawing}
      <DrawingPane on:updateBlob {imgUrl} />
    {:else}
      <img alt="drawing of item" id="img" src={imgUrl} width={500} />
    {/if}
  </div>

  <div class="flex flex-col gap-4">
    <div>
      <p
        class={`transition-all ${
          showError ? 'opacity-100' : 'opacity-0'
        } text-red-700  font-light text-xs`}
      >
        Missing name or categories
      </p>
      <p class="tag flex items-center gap-1">
        name
        {#if editState.name === ''}<span class="alert">!</span>{/if}
      </p>
      <input
        class="input"
        class:ring-2={editState.name === ''}
        class:ring-red-500={editState.name === ''}
        bind:value={editState.name}
      />
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
        on:click={() => {
          formErrors() ? toggleError() : dispatch('sendItem', editState)
        }}
        class="py-1 px-4 text-white bg-black rounded-full">{pageName}</button
      >
    </div>
  </div>
</div>

<style lang="postcss">
  .tag {
    @apply font-bold;
  }

  input {
    @apply rounded-md px-1 mt-2 border;
  }

  a {
    @apply text-sm p-6;
  }

  .alert {
    @apply text-white bg-red-500 rounded-full px-2 text-xs font-normal;
  }

  .imgtool {
    @apply px-4 py-1 transition-all bg-gray-100 hover:bg-gray-200 rounded-sm border text-xs;
  }
</style>
