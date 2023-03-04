<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  const dispatch = createEventDispatcher()
  export let defaultRating: number = 0
  export let clicked = false
  function handleChange(i) {
    if (!clicked) {
      clicked = true
    } else {
      defaultRating = i + 1
    }
    dispatch('rating', {
      rating: defaultRating,
    })
  }

  function fillStar(i: number) {
    if (!clicked) {
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
