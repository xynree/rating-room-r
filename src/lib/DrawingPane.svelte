<script lang="ts">
  import { DRAW_ICON, ERASE_ICON, ToolType } from 'service/canvas'
  import { saveFile, saveNewImage } from 'service/file'
  import { onMount } from 'svelte'

  $: toolType = ToolType.draw
  let isDrawing = false

  let canvas: HTMLCanvasElement
  let ctx: CanvasRenderingContext2D | null
  let coords: { x: number; y: number }

  onMount(() => {
    canvas = document.getElementById('canvas') as HTMLCanvasElement
    ctx = canvas.getContext('2d')
    canvas.getBoundingClientRect().left
    coords = {
      x: canvas.getBoundingClientRect().left,
      y: canvas.getBoundingClientRect().top,
    }
    console.log(coords)
  })

  async function saveImage() {
    return canvas.toBlob(async (b) => {
      if (b) {
        let filename = await saveNewImage(b)
        console.log(filename)
        return filename
      }
    })
  }

  function drawCircle(x: number, y: number) {
    console.log('draw circle run')
    if (ctx) {
      if (toolType == ToolType.erase) {
        ctx.arc(x - coords.x, y - coords.y, 10, 0, Math.PI * 2, true)

        ctx.strokeStyle = '#fff'
        ctx.fillStyle = '#fff'
      } else {
        ctx.arc(x - coords.x, y - coords.y, 3, 0, Math.PI * 2, true)

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
  }
</script>

<div class="flex" id="drawing">
  <button
    class:!bg-yellow-400={toolType === ToolType.draw}
    on:click={() => (isDrawing = true)}>{DRAW_ICON}</button
  >
  <button
    class:!bg-yellow-400={toolType === ToolType.erase}
    on:click={() => (toolType = ToolType.erase)}>{ERASE_ICON}</button
  >
</div>
<canvas
  id="canvas"
  width="300"
  height="300"
  class="hover:cursor-pointer border"
  on:mousedown={mouseDown}
  on:mousemove={handleDrawing}
  on:mouseup={mouseUp}
  on:blur={mouseUp}
/>
<button class="outline" on:click={saveImage}>Save Image</button>

<style lang="postcss">
  #drawing > button {
    @apply px-4 transition-all bg-gray-100 hover:bg-gray-200 mx-2 rounded-full;
  }
</style>
