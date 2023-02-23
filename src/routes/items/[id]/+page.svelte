<script lang="ts">
  import { page } from "$app/stores"
  import { invoke } from "@tauri-apps/api"
  import { getItem } from "../../../service/db"

  let id = Number($page.params.id)

  let item: Item
  getItem(id).then((r) => (item = r))

  let ratings: Rating[]
  invoke("get_ratings", { itemId: id }).then((r) => {
    ratings = r as Rating[]
    console.log(r)
  })
</script>

<div class="flex items-center justify-center w-screen gap-12 my-24">
  <a href="/" class="transition-all hover:text-gray-600">☜ go back </a>
  <img alt="image here" class="w-48 h-48 bg-gray-500 rounded-2xl" />
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
      <p>categories here</p>
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

  a {
    @apply text-sm p-6;
  }
</style>
