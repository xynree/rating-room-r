<script lang="ts">
  import { DELETE_ICON, DRAW_ICON, ERASE_ICON, ToolType } from 'service/canvas'
  import { createEventDispatcher, onMount } from 'svelte'

  const dispatch = createEventDispatcher()
  let isDrawing = false
  export let imgUrl: string
  let canvas: HTMLCanvasElement
  let ctx: CanvasRenderingContext2D | null
  $: toolType = ToolType.draw

  let backgroundImg: string = ''
  $: if (imgUrl != backgroundImg) {
    let img = new Image()
    img.crossOrigin = 'anonymous'
    img.onload = function () {
      if (ctx) {
        ctx.drawImage(img, 0, 0, 480, 480)
      }
    }
    img.src = imgUrl
    backgroundImg = imgUrl
  }

  onMount(() => {
    canvas = document.getElementById('canvas') as HTMLCanvasElement
    ctx = canvas.getContext('2d')
    canvas.getBoundingClientRect().left
    if (ctx) {
      ctx.fillStyle = '#fff'
      ctx.fillRect(0, 0, canvas.width, canvas.height)
    }
  })

  function findPixel(x: number, y: number) {
    let start = { x: 0, y: 0 }
    if (x % 4 == 0) {
      start.x = x
    } else {
      start.x = x - (x % 4)
    }
    if (y % 4 == 0) {
      start.y = y
    } else {
      start.y = y - (y % 4)
    }
    return start
  }

  function getMousePos(canvas, evt) {
    var rect = canvas.getBoundingClientRect()
    return {
      x: Math.floor(
        ((evt.clientX - rect.left) / (rect.right - rect.left)) * canvas.width
      ),
      y: Math.floor(
        ((evt.clientY - rect.top) / (rect.bottom - rect.top)) * canvas.height
      ),
    }
  }

  function drawPixel(x: number, y: number) {
    if (ctx) {
      if (toolType == ToolType.erase) {
        let pos = findPixel(x, y)
        ctx.fillRect(pos.x, pos.y, 4, 4)
        ctx.strokeStyle = '#fff'
        ctx.fillStyle = '#fff'
      } else {
        let pos = findPixel(x, y)
        ctx.fillRect(pos.x, pos.y, 4, 4)
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
      let { x, y } = getMousePos(canvas, e)
      drawPixel(x, y)
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
  width="480"
  height="480"
  style="image-rendering:pixelated"
  class="hover:cursor-pointer "
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
