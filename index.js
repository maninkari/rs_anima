import init, * as wasm from "./pkg/rust_anima.js";
import mirrorwasm from "./pkg/rust_anima_bg.wasm";
const WIDTH = 720.0;
const HEIGHT = 480.0;
const video = document.createElement("video");
document.body.appendChild(video);
// setup and play video
(async () => {
  await init(mirrorwasm);
  const mirror = new wasm.Mirror(777);
  console.log(mirror.talk());
  const stream = await navigator.mediaDevices.getUserMedia({
    audio: false,
    video: {
      facingMode: "user",
      width: WIDTH,
      height: HEIGHT,
    },
  });
  video.srcObject = stream;
  await video.play();
})();
