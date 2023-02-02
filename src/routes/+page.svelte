<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let items: Item[] = [];
    async function get_items() {
        // TODO: figure out what type to assign for items that allows a catch returning an error
        items = await invoke("get_items", {
            filter: {
                category: ["Drinks"],
                rating: ["2"],
            },
        }).catch((error) => {
            console.error(error);
        });
    }

    // run on load?
    get_items();
</script>

<div class="border m-3 p-3">
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
