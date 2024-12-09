import "./style.css";
import init, { initialize, Scene } from "../../pkg/rrtm";

const html = `
    <div class="container">
        <div class="render-section">
            <h1>Wow</h1>
            <p id="dimensions">dimensions</p>
            <canvas id="canvas"></canvas>
            <p id="fps">fps</p>
            <p id="computes">computers</p>
            <p id="threads">threads</p>
        </div>
        <div class="controls-section">
            <h2>Camera Settings</h2>
            <textarea id="cameraSettings" rows="20" cols="50" style="font-family: monospace;"></textarea>
            <button id="updateCamera">Update Camera</button>
        </div>
    </div>
`;
document.querySelector<HTMLDivElement>("#app")!.innerHTML = html;

const defaultSettings = {
  width: 1000,
  aspectRatio: 16.0 / 9.0,
  vfov: 20.0,
  lookfrom: [13, 2, 3],
  lookat: [0, 0, 0],
  vup: [0, 1, 0],
  defocusAngle: 0.0,
  focus_dist: 12.0,
};
const updateButton = document.getElementById(
  "updateCamera"
) as HTMLButtonElement;
const settingsTextarea = document.getElementById(
  "cameraSettings"
) as HTMLTextAreaElement;
settingsTextarea.value = JSON.stringify(
  {
    ...defaultSettings,
    aspectRatio: 16.0 / 9.0,
  },
  null,
  2
);

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
canvas.style.border = "1px solid red";
const ctx = canvas.getContext("2d");
const fpsCounter = document.getElementById("fps") as HTMLElement;
const threadsCounter = document.getElementById("threads") as HTMLElement;
const computeCounter = document.getElementById("computes") as HTMLElement;
const dimensions = document.getElementById("dimensions") as HTMLElement;

const maxThreads = navigator.hardwareConcurrency;
threadsCounter.innerText = `${maxThreads} threads`;
// init wasm
await init();
await initialize(maxThreads, true);

const width = 10 * 100;
const aspectRatio = 16.0 / 9.0;
const samplesPerPixel = 10;
const maxBounces = 12;
const scene = Scene.new(width, aspectRatio, samplesPerPixel, maxBounces);

let totalTime = 0.0;
updateButton.addEventListener("click", () => {
  try {
    console.log("Current settings text:", settingsTextarea.value);
    const settings = JSON.parse(settingsTextarea.value);
    scene.update_camera(settings);
    canvas.width = scene.image_width();
    canvas.height = scene.image_height();

    // Reset total time for FPS calculation
    totalTime = 0.0;
  } catch (e) {
    console.error("Invalid JSON:", e);
    alert("Invalid JSON settings");
  }
});

console.log(scene.to_obj());
canvas.width = scene.image_width();
canvas.height = scene.image_height();
console.log("canvas width: " + scene.image_width());
console.log("canvas height: " + scene.image_height());

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function renderLoop(scene: Scene) {
  while (true) {
    if (scene.current_samples() < samplesPerPixel) {
      dimensions.innerHTML = `${scene.image_width()}px * ${scene.image_height()}px`;
      const start = performance.now();
      scene.render();
      const elapsed = performance.now() - start;
      totalTime += elapsed;

      const fps = scene.current_samples() / (totalTime / 1000);
      fpsCounter.innerText = `${fps.toFixed(2)} fps`;
      const computePerFrame = scene.image_width() * scene.image_height() * fps;
      computeCounter.innerText = `${Math.round(
        computePerFrame
      ).toLocaleString()} computations per second`;

      const raw = scene.get_image();
      const imageData = new ImageData(raw, scene.image_width());
      ctx?.putImageData(imageData, 0, 0);
      await new Promise((resolve) => setTimeout(resolve, 0));
    } else {
      await sleep(1000);
    }
  }
}

renderLoop(scene);
