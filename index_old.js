import init, * as wasm from "./pkg/rust_anima.js";
import mirrorwasm from "./pkg/rust_anima_bg.wasm";

const A = 1.0;
const B = 2.0;
const R = 5.0;
const n = 7;
const r = 1.0;

// Create main container
const mainContainer = document.createElement("div");
mainContainer.style.display = "flex";
mainContainer.style.alignItems = "flex-start";
mainContainer.style.gap = "20px";
mainContainer.style.marginBottom = "20px";

// Create content container for canvas
const contentContainer = document.createElement("div");

// Create speed control container
const speedContainer = document.createElement("div");
speedContainer.style.display = "flex";
speedContainer.style.flexDirection = "column";
speedContainer.style.alignItems = "center";
speedContainer.style.gap = "5px";

// Speed control state (range: -0.5 to 0.5)
let currentSpeed = 0.1;

// Create speed display
const speedDisplay = document.createElement("div");
speedDisplay.style.fontSize = "14px";
speedDisplay.style.fontWeight = "bold";
speedDisplay.style.marginBottom = "15px";
speedDisplay.style.textAlign = "center";
speedDisplay.textContent = `Speed: ${currentSpeed.toFixed(2)}`;

// Create camera t-value display
const tValueDisplay = document.createElement("div");
tValueDisplay.style.fontSize = "14px";
tValueDisplay.style.fontWeight = "bold";
tValueDisplay.style.marginBottom = "15px";
tValueDisplay.style.textAlign = "center";
tValueDisplay.style.color = "#ffd700";
tValueDisplay.textContent = `Camera t: 0.00`;

// Create speed slider
const speedSlider = document.createElement("input");
speedSlider.type = "range";
speedSlider.min = "-0.5";
speedSlider.max = "0.5";
speedSlider.step = "0.01";
speedSlider.value = currentSpeed.toString();
speedSlider.style.width = "120px";
speedSlider.style.height = "20px";
speedSlider.style.marginBottom = "10px";

// Style the slider
speedSlider.style.background =
  "linear-gradient(90deg, #ff6b6b 0%, #4ecdc4 50%, #45b7d1 100%)";
speedSlider.style.borderRadius = "10px";
speedSlider.style.outline = "none";
speedSlider.style.cursor = "pointer";

// Add event listener for slider changes
speedSlider.addEventListener("input", (event) => {
  currentSpeed = parseFloat(event.target.value);
  speedDisplay.textContent = `Speed: ${currentSpeed.toFixed(2)}`;
  wasm.update_speed(currentSpeed);
});

// Create slider container with labels
const sliderContainer = document.createElement("div");
sliderContainer.style.display = "flex";
sliderContainer.style.alignItems = "center";
sliderContainer.style.gap = "10px";
sliderContainer.style.marginBottom = "10px";

const reverseLabel = document.createElement("div");
reverseLabel.style.fontSize = "16px";
reverseLabel.style.color = "#ff6b6b";
reverseLabel.style.fontWeight = "bold";

const forwardLabel = document.createElement("div");
forwardLabel.style.fontSize = "16px";
forwardLabel.style.color = "#45b7d1";
forwardLabel.style.fontWeight = "bold";

// Add zero marker
const zeroLabel = document.createElement("div");
zeroLabel.textContent = "⦁";
zeroLabel.style.fontSize = "12px";
zeroLabel.style.color = "#4ecdc4";
zeroLabel.style.position = "absolute";
zeroLabel.style.transform = "translateX(-50%)";
zeroLabel.style.marginTop = "-25px";
zeroLabel.style.left = "50%";

// Create positioned container for slider with zero marker
const sliderWithMarker = document.createElement("div");
sliderWithMarker.style.position = "relative";
sliderWithMarker.style.display = "flex";
sliderWithMarker.style.alignItems = "center";
sliderWithMarker.appendChild(speedSlider);
sliderWithMarker.appendChild(zeroLabel);

// Assemble slider container
sliderContainer.appendChild(reverseLabel);
sliderContainer.appendChild(sliderWithMarker);
sliderContainer.appendChild(forwardLabel);

