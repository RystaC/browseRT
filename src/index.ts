import('./wasm/main').then(module => {
    const { noise_image } = module

    const canvas = document.getElementById('raytrace')
    if(!(canvas instanceof HTMLCanvasElement)) throw new Error('#app is not canvas element')

    const ctx = canvas.getContext('2d')
    if(!ctx) throw new Error('cannot get context from canvas')

    const { width, height } = canvas

    let imageData = ctx.createImageData(width, height)

    const data = noise_image(width, height)
    console.log(data)

    let idx = 0;

    for(let i = 0; i < imageData.width * imageData.height * 4;) {
        imageData.data[i++] = data[idx++]
        imageData.data[i++] = data[idx++]
        imageData.data[i++] = data[idx++]
        imageData.data[i++] = 255
    }

    ctx.putImageData(imageData, 0, 0)
})