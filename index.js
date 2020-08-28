const rust = import('./pkg')

function getUpdateTime() {
    const FPS = document.getElementById("fps").value;
    return 1000.0 / FPS;
}


rust.then(m => {
    let lastRenderTime = Date.now();

    const client = new m.Client();

    function render() {
        window.requestAnimationFrame(render)
        const currentTime = Date.now();
        const updateTime = getUpdateTime();

        if (currentTime >= lastRenderTime + updateTime) {
            let elapsedTime = currentTime - lastRenderTime;
            lastRenderTime = currentTime;
            client.update(elapsedTime, window.innerHeight, window.innerWidth);
            client.render();
        }
    }

    render();
})



