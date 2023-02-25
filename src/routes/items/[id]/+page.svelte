<script lang="ts">
  import { page } from '$app/stores'
  import { invoke } from '@tauri-apps/api'
  import { getItem } from 'service/db'
  import { imgURL } from 'service/file'
  import { onMount } from 'svelte'

  let id = Number($page.params.id)
  let imgUrl: string = ''
  let item: Item
  let ratings: Rating[]
  let categories: Category[] = []

  onMount(async () => {
    item = await getItem(id)
    imgUrl = await imgURL(item.img_path)
    ratings = await invoke('get_ratings', { itemId: id })
    categories = await invoke('get_categories_for_item', { id })
  })
</script>

<div class="flex gap-12 justify-center items-center my-24 w-screen">
  <a href="/" class="transition-all hover:text-gray-600">☜ go back </a>
  <img
    alt="drawing of item"
    src={imgUrl}
    width={300}
    class="bg-gray-500 rounded-2xl"
  />
  <div class="flex flex-col gap-4">
    <div>
      <p class="tag">name</p>
      <p>{item?.name}</p>
    </div>
    <div>
      <p class="tag">description</p>
      <p>{item?.description || 'no description'}</p>
    </div>
    <div>
      <p class="tag">rating</p>
      {#if ratings}
        <div class="flex text-slate-600">
          {#each Array(ratings[0].rating) as _}
            <p>★</p>
          {/each}
        </div>
      {/if}
    </div>
    <div>
      <p class="tag">categories</p>
      <div class="flex gap-2">
        {#each categories as { name }}
          <p class="badge">{name}</p>
        {/each}
      </div>
    </div>
    <div>
      <p class="tag">comments</p>
      <p class="text-sm">{item?.comments || 'Nothing to say.'}</p>
    </div>
    <div>
      <p class="tag">last rated</p>
      <p class="text-sm">
        {ratings && new Date(ratings[0].date).toDateString()}
      </p>
    </div>
  </div>
</div>

<style>
  .tag {
    @apply font-bold;
  }

  .badge {
    @apply rounded-full bg-slate-200 text-xs px-3 py-1;
  }

  a {
    @apply text-sm p-6;
  }
</style>