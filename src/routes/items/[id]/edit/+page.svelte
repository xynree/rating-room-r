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

  onMount(async () => {
    item = await getItem(id)
    imgUrl = await imgURL(item.img_path)
    ratings = await invoke("get_ratings", { itemId: id })
    categories = await invoke("get_categories_for_item", { id })
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
          <p class="badge">{name}</p>
        {/each}
      </div>
    </div>
    <div>
      <p class="tag">comments</p>
      <input value={item?.comments} placeholder="Comments" />
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
  input {
    @apply border-2 border-blue-400;
  }
</style>
