<script>
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import { invoke } from "@tauri-apps/api/tauri";

    const id = $page.params.id;
    console.log(id);
    let showModal = false;

    async function deleteItem() {
        await invoke("delete_item", { id: Number(id) });
        goto("/");
    }
    function toggleModal() {
        showModal = !showModal


        }

</script>

<nav class="flex justify-between p-6 w-screen text-sm font-medium shadow-sm">
    <a href="/">my collection</a>
    <div class="flex gap-5">
        <button on:click={toggleModal} class="underline">delete item</button>
        <a href={`/items/${id}/edit`} class="underline">+ edit item</a>
    </div>
</nav>
{#if showModal}
<div on:click={toggleModal} class="fixed w-screen h-screen bg-black bg-opacity-20"></div>
    <div
        id="deleteItem"
        class="flex fixed top-0 right-0 bottom-0 left-0 flex-col content-center items-center m-auto bg-white rounded-3xl border border-black border-dashed h-[50vh] w-[50vw]"
    >
        <div class="flex flex-col items-center m-auto">
            <p>Are you sure you want to delete item?</p>
            <div class="flex gap-1">
                <button
                    on:click={toggleModal}
                    class="px-3 rounded-full border-2 border-black"
                    >cancel</button
                ><button
                    on:click={deleteItem}
                    class="px-3 text-white bg-black rounded-full"
                    >yes, delete</button
                >
            </div>
        </div>
    </div>
{/if}

<slot />
<style lang="postcss">
    a {
        @apply hover:text-gray-800 transition-all text-sm;
    }
</style>
