import "./style.css";
import init, { hello, Scene } from "../../pkg/rrtm";

const html = `
    <div>
        <h1>wow</h1>
        <p>yo</p>
    </div>
    `;

// init wasm
await init();
console.log("wow");
console.log(hello());
const scene = Scene.new(100, 16.0 / 9.0, 1000);
const image = scene.get_image();
console.log(image);
document.querySelector<HTMLDivElement>("#app")!.innerHTML = html;
