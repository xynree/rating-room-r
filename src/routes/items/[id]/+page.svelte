<script lang="ts">
  import { page } from "$app/stores"
  import { invoke } from "@tauri-apps/api"
  import { getItem } from "../../../service/db"
  import { appDataDir, join } from "@tauri-apps/api/path"
  import { convertFileSrc } from "@tauri-apps/api/tauri"
  let id = Number($page.params.id)

  let imgUrl: string = ""
  let item: Item
  getItem(id).then((r) => {
    item = r

    getImage(item)
  })

  async function getImage(item: Item) {
    const appDataDirPath = await appDataDir()
    const imgPath = await join(appDataDirPath, `imgs/${item.img_path}`)
    imgUrl = convertFileSrc(imgPath)
  }

  let ratings: Rating[]
  invoke("get_ratings", { itemId: id }).then((r) => {
    ratings = r as Rating[]
    console.log(r)
  })

  let categories: Category[] = []
  invoke("get_categories_for_item", { id }).then((cat) => {
    categories = cat as Category[]
    console.log(categories)
  })
</script>

<div class="flex items-center justify-center w-screen gap-12 my-24">
  <a href="/" class="transition-all hover:text-gray-600">☜ go back </a>
  <img alt="drawing of item" src={imgUrl} class="w-48 h-48 bg-gray-500 rounded-2xl" />
  <div class="flex flex-col gap-4">
    <div>
      <p class="tag">name</p>
      <p>{item?.name}</p>
    </div>
    <div>
      <p class="tag">description</p>
      <p>{item?.description || "no description"}</p>
    </div>
    <div>
      <p class="tag">rating</p>
      <p>{ratings && ratings[0].rating}</p>
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
      <p>{item?.comments || "no comments"}</p>
    </div>
    <div>
      <p class="tag">last rated</p>
      <p>{ratings && new Date(ratings[0].date).toDateString()}</p>
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
