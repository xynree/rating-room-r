<script>
  import { goto } from '$app/navigation'
  import { page } from '$app/stores'
  import Modal from '$lib/Modal.svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  import { itemsStore } from 'store'

  $: id = Number($page.params.id)
  let showModal = false

  async function deleteItem() {
    await invoke('delete_item', { id })
    goto('/')
  }

  function toggleModal() {
    showModal = !showModal
  }
</script>

<nav
  class="flex justify-between h-24 items-center px-12 w-screen text-sm font-medium"
>
  <a href="/">my collection ({$itemsStore.length} items)</a>
  <div class="flex gap-5">
    <button on:click={toggleModal} class="underline">delete item</button>
    <a href={`/items/${id}/edit`} class="underline">+ edit item</a>
  </div>
</nav>
{#if showModal}
  <Modal on:modalClose={toggleModal} on:modalSubmit={deleteItem} />
{/if}

<slot />

<style lang="postcss">
  a {
    @apply hover:text-gray-800 transition-all text-sm;
  }
</style>
