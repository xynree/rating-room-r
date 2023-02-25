<script lang="ts">
    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();
    export let rating: number;
    function handleChange(e) {
        dispatch("rating", {
            rating: Number(e.target.value) + 1,
        });
    }
</script>

<div>
    <p class="tag">rating</p>
    <select class="flex text-slate-600" on:change={handleChange}>
        {#each Array(5) as _, i}
            <option value={i} selected={rating === i + 1}>
                {#each Array(i + 1) as _}★{/each}
            </option>
        {/each}
    </select>
</div>
