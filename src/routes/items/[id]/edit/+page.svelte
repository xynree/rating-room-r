<script lang="ts">
  import { onMount } from "svelte"
  import { page } from "$app/stores"
  import { invoke } from "@tauri-apps/api"
  import { getItem } from "../../../../service/db"
  import { imgURL } from "../../../../service/file"

  let id = Number($page.params.id)
  let imgUrl: string = ""
  let item: Item
  let ratings: Rating[]
  let categories: Category[] = []
  let allCategories: Category[] = []

  onMount(async () => {
    item = await getItem(id)
    imgUrl = await imgURL(item.img_path)
    ratings = await invoke("get_ratings", { itemId: id })
    categories = await invoke("get_categories_for_item", { id })
    allCategories = await invoke("get_categories")
  })
</script>

<div class="flex items-center justify-center w-screen gap-12 my-24">
  <a href="/" class="transition-all hover:text-gray-600">☜ go back </a>
  <img alt="drawing of item" src={imgUrl} class="w-48 h-48 bg-gray-500 rounded-2xl" />
  <div class="flex flex-col gap-4">
    <div>
      <p class="tag">name</p>
      <input value={item?.name} />
    </div>
    <div>
      <p class="tag">description</p>
      <input value={item?.description} placeholder="description" />
    </div>
    <div>
      <p class="tag">rating</p>
      {#if ratings}
        <select class="flex text-slate-600">
          {#each Array(5) as _, i}
            <option value={i} selected={Number(ratings[0].rating) === i + 1}>
              {#each Array(i + 1) as _}★{/each}
            </option>
          {/each}
        </select>
      {/if}
    </div>
    <div>
      <p class="tag">categories</p>
      <div class="flex">
        {#each categories as { name }}
          <div class="flex gap-1 badge"><button>x</button>{name}</div>
        {/each}
        <select class="badge">
          <option>add category</option>
          {#each allCategories as category}<option>{category.name}</option>{/each}
        </select>
      </div>
    </div>
    <div>
      <p class="tag">comments</p>
      <input value={item?.comments} placeholder="Comments" />
    </div>
    <button class="badge">Save Item</button>
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
