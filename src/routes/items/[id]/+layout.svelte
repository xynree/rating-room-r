<script>
  import { goto } from '$app/navigation'
  import { page } from '$app/stores'
  import Modal from '$lib/Modal.svelte'
  import { invoke } from '@tauri-apps/api/tauri'

  const id = $page.params.id
  let showModal = false

  async function deleteItem() {
    await invoke('delete_item', { id: Number(id) })
    goto('/')
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
  <Modal {toggleModal} {deleteItem} />
{/if}

<slot />

<style lang="postcss">
  a {
    @apply hover:text-gray-800 transition-all text-sm;
  }
</style>
