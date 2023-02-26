<script lang="ts">
  import { goto } from '$app/navigation'
  import { page } from '$app/stores'
  import { invoke } from '@tauri-apps/api'
  import { getItem } from 'service/db'
  import { imgURL } from 'service/file'
  import { itemsStore } from 'store'
  import { afterUpdate, onMount } from 'svelte'

  let id = Number($page.params.id)
  let imgUrl: string = ''
  let item: Item
  let ratings: Rating[]
  let categories: Category[] = []
  let prev: Item | null
  let next: Item | null
  let itemIdx: number

  const refresh = async () => {
    item = await getItem(id)
    itemIdx = $itemsStore.findIndex((i) => i.item_id === id)
    prev = itemIdx - 1 < 0 ? null : $itemsStore[itemIdx - 1]
    next = itemIdx + 1 > $itemsStore.length ? null : $itemsStore[itemIdx + 1]
    imgUrl = await imgURL(item.img_path)
    ratings = await invoke('get_ratings', { itemId: id })
    categories = await invoke('get_categories_for_item', { id })
  }

  onMount(refresh)

  afterUpdate(() => {
    let newId = Number($page.params.id)
    if (id !== newId) {
      id = newId
      refresh()
    }
  })

  onkeydown = (e) => {
    switch (e.code) {
      case 'ArrowLeft':
      case 'KeyH':
        itemIdx - 1 < 0 || navigate.prev()
        break
      case 'ArrowRight':
      case 'KeyL':
        itemIdx + 1 >= $itemsStore.length || navigate.next()
        break
      default:
        break
    }
  }

  const navigate = {
    prev: () => {
      goto(`/items/${prev?.item_id}`)
    },
    next: () => {
      goto(`/items/${next?.item_id}`)
    },
  }
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
    {#if prev}
      <button class="navigation" on:click={navigate.prev}>
        ← {prev.name}
      </button>
    {/if}
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
    {#if next}
      <button class="navigation" on:click={navigate.next}>{next.name} →</button>
    {/if}
  </div>
</div>

<style lang="postcss">
  .tag {
    @apply font-bold;
  }

  .badge {
    @apply rounded-full bg-slate-200 text-xs px-3 py-1;
  }

  a {
    @apply text-sm;
  }

  .navigation {
    @apply text-sm font-black py-6 hover:text-gray-600 hover:cursor-pointer transition-all text-left;
  }
</style>
