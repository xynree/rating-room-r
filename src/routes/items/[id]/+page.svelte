<script lang="ts">
  import { goto } from '$app/navigation'
  import { page } from '$app/stores'
  import { invoke } from '@tauri-apps/api'
  import { getItem } from 'service/db'
  import { imgURL } from 'service/file'
  import { itemsStore } from 'store'

  let item: Item
  let url: string
  let ratings: Rating[]
  let categories: Category[]

  $: id = Number($page.params.id)
  $: getItem(id).then(async (i) => {
    item = i
    url = await imgURL(i.img_path)
    ratings = await invoke('get_ratings', { itemId: i.item_id })
    categories = await invoke('get_categories_for_item', { id: i.item_id })
  })
  $: itemIdx = $itemsStore.findIndex((i) => i.item_id === id)
  $: prev = itemIdx - 1 < 0 ? null : $itemsStore[itemIdx - 1]
  $: next = itemIdx + 1 > $itemsStore.length ? null : $itemsStore[itemIdx + 1]

  onkeydown = (e) => {
    switch (e.code) {
      case 'ArrowLeft':
        itemIdx - 1 < 0 || navigate.prev()
        break
      case 'ArrowRight':
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

{#if item}
  <div class="flex gap-10 justify-center items-center w-screen h-3/4">
    <a href="/" class="transition-all hover:text-gray-600 w-36 text-center"
      >☜ go back
    </a>

    <img
      alt="drawing of item"
      src={url}
      width={450}
      class="bg-gray-500 border border-black rounded-sm"
    />
    <div class="flex flex-col gap-3 w-1/3 ">
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
        <div class="flex">
          {#if ratings}
            {#each Array(ratings[0].rating) as _}
              <p>★</p>
            {/each}
          {/if}
        </div>
      </div>
      <div>
        <p class="tag">categories</p>
        <div class="flex gap-1">
          {#if categories}
            {#each categories as { name }}
              <p class="badge">{name}</p>
            {/each}
          {/if}
        </div>
      </div>
      <div>
        <p class="tag">comments</p>
        <p>{item?.comments || 'Nothing to say.'}</p>
      </div>
      <div>
        <p class="tag">last rated</p>
        <p class="badge">
          {ratings && new Date(ratings[0].date).toDateString()}
        </p>
      </div>
      {#if next}
        <button class="navigation" on:click={navigate.next}
          >{next.name} →</button
        >
      {/if}
    </div>
  </div>
{/if}

<style lang="postcss">
  .tag {
    @apply font-bold;
  }

  .badge {
    @apply rounded-full bg-zinc-200 text-xs px-4 py-1 w-fit;
  }

  a {
    @apply text-sm;
  }

  .navigation {
    @apply text-sm font-normal py-4 hover:text-gray-600 hover:cursor-pointer transition-all text-left;
  }
</style>
