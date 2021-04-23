import('./wasm/main').then(module => {
    const { noise_image } = module

    const canvas = document.getElementById('raytrace')
    if(!(canvas instanceof HTMLCanvasElement)) throw new Error('#app is not canvas element')

    const ctx = canvas.getContext('2d')
    if(!ctx) throw new Error('cannot get context from canvas')

    const { width, height } = canvas

    let imageData = ctx.createImageData(width, height)

    imageData.data.set(noise_image(width, height))

    ctx.putImageData(imageData, 0, 0)
})