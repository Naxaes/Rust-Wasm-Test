const rust = import('./pkg')

const canvas = document.getElementById('canvas');
const gl = canvas.getContext('webgl', { antialias: true });

rust.then(m => {
    if (!gl) {
        alert('Failed to initialize WebGL.');
        return;
    }

    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

    const FPS_THROTTLE = 1000.0 / 30.0  //
    var initialTime = Date.now();
    var lastRenderTime = -1;

    var client = new m.Client();

    function render() {
        window.requestAnimationFrame(render)
        const currentTime = Date.now();

        if (currentTime >= lastRenderTime + FPS_THROTTLE) {
            lastRenderTime = currentTime;

            if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
                canvas.height = window.innerHeight;
                canvas.clientHeight = window.innerHeight;
                canvas.style.height = window.innerHeight;

                canvas.width = window.innerWidth;
                canvas.clientWidth = window.innerWidth;
                canvas.style.width = window.innerWidth;

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

            let elapsedTime = currentTime - initialTime;
            client.update(elapsedTime, window.innerHeight, window.innerWidth);
            client.render();
        }

    }

    render();
})



