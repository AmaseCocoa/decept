import init, { run_wdec, pack_logic } from "./pkg/decept.js";

let wasmReady = false;

async function setup() {
  try {
    await init();
    wasmReady = true;
    document.getElementById("status").innerText = "WASM Loaded";
  } catch (e) {
    document.getElementById("status").innerText = "Load Failed";
  }
}

document.querySelectorAll(".tab-btn").forEach((btn) => {
  btn.addEventListener("click", () => {
    document
      .querySelectorAll(".tab-btn, .tab-content")
      .forEach((el) => el.classList.remove("active"));
    btn.classList.add("active");
    document
      .getElementById(`${btn.dataset.tab}-section`)
      .classList.add("active");
  });
});

document.getElementById("run-btn").addEventListener("click", async () => {
  if (!wasmReady) return;
  const file = document.getElementById("logic-file").files[0];
  if (!file) return alert("Select .decc file");

  const code = document.getElementById("dsl-input").value;
  const bin = new Uint8Array(await file.arrayBuffer());

  try {
    const result = run_wdec(bin, code);
    document.getElementById("output").innerText = result;
  } catch (e) {
    document.getElementById("output").innerText = "Error: " + e;
  }
});

document.getElementById("compile-btn").addEventListener("click", () => {
  if (!wasmReady) return;
  const source = document.getElementById("compile-input").value;
  if (!source) return alert("Enter script to compile");

  try {
    const packedData = pack_logic(source);

    const blob = new Blob([packedData], {
      type: "application/octet-stream",
    });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "logic.decc";
    a.click();
    URL.revokeObjectURL(url);

    document.getElementById("output").innerText =
      "Successfully packed and downloaded logic.decc";
  } catch (e) {
    document.getElementById("output").innerText = "Compile Error: " + e;
  }
});

setup();
