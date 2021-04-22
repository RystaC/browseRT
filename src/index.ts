import('./wasm/main').then(module => {
    const { take_number } = module
    console.log(take_number())
})