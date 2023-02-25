<script lang="ts">
  import { createEventDispatcher } from 'svelte'

  const dispatch = createEventDispatcher()
  const modalClose = () => dispatch('modalClose')
  const modalSubmit = () => dispatch('modalSubmit')
</script>

<div
  on:click={modalClose}
  on:keydown={modalClose}
  class="fixed w-screen h-screen bg-black bg-opacity-40"
/>
<div
  id="deleteItem"
  class="flex fixed top-0 right-0 bottom-0 left-0 flex-col content-center items-center m-auto bg-white rounded-3xl border-2 border-black border-dashed h-[200px] w-[400px]"
>
  <div class="flex flex-col items-center m-auto gap-6">
    <p>Are you sure you want to delete this item?</p>
    <div class="flex gap-1">
      <button
        on:click={modalClose}
        class="border-2 border-black hover:bg-gray-200 ">cancel</button
      >
      <button
        on:click={modalSubmit}
        class=" text-white bg-black hover:bg-gray-800">yes, delete</button
      >
    </div>
  </div>
</div>

<style lang="postcss">
  button {
    @apply px-4 rounded-full transition-all;
  }
</style>
