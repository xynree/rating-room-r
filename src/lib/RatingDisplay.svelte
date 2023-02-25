<script lang="ts">
  export let ratings: Rating[]
  let currentRatingIdx: number = 0

  let prevRating = () =>  
     currentRatingIdx = currentRatingIdx > 0
      ? (currentRatingIdx - 1)
      : currentRatingIdx
  
  let nextRating = () =>
    currentRatingIdx = currentRatingIdx < ratings.length - 1
      ? ( currentRatingIdx + 1)
      : currentRatingIdx
</script>

<p class="tag">rating {currentRatingIdx + 1} of {ratings.length}</p>
<div class="flex text-slate-600">
  <button on:click={prevRating} class="badge">back</button>
  {#each Array(ratings[currentRatingIdx].rating) as _}
    <p>★</p>
  {/each}
  <button on:click={nextRating} class="badge">next</button>
</div>
<p class="text-sm">
  {ratings && new Date(ratings[currentRatingIdx].date + 'Z').toLocaleString()}
</p>
