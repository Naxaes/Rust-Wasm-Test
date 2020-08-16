const rust = import('./pkg')

rust.then(m => {
    const FPS = 10.0;
    const UPDATE_TIME_MS = 1000.0 / FPS;
    let lastRenderTime = -1;

    const client = new m.Client();

    function render() {
        window.requestAnimationFrame(render)
        const currentTime = Date.now();

        if (currentTime >= lastRenderTime + UPDATE_TIME_MS) {
            let elapsedTime = currentTime - lastRenderTime;
            lastRenderTime = currentTime;
            client.update(elapsedTime, window.innerHeight, window.innerWidth);
            client.render();
        }
    }

    render();
})



