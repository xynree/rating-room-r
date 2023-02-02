<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let items: Item[] = [];
    async function get_items() {
        // TODO: figure out what type to assign for items that allows a catch returning an error
        items = await invoke("get_items", {
            filter: {
                rating: ["3", "4", "5"],
                category: [],
            },
        }).catch((error) => {
            console.error(error);
        });
    }

    let item_id: number = 0;
    let item: Item | null = null;
    async function get_item() {
        item = await invoke("get_item", { id: item_id });
    }

    // run on load?
    get_items();
</script>

<div class="border m-3 p-3">
    <div class="m-3 flex flex-col max-w-sm items-start">
        <h1 class="font-bold">API Testing Demo</h1>
        <input type="number" id="item_id" bind:value={item_id} />
        <button class=" p-1 font-bold" on:click={get_item}>Get Item</button>
        {#if item != null}
            <h1>My Item</h1>
            <p>{item.name} - {item.description} - {item.date}</p>
        {/if}
        <!-- example usage-->
        {#if items.length == 0}
            <p>No Items</p>
        {:else}
            <h2 class="text-xl font-bold">Items</h2>
            {#each items as item}
                <p>{item.name} - {item.description} - {item.date}</p>
            {/each}
        {/if}
    </div>
</div>
