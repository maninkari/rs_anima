import init, * as wasm from "./pkg/rust_anima.js";
import mirrorwasm from "./pkg/rust_anima_bg.wasm";

// Lissajous parameters
const A = 2.0;
const B = 7.0;
const R = 5.0;

// Tunnel parameters
const polygon_radius = 1.0;
const polygon_sides = 7;
const num_polygons = 200;

// Create simple controls
const container = document.createElement("div");
container.style.padding = "20px";

const canvas = document.createElement("canvas");
canvas.id = "canvas";
canvas.width = 1120;
canvas.height = 630;
canvas.style.border = "1px solid #333";
canvas.style.display = "block";
canvas.style.marginBottom = "20px";

const controls = document.createElement("div");
controls.style.display = "flex";
controls.style.gap = "20px";
controls.style.alignItems = "center";

// Speed control
const speedLabel = document.createElement("label");
speedLabel.textContent = "Speed: ";
const speedSlider = document.createElement("input");
speedSlider.type = "range";
speedSlider.min = "-0.5";
speedSlider.max = "0.5";
speedSlider.step = "0.01";
speedSlider.value = "0.1";
speedSlider.addEventListener("input", (e) => {
  wasm.set_speed(parseFloat(e.target.value));
});

// Show/hide toggles
const longitudeCheck = document.createElement("input");
longitudeCheck.type = "checkbox";
longitudeCheck.checked = true;
longitudeCheck.addEventListener("change", (e) => {
  wasm.set_show_longitude(e.target.checked);
});
const longitudeLabel = document.createElement("label");
longitudeLabel.textContent = "Longitude";
longitudeLabel.prepend(longitudeCheck);

const latitudeCheck = document.createElement("input");
latitudeCheck.type = "checkbox";
latitudeCheck.checked = true;
latitudeCheck.addEventListener("change", (e) => {
  wasm.set_show_latitude(e.target.checked);
});
const latitudeLabel = document.createElement("label");
latitudeLabel.textContent = "Latitude";
latitudeLabel.prepend(latitudeCheck);

const tunnelCheck = document.createElement("input");
tunnelCheck.type = "checkbox";
tunnelCheck.checked = true;
tunnelCheck.addEventListener("change", (e) => {
  wasm.set_show_tunnel(e.target.checked);
});
const tunnelLabel = document.createElement("label");
tunnelLabel.textContent = "Tunnel";
tunnelLabel.prepend(tunnelCheck);

// Number of polygons slider
const polygonsLabel = document.createElement("label");
polygonsLabel.textContent = "Polygons: ";
const polygonsSlider = document.createElement("input");
polygonsSlider.type = "range";
polygonsSlider.min = "10";
polygonsSlider.max = "500";
polygonsSlider.step = "10";
polygonsSlider.value = num_polygons.toString();
const polygonsDisplay = document.createElement("span");
polygonsDisplay.textContent = num_polygons.toString();
polygonsSlider.addEventListener("input", (e) => {
  const value = parseInt(e.target.value);
  polygonsDisplay.textContent = value.toString();
  wasm.set_num_polygons(value);
});

// Outside view toggle
const outsideCheck = document.createElement("input");
outsideCheck.type = "checkbox";
outsideCheck.checked = false;
outsideCheck.addEventListener("change", (e) => {
  wasm.set_outside_view(e.target.checked);
});
const outsideLabel = document.createElement("label");
outsideLabel.textContent = "Outside View";
outsideLabel.prepend(outsideCheck);

controls.appendChild(speedLabel);
controls.appendChild(speedSlider);
controls.appendChild(polygonsLabel);
controls.appendChild(polygonsSlider);
controls.appendChild(polygonsDisplay);
controls.appendChild(longitudeLabel);
controls.appendChild(latitudeLabel);
controls.appendChild(tunnelLabel);
controls.appendChild(outsideLabel);

container.appendChild(canvas);
container.appendChild(controls);
document.body.appendChild(container);

// Initialize
(async () => {
  await init(mirrorwasm);

  wasm.start_simple_tunnel(
    "canvas",
    A,
    B,
    R,
    polygon_radius,
    polygon_sides,
    num_polygons
  );

  // Add keyboard controls
  let currentSpeed = 0.1; // Track current speed

  document.addEventListener("keydown", (event) => {
    switch (event.code) {
      case "ArrowLeft":
        // Decrease speed
        currentSpeed = Math.max(-0.5, currentSpeed - 0.005);
        wasm.set_speed(currentSpeed);
        speedSlider.value = currentSpeed.toString();
        event.preventDefault();
        break;

      case "ArrowRight":
        // Increase speed
        currentSpeed = Math.min(0.5, currentSpeed + 0.005);
        wasm.set_speed(currentSpeed);
        speedSlider.value = currentSpeed.toString();
        event.preventDefault();
        break;

      case "Space":
        // Stop/pause (set speed to 0)
        currentSpeed = 0.0;
        wasm.set_speed(currentSpeed);
        speedSlider.value = currentSpeed.toString();
        event.preventDefault();
        break;
    }
  });
})();
