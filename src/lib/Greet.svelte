<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = ""
  let greetMsg = ""

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet")
  }

  greet()
</script>

<div>
  <div>
    <p>{greetMsg}</p>
  </div>
</div>
