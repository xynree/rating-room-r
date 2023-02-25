<script lang="ts">
  import CategorySelect from "$lib/CategorySelect.svelte"
  import RatingSelect from "$lib/RatingSelect.svelte"
  import { invoke } from "@tauri-apps/api"
  import { onDestroy, onMount } from "svelte"
  import { saveFile } from "../../../service/file"

  let imgUrl = ""

  function saveImg(e: any) {
    const file = document.getElementById("imageInput") as HTMLInputElement
    const savePath = file.files && file.files[0] && saveFile(file.files[0])

    savePath?.then(console.log)
  }

  async function update() {
    const file = document.getElementById("imageInput") as HTMLInputElement
    if (file && file.files) {
      const blob = file.files[0]
      const url = window.URL.createObjectURL(blob)
      window.URL.revokeObjectURL(imgUrl)
      imgUrl = url
    }
  }

  let allCategories: Category[] = []

  onMount(async () => {
    allCategories = await invoke("get_categories")
  })

  onDestroy(() => {
    window.URL.revokeObjectURL(imgUrl)
  })
</script>

<nav class="flex justify-between w-screen p-6 text-sm font-medium shadow-sm">
  <a href="/">☜ go back</a>
  <p>add item</p>
</nav>
<div class="flex items-center justify-center w-screen gap-12 my-8">
  <div class="flex flex-col">
    <img
      alt="drawing of item"
      id="img"
      src={imgUrl}
      width={300}
      height={300}
      class="bg-gray-300 rounded-lg"
    />
    <input type="file" accept="image/*" id="imageInput" on:change={update} />
  </div>
  <div class="flex flex-col gap-4">
    <div>
      <p class="tag">name</p>
      <input />
    </div>
    <div>
      <p class="tag">description</p>
      <textarea />
    </div>
    <div>
      <RatingSelect rating={0} />
    </div>
    <div>
      <CategorySelect {allCategories} />
    </div>
    <div>
      <p class="tag">comments</p>
      <input />
    </div>
    <button class="badge"> Create New Item </button>
  </div>
</div>

<style lang="postcss">
  .tag {
    @apply font-bold;
  }

  .badge {
    @apply rounded-full bg-slate-200 text-xs px-3 py-1 hover:bg-slate-300 transition-all;
  }

  input,
  textarea {
    @apply border-2 border-blue-400;
  }
</style>
