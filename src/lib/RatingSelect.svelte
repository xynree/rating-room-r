<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  const dispatch = createEventDispatcher()
  export let defaultRating: number = 0
  export let adjustable = true
  function handleChange() {
    adjustable = !adjustable
    if (!adjustable) {
      dispatch('rating', {
        rating: defaultRating,
      })
    }
  }
  function fillStar(i: number) {
    if (adjustable) {
      defaultRating = i + 1
    }
  }
</script>

<div>
  <p class="font-bold">rating</p>
  {#each Array(5) as _, i}
    <button
      on:click={() => handleChange(i)}
      on:mouseover={() => fillStar(i)}
      on:focus={() => fillStar(i)}
      class:text-yellow-400={defaultRating > i}
    >
      ★
    </button>
  {/each}
</div>
