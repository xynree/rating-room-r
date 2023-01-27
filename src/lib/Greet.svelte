<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

    let response = "";
    let rating_key = "";

    let new_rating = {
        name: "",
        cost: "",
    };

    async function get() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        response = await invoke("get", { key: rating_key });
    }

    async function insert() {
        await invoke("insert", { rating: new_rating });
        response = `Inserted ${new_rating.name}`;
    }
</script>

<div>
    <div class="m-5">
        <h1>Insert Rating</h1>
        <input placeholder="Name..." bind:value={new_rating.name} />
        <input placeholder="Cost..." bind:value={new_rating.cost} />
        <button on:click={insert}>Insert</button>
    </div>
    <div class="m-5">
        <h1>Get Rating</h1>
        <input placeholder="Rating Name..." bind:value={rating_key} />
        <button on:click={get}>Get</button>
        <p>{response}</p>
    </div>
</div>
