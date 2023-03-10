<script lang="ts">
  import { goto } from '$app/navigation'
  import { page } from '$app/stores'
  import { imgURL } from 'service/file'
  import { FilterType, itemsStore, itemView } from 'store'

  let item: FullItem
  let url: string
  let itemIdx: number
  let prev: FullItem | null
  let next: FullItem | null

  $: id = Number($page.params.id)
  $: {
    if ($itemsStore.items.length == 0) {
      itemsStore.refresh()
    }
    itemIdx = $itemView.findIndex((i) => i.item_id === id)
    item = $itemView[itemIdx]
    imgURL(item?.img_path).then((path) => (url = path))
    prev = itemIdx - 1 < 0 ? null : $itemView[itemIdx - 1]
    next = itemIdx + 1 > $itemView.length ? null : $itemView[itemIdx + 1]
  }

  onkeydown = (e) => {
    switch (e.code) {
      case 'ArrowLeft':
        itemIdx - 1 < 0 || navigate.prev()
        break
      case 'ArrowRight':
        itemIdx + 1 >= $itemView.length || navigate.next()
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

  function viewAllInCategory(e) {
    const category = e.target.name
    itemsStore.filters.reset()
    itemsStore.filters.toggle(FilterType.categories, category)
    goto('/')
  }
</script>

{#if item}
  <div class="flex gap-12 justify-center items-center my-24 w-screen h-7/8">
    <a href="/" class="transition-all w-1/4 text-right hover:text-gray-600"
      >☜ go back
    </a>
    <img alt="drawing of item" src={url} width={500} />
    <div class="flex relative flex-col gap-8 w-1/4">
      {#if prev}
        <button
          class="navigation absolute top-[-6rem]"
          on:click={navigate.prev}
        >
          ← {prev.name}
        </button>
      {/if}
      <div>
        <p class="tag">name</p>
        <p class="text-lg">{item.name}</p>
      </div>
      <div>
        <p class="tag">description</p>
        <p class="text-lg">{item.description || 'no description'}</p>
      </div>
      <div>
        <p class="tag">rating</p>
        <div class="flex text-slate-600">
          {#each Array(item.rating.rating) as _}
            <p class="text-yellow-400">★</p>
          {/each}
        </div>
      </div>
      <div>
        <p class="tag">categories</p>
        <div class="flex gap-2">
          {#if item.categories}
            {#each item.categories as { name }}
              <button
                class="badge mt-1"
                on:click={viewAllInCategory}
                on:keydown={viewAllInCategory}
                {name}
              >
                {name}
              </button>
            {/each}
          {/if}
        </div>
      </div>
      <div>
        <p class="tag">comments</p>
        <p class="text-lg">{item.comments || 'Nothing to say.'}</p>
      </div>
      <div>
        <p class="tag">last rated</p>
        <p class="text-lg">
          {item.rating && new Date(item.rating.date).toDateString()}
        </p>
      </div>
      {#if next}
        <button
          class="navigation absolute bottom-[-6rem]"
          on:click={navigate.next}>{next.name} →</button
        >
      {/if}
    </div>
  </div>
{/if}

<style lang="postcss">
  .tag {
    @apply font-semibold;
  }

  .badge {
    @apply rounded-full transition-all bg-neutral-200 hover:bg-neutral-300 px-3 py-1;
  }

  a {
    @apply text-sm;
  }

  .navigation {
    @apply text-sm py-6 hover:text-gray-600 hover:cursor-pointer transition-all text-left w-full font-light;
  }
</style>
