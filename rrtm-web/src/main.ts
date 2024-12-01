import "./style.css";
import init, { hello } from "../../pkg/rrtm";

const html = `
    <div>
        <h1>wow</h1>
        <p>yo</p>
    </div>
    `;
await init();
console.log("wow");
console.log(hello());
document.querySelector<HTMLDivElement>("#app")!.innerHTML = html;