// Create perspective toggle button
const perspectiveButton = document.createElement("button");
perspectiveButton.textContent = "Outside View";
perspectiveButton.style.padding = "8px 16px";
perspectiveButton.style.marginTop = "15px";
perspectiveButton.style.backgroundColor = "#4ecdc4";
perspectiveButton.style.color = "white";
perspectiveButton.style.border = "none";
perspectiveButton.style.borderRadius = "5px";
perspectiveButton.style.cursor = "pointer";
perspectiveButton.style.fontSize = "14px";
perspectiveButton.style.fontWeight = "bold";

// Track current perspective state
let isInsideView = true;

perspectiveButton.addEventListener("click", () => {
  isInsideView = !isInsideView;
  perspectiveButton.textContent = isInsideView ? "Outside View" : "Inside View";
  perspectiveButton.style.backgroundColor = isInsideView
    ? "#4ecdc4"
    : "#45b7d1";
  wasm.toggle_perspective(!isInsideView);
});

// Create polygon count control
const polygonCountDisplay = document.createElement("div");
polygonCountDisplay.style.fontSize = "14px";
polygonCountDisplay.style.fontWeight = "bold";
polygonCountDisplay.style.marginTop = "15px";
polygonCountDisplay.style.textAlign = "center";
let currentPolygonCount = 500;
polygonCountDisplay.textContent = `Polygons: ${currentPolygonCount}`;

const polygonSlider = document.createElement("input");
polygonSlider.type = "range";
polygonSlider.min = "10";
polygonSlider.max = "1000";
polygonSlider.step = "10";
polygonSlider.value = currentPolygonCount.toString();
polygonSlider.style.width = "120px";
polygonSlider.style.height = "20px";
polygonSlider.style.marginTop = "5px";
polygonSlider.style.background =
  "linear-gradient(90deg, #ff9a9e 0%, #fecfef 50%, #fecfef 100%)";
polygonSlider.style.borderRadius = "10px";
polygonSlider.style.outline = "none";
polygonSlider.style.cursor = "pointer";

polygonSlider.addEventListener("input", (event) => {
  currentPolygonCount = parseInt(event.target.value);
  polygonCountDisplay.textContent = `Polygons: ${currentPolygonCount}`;
  wasm.set_num_polygons(currentPolygonCount);
  // Restart visualization with new polygon count
  wasm.restart_visualization("canvas");
});

// Create alpha control
const alphaDisplay = document.createElement("div");
alphaDisplay.style.fontSize = "14px";
alphaDisplay.style.fontWeight = "bold";
alphaDisplay.style.marginTop = "15px";
alphaDisplay.style.textAlign = "center";
let currentAlpha = 0.8;
alphaDisplay.textContent = `Alpha: ${currentAlpha.toFixed(2)}`;

const alphaSlider = document.createElement("input");
alphaSlider.type = "range";
alphaSlider.min = "0.0";
alphaSlider.max = "1.0";
alphaSlider.step = "0.05";
alphaSlider.value = currentAlpha.toString();
alphaSlider.style.width = "120px";
alphaSlider.style.height = "20px";
alphaSlider.style.marginTop = "5px";
alphaSlider.style.background =
  "linear-gradient(90deg, transparent 0%, rgba(75,192,192,0.3) 50%, rgba(75,192,192,1) 100%)";
alphaSlider.style.borderRadius = "10px";
alphaSlider.style.outline = "none";
alphaSlider.style.cursor = "pointer";

alphaSlider.addEventListener("input", (event) => {
  currentAlpha = parseFloat(event.target.value);
  alphaDisplay.textContent = `Alpha: ${currentAlpha.toFixed(2)}`;
  wasm.set_wall_alpha(currentAlpha);
  // Restart visualization with new alpha
  wasm.restart_visualization("canvas");
});

// Create intermittent walls toggle button
const wallsButton = document.createElement("button");
wallsButton.textContent = "Full Walls";
wallsButton.style.padding = "8px 16px";
wallsButton.style.marginTop = "15px";
wallsButton.style.backgroundColor = "#ff6b6b";
wallsButton.style.color = "white";
wallsButton.style.border = "none";
wallsButton.style.borderRadius = "5px";
wallsButton.style.cursor = "pointer";
wallsButton.style.fontSize = "14px";
wallsButton.style.fontWeight = "bold";

