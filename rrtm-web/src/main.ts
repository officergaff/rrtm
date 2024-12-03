import "./style.css";
import init, { initialize, hello, Scene } from "../../pkg/rrtm";

const html = `
    <div>
        <h1>wow</h1>
        <canvas id="canvas"></canvas>
        <p>yo</p>
    </div>
`;

document.querySelector<HTMLDivElement>("#app")!.innerHTML = html;

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d");

// init wasm
await init();
await initialize(1, true);

console.log(hello());
const width = 4 * 100;
const scene = Scene.new(width, 16.0 / 9.0);
console.log("starting render");
console.log(scene.render());
console.log("render ended");
canvas.width = scene.image_width();
canvas.height = scene.image_height();

const raw = scene.get_image();
const imageData = new ImageData(raw, width, scene.image_height());
ctx?.putImageData(imageData, 0, 0);
console.log(imageData);
