import {Runtime} from "wgpu-wasm-demo";


Runtime.new(document.getElementById("wasm-example")).then(canvas => {
    console.log("new a canvas");
    canvas.render();
});