// Track current walls state (starts with intermittent = true)
let isIntermittentWalls = true;

wallsButton.addEventListener("click", () => {
  isIntermittentWalls = !isIntermittentWalls;
  wallsButton.textContent = isIntermittentWalls ? "Full Walls" : "Intermittent";
  wallsButton.style.backgroundColor = isIntermittentWalls
    ? "#ff6b6b"
    : "#45b7d1";
  wasm.set_intermittent_walls(isIntermittentWalls);
  // Restart visualization with new wall pattern
  wasm.restart_visualization("canvas");
});

// Create wireframe toggle button
const wireframeButton = document.createElement("button");
wireframeButton.textContent = "Hide Wireframe";
wireframeButton.style.padding = "8px 16px";
wireframeButton.style.marginTop = "15px";
wireframeButton.style.backgroundColor = "#45b7d1";
wireframeButton.style.color = "white";
wireframeButton.style.border = "none";
wireframeButton.style.borderRadius = "5px";
wireframeButton.style.cursor = "pointer";
wireframeButton.style.fontSize = "14px";
wireframeButton.style.fontWeight = "bold";

// Track current wireframe state (starts with wireframe = true)
let showWireframe = true;

wireframeButton.addEventListener("click", () => {
  showWireframe = !showWireframe;
  wireframeButton.textContent = showWireframe
    ? "Hide Wireframe"
    : "Show Wireframe";
  wireframeButton.style.backgroundColor = showWireframe ? "#45b7d1" : "#6c757d";
  wasm.set_show_wireframe(showWireframe);
});

// Create culling toggle button
const cullingButton = document.createElement("button");
cullingButton.textContent = "Disable Culling";
cullingButton.style.padding = "8px 16px";
cullingButton.style.marginTop = "15px";
cullingButton.style.backgroundColor = "#28a745";
cullingButton.style.color = "white";
cullingButton.style.border = "none";
cullingButton.style.borderRadius = "5px";
cullingButton.style.cursor = "pointer";
cullingButton.style.fontSize = "14px";
cullingButton.style.fontWeight = "bold";

// Track current culling state (starts with culling = true)
let enableCulling = true;

cullingButton.addEventListener("click", () => {
  enableCulling = !enableCulling;
  cullingButton.textContent = enableCulling
    ? "Disable Culling"
    : "Enable Culling";
  cullingButton.style.backgroundColor = enableCulling ? "#28a745" : "#dc3545";
  wasm.set_enable_culling(enableCulling);
});

// Assemble speed controls
speedContainer.appendChild(tValueDisplay);
speedContainer.appendChild(speedDisplay);
speedContainer.appendChild(sliderContainer);
speedContainer.appendChild(perspectiveButton);
speedContainer.appendChild(polygonCountDisplay);
speedContainer.appendChild(polygonSlider);
speedContainer.appendChild(alphaDisplay);
speedContainer.appendChild(alphaSlider);
speedContainer.appendChild(wallsButton);
speedContainer.appendChild(wireframeButton);
speedContainer.appendChild(cullingButton);

// Add containers to main container
mainContainer.appendChild(contentContainer);
mainContainer.appendChild(speedContainer);
document.body.appendChild(mainContainer);

const WIDTH = 720.0;
const HEIGHT = 480.0;

const canvas = document.createElement("canvas");
canvas.id = "canvas";
canvas.width = WIDTH;
canvas.height = HEIGHT;
canvas.style.border = "1px solid #333";
contentContainer.appendChild(canvas);

// Initialize WASM and auto-start tunnel visualization
(async () => {
  await init(mirrorwasm);

  // Auto-start tunnel with simple parameters
  wasm.start_lissajous_tunnel("canvas", A, B, R, r, n, currentSpeed);

  // Update t-value display periodically
  setInterval(() => {
    const currentT = wasm.get_current_camera_t();
    tValueDisplay.textContent = `Camera t: ${currentT.toFixed(3)}`;
  }, 100); // Update every 100ms
})();
