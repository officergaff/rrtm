import "./style.css";
import init, { initialize, hello, Scene } from "../../pkg/rrtm";

const html = `
    <div>
        <h1>wow</h1>
        <canvas id="canvas"></canvas>
        <p id="fps">fps</p>
    </div>
`;

document.querySelector<HTMLDivElement>("#app")!.innerHTML = html;

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
canvas.style.border = "1px solid red";
const ctx = canvas.getContext("2d");
const fpsCounter = document.getElementById("fps") as HTMLElement;

// init wasm
await init();
await initialize(16, true);

const width = 10 * 100;
const aspectRatio = 16.0 / 9.0;
const samplesPerPixel = 10;
const maxBounces = 12;
const scene = Scene.new(width, 16.0 / 9.0, samplesPerPixel, maxBounces);

console.log(scene.to_obj());
canvas.width = scene.image_width();
canvas.height = scene.image_height();
console.log("canvas width: " + scene.image_width());
console.log("canvas height: " + scene.image_height());

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function renderLoop(scene: Scene) {
  let totalTime = 0.0;
  while (true) {
    if (scene.current_samples() < samplesPerPixel) {
      console.log(scene.current_samples());
      const start = performance.now();
      scene.render();
      const elapsed = performance.now() - start;
      totalTime += elapsed;

      const fps = scene.current_samples() / (totalTime / 1000);
      fpsCounter.innerText = `${fps.toFixed(2)} fps`;

      const raw = scene.get_image();
      const imageData = new ImageData(raw, width);
      console.log(imageData);
      ctx?.putImageData(imageData, 0, 0);
    } else {
      await sleep(10000);
    }
  }
}

renderLoop(scene);
