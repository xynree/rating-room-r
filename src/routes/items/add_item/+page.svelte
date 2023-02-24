<script lang="ts">
  import { invoke } from "@tauri-apps/api"
  import { onMount } from "svelte"
  import { saveFile } from "../../../service/file"

  function saveImg(e: any) {
    const file = document.getElementById("imageInput") as HTMLInputElement
    const savePath = file.files && file.files[0] && saveFile(file.files[0])

    savePath?.then(console.log)
  }
  let categories: Category[] = []
  onMount(async () => {
    categories = await invoke("get_categories")
  })
</script>

<nav class="flex justify-between w-screen p-6 text-sm font-medium shadow-sm">
  <a href="/">☜ go back</a>
  <div class="flex gap-5">
    <p>add item</p>
  </div>
</nav>
<div class="flex items-center justify-center w-screen gap-12 my-8">
  <div class="flex flex-col gap-4">
    <div>
      <p class="tag">name</p>
      <input />
    </div>
    <div>
      <p class="tag">description</p>
      <textarea class="outline" />
    </div>
    <div>
      <p class="tag">rating</p>
      <input type="range" min="0" max="5" />
    </div>
    <div>
      <p class="tag">categories</p>
      <select>
        {#each categories as category}
          <option>{category.name}</option>
        {/each}
      </select>
    </div>
    <div>
      <p class="tag">comments</p>
      <input />
    </div>
    <div>
      <div>image</div>
      <input type="file" accept="image/*" id="imageInput" />
    </div>
    <button on:click={saveImg} class="p-2 bg-gray-300">Save Item</button>
  </div>
</div>

<style>
  input {
    @apply outline;
  }
</style>
