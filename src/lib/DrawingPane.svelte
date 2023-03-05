<script lang="ts">
  import { DELETE_ICON, DRAW_ICON, ERASE_ICON, ToolType } from 'service/canvas'
  import { createEventDispatcher, onMount } from 'svelte'

  const dispatch = createEventDispatcher()
  let isDrawing = false
  let canvas: HTMLCanvasElement
  let ctx: CanvasRenderingContext2D | null
  let coords: { x: number; y: number }

  $: toolType = ToolType.draw

  onMount(() => {
    canvas = document.getElementById('canvas') as HTMLCanvasElement
    ctx = canvas.getContext('2d')
    canvas.getBoundingClientRect().left
    coords = {
      x: canvas.getBoundingClientRect().left,
      y: canvas.getBoundingClientRect().top,
    }
    if (ctx) {
      ctx.fillStyle = '#fff'
      ctx.fillRect(0, 0, canvas.width, canvas.height)
    }
  })

  function loadOldImage() {}

  function drawCircle(x: number, y: number) {
    if (ctx) {
      if (toolType == ToolType.erase) {
        ctx.arc(x - coords.x, y - coords.y, 10, 0, Math.PI * 2, true)

        ctx.strokeStyle = '#fff'
        ctx.fillStyle = '#fff'
      } else {
        ctx.arc(x - coords.x, y - coords.y, 3, 0, Math.PI * 2, true)
        ctx.strokeStyle = '#000'
        ctx.fillStyle = '#000'
      }
      ctx.stroke()
      ctx.fill()
    }
  }

  function mouseDown() {
    if (!isDrawing) {
      isDrawing = true
    }
  }

  function handleDrawing(e) {
    if (isDrawing) {
      ctx?.beginPath()
      drawCircle(e.clientX, e.clientY)
    }
  }

  function mouseUp() {
    ctx?.closePath()
    isDrawing = false

    canvas.toBlob((b) => dispatch('updateBlob', b))
  }

  function clearCanvas() {
    if (ctx) ctx.clearRect(0, 0, canvas.width, canvas.height)
    dispatch('updateBlob', null)
  }
</script>

<div class="flex items-center gap-1" id="drawing">
  <button
    class:!bg-yellow-400={toolType === ToolType.draw}
    on:click={() => (toolType = ToolType.draw)}>{DRAW_ICON}</button
  >
  <button
    class:!bg-yellow-400={toolType === ToolType.erase}
    on:click={() => (toolType = ToolType.erase)}>{ERASE_ICON}</button
  >
  <button on:click={clearCanvas}>{DELETE_ICON}</button>
</div>
<canvas
  id="canvas"
  width="500"
  height="500"
  class="hover:cursor-pointer border"
  on:mousedown={mouseDown}
  on:mousemove={handleDrawing}
  on:mouseup={mouseUp}
  on:blur={mouseUp}
/>

<style lang="postcss">
  #drawing > button {
    @apply px-4 transition-all bg-gray-100 hover:bg-gray-200 rounded-sm border;
  }

  #drawing > button:last-child {
    @apply hover:bg-black hover:text-white ml-auto;
  }
</style>
