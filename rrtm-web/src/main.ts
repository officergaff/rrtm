import "./style.css";

const html = `
<div>
	<h1>wow</h1>
	<p>yo</p>
</div>
`;

console.log("wow");
document.querySelector<HTMLDivElement>("#app")!.innerHTML = html;
