import "./style.css";
import init, { initialize, Scene } from "../../pkg/rrtm";

// Camera initialization
const width = 7 * 100;
const aspectRatio = 16.0 / 9.0;
const vfov = 20.0;
const samplesPerPixel = 110;
const maxBounces = 30;
const lookfrom = [13, 2, 3];
const lookat = [0, 0, 0];
const defaultSettings = {
  width: width,
  aspectRatio: aspectRatio,
  vfov: vfov,
  lookfrom: lookfrom,
  lookat: lookat,
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
  },
  null,
  2
);

// html elements
const canvas = document.getElementById("canvas") as HTMLCanvasElement;
canvas.style.border = "1px solid red";
const ctx = canvas.getContext("2d");
const fpsCounter = document.getElementById("fps") as HTMLElement;
const threadsCounter = document.getElementById("threads") as HTMLElement;
const computeCounter = document.getElementById("computes") as HTMLElement;
// dimensions display
const dimensions = document.getElementById("dimensions") as HTMLElement;
// fov controls
const fovSlider = document.getElementById("fov") as HTMLInputElement;
const fovDisplay = document.getElementById("fov-value") as HTMLElement;
fovDisplay.innerHTML = `fov: ${vfov}`;
fovSlider.value = vfov.toString();
// camera position controls
const xSlider = document.getElementById("x-slide") as HTMLInputElement;
const xDisplay = document.getElementById("x-value") as HTMLElement;
xDisplay.innerHTML = `x: ${lookfrom[0]}`;
xSlider.value = lookfrom[0].toString();

const ySlider = document.getElementById("y-slide") as HTMLInputElement;
const yDisplay = document.getElementById("y-value") as HTMLElement;
yDisplay.innerHTML = `y: ${lookfrom[1]}`;
ySlider.value = lookfrom[1].toString();

const zSlider = document.getElementById("z-slide") as HTMLInputElement;
const zDisplay = document.getElementById("z-value") as HTMLElement;
zDisplay.innerHTML = `z: ${lookfrom[2]}`;
zSlider.value = lookfrom[2].toString();

// init wasm
const maxThreads = navigator.hardwareConcurrency;
threadsCounter.innerText = `${maxThreads} threads`;
await init();
await initialize(maxThreads, true);

const scene = Scene.new(width, aspectRatio, samplesPerPixel, maxBounces);

let totalTime = 0.0;

canvas.width = scene.image_width();
canvas.height = scene.image_height();

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
fovSlider.oninput = () => {
  fovDisplay.innerHTML = `fov: ${fovSlider.value}`;
  const settings = JSON.parse(settingsTextarea.value);
  const updatedSettings = { ...settings, vfov: Number(fovSlider.value) };
  settingsTextarea.value = JSON.stringify(updatedSettings, null, 2);
  scene.update_camera(updatedSettings);
  totalTime = 0.0;
};
xSlider.oninput = () => {
  xSlider.innerHTML = `x: ${xSlider.value}`;
  const numVal = Number(xSlider.value);
  lookfrom[0] = numVal;
  const settings = JSON.parse(settingsTextarea.value);
  const updatedSettings = { ...settings, lookfrom: lookfrom };
  settingsTextarea.value = JSON.stringify(updatedSettings, null, 2);
  scene.update_camera(updatedSettings);
  totalTime = 0.0;
};
ySlider.oninput = () => {
  ySlider.innerHTML = `y: ${ySlider.value}`;
  const numVal = Number(ySlider.value);
  lookfrom[1] = numVal;
  const settings = JSON.parse(settingsTextarea.value);
  const updatedSettings = { ...settings, lookfrom: lookfrom };
  settingsTextarea.value = JSON.stringify(updatedSettings, null, 2);
  scene.update_camera(updatedSettings);
  totalTime = 0.0;
};
zSlider.oninput = () => {
  zSlider.innerHTML = `z: ${zSlider.value}`;
  const numVal = Number(zSlider.value);
  lookfrom[2] = numVal;
  const settings = JSON.parse(settingsTextarea.value);
  const updatedSettings = { ...settings, lookfrom: lookfrom };
  settingsTextarea.value = JSON.stringify(updatedSettings, null, 2);
  scene.update_camera(updatedSettings);
  totalTime = 0.0;
};
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
