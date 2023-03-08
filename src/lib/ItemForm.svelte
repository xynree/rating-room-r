<script lang="ts">
  import { onDestroy } from 'svelte'
  import { imgURL, saveFile } from 'service/file'
  import RatingSelect from '$lib/RatingSelect.svelte'
  import CategorySelect from '$lib/CategorySelect.svelte'
  import { createEventDispatcher } from 'svelte'
  import DrawingPane from './DrawingPane.svelte'

  $: imgUrl = ''

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
        <span class="imgtool cursor-pointer">upload image </span>
      </label>
      <!-- <button -->
      <!--   class="imgtool cursor-pointer" -->
      <!--   on:click={() => (drawing = !drawing)} -->
      <!-- > -->
      <!--   {drawing ? 'upload image' : 'draw image'} -->
      <!-- </button> -->
    </div>
    <!-- {#if drawing} -->
    <DrawingPane bind:imgUrl on:updateBlob />
    <!-- {:else} -->
    <!--   <img -->
    <!--     class="h-[500px] w-[500px]" -->
    <!--     alt="drawing of item" -->
    <!--     id="img" -->
    <!--     src={imgUrl} -->
    <!--     width={500} -->
    <!--   /> -->
    <!-- {/if} -->
  </div>

  <div class="flex flex-col gap-4">
    <div>
      <p class="tag">name</p>
      <input class="input" bind:value={editState.name} />
    </div>
    <div>
      <p class="tag">description</p>
      <textarea
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
        class="py-1 px-4 rounded-full hover:bg-neutral-50 border-black border-[1.5px]"
        >cancel</button
      >
      <button
        on:click={() => dispatch('sendItem', editState)}
        class="py-1 px-4 text-white hover:bg-neutral-800  bg-black rounded-full"
        >update</button
      >
    </div>
  </div>
</div>

<style lang="postcss">
  .tag {
    @apply font-bold;
  }

  input,
  textarea {
    @apply px-2 py-1 mt-2 bg-neutral-100  border-black rounded-xl border-2 overflow-ellipsis text-sm w-full;
  }

  input:focus {
    @apply outline-none ring border-black rounded-xl;
  }

  a {
    @apply text-sm p-6;
  }

  .imgtool {
    @apply px-4 py-1 transition-all bg-gray-100 hover:bg-gray-200 rounded-sm border text-xs;
  }
</style>
