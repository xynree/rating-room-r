<script lang="ts">
  import { page } from "$app/stores"
  import { invoke } from "@tauri-apps/api"
  import { getItem } from "../../../service/db"
  let id = Number($page.params.id)

  let item: Item
  getItem(id).then((r) => (item = r))

  let ratings: Rating[]
  invoke("get_ratings", { id }).then((r) => (ratings = r as Rating[]))
</script>

<div class="flex items-center justify-center w-screen gap-12 my-24">
  <a href="/" class="transition-all hover:text-gray-600">☜ go back </a>
  <img src={item?.img_path} class="w-48 h-48 bg-gray-500" />
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
      <p>{(ratings && ratings[0]) || "get rating number"}</p>
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
      <p class="tag">date added</p>
      <p>{item?.date || "no date"}</p>
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
