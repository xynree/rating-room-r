<script>
  import { goto } from '$app/navigation'
  import { page } from '$app/stores'
  import Modal from '$lib/Modal.svelte'
  import { deleteItem } from 'service/db'
  import { itemsStore } from 'store'

  $: id = Number($page.params.id)
  let showModal = false

  async function delItem() {
    await deleteItem(id)
    goto('/')
  }

  function toggleModal() {
    showModal = !showModal
  }
</script>

<nav class="flex justify-between p-6 w-screen text-sm">
  <a href="/">my collection ({$itemsStore.items.length} items)</a>
  <div class="flex gap-5">
    <button on:click={toggleModal}>delete item</button>
    <a href={`/items/${id}/edit`}>+ edit item</a>
  </div>
</nav>
{#if showModal}
  <Modal on:modalClose={toggleModal} on:modalSubmit={delItem} />
{/if}

<slot />

<style lang="postcss">
  a {
    @apply hover:text-gray-800 transition-all text-sm hover:underline;
  }
</style>
