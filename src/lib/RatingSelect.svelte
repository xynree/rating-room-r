<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  const dispatch = createEventDispatcher()
  export let defaultRating: number
  function handleChange(num: number) {
    dispatch('rating', {
      rating: num + 1,
    })
  }
  function fillStar(e) {
    console.log('hovered')
  }
</script>

<div>
  <p class="font-bold">rating</p>
  {#each Array(5) as _, i}
    <button on:click={() => handleChange(i)} on:mouseover={fillStar}>
      {defaultRating > i ? '★' : '☆'}
    </button>
  {/each}
</div>
