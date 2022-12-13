import {Canvas} from "wgpu-wasm-demo";

Canvas.new().then(canvas => {
    console.log("new a canvas");
    const render = () => {
        setTimeout(() => {
            canvas.run(Math.random() - 0.5, Math.random() - 0.5);
            render()
        }, 16);
    }
    render();
});

