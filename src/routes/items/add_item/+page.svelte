<script lang="ts">
  import { saveFile } from "../../../service/file"

  function saveImg(e: any) {
    const file = document.getElementById("imageInput") as HTMLInputElement
    const savePath = file.files && file.files[0] && saveFile(file.files[0])

    savePath?.then(console.log)
  }
</script>

<div>
  <input type="file" accept="image/*" id="imageInput" /><button on:click={saveImg}
    >Save Img to Tauri :)
  </button>
</div>
<a class="outline" href="/">Go back</a>
