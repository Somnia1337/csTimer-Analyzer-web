import init, {
  wasm_analyze,
  render_markdown,
} from "../pkg/cstimer_analyzer_web.js";

async function run() {
  await init();
  const optionsText = document.getElementById("options").value;
  const file2 = document.getElementById("file2").files[0];
  const markdownContent = document.getElementById("markdown-content");
  const errorMessage = document.getElementById("error-message");
  const errorText = document.getElementById("error-text");
  const canvas = document.createElement("canvas");

  canvas.style.display = "none";
  canvas.width = 1920;
  canvas.height = 1080;
  document.body.appendChild(canvas);

  errorMessage.classList.remove("active");

  if (!optionsText) {
    errorText.textContent = "Please enter analysis options.";
    errorMessage.classList.add("active");
    canvas.remove();
    return;
  }

  if (!file2) {
    errorText.textContent = "Please select a csTimer data file.";
    errorMessage.classList.add("active");
    canvas.remove();
    return;
  }

  markdownContent.innerHTML = `<div class="loader active"><div class="loader-spinner"></div><p>Analyzing...</p></div>`;

  const encoder = new TextEncoder();
  const data1 = encoder.encode(optionsText);

  try {
    document.body.classList.add("loading");

    const data2 = await file2.arrayBuffer();

    try {
      const result = wasm_analyze(
        new Uint8Array(data1),
        new Uint8Array(data2),
        canvas
      );

      markdownContent.innerHTML = render_markdown(result);
      markdownContent.scrollIntoView({ behavior: "smooth", block: "start" });

      canvas.remove();
    } catch (e) {
      errorText.textContent = "Analysis error: " + e.message;
      errorMessage.classList.add("active");
      canvas.remove();
    }
  } catch (e) {
    errorText.textContent = "File reading error: " + e.message;
    errorMessage.classList.add("active");
    canvas.remove();
  } finally {
    document.body.classList.remove("loading");
  }
}

document.addEventListener("DOMContentLoaded", function () {
  document
    .getElementById("file2")
    .addEventListener("change", async function (e) {
      const file = e.target.files[0];
      const fileSelected = document.getElementById("file-selected");
      const filenameDisplay = document.getElementById("filename-display");
      const label = document.getElementById("file2-label");

      if (file) {
        filenameDisplay.textContent = file.name;
        fileSelected.classList.add("active");
        label.textContent = "File selected";
        await run();
      } else {
        fileSelected.classList.remove("active");
        label.textContent = "Select csTimer Data";
      }
    });
});

window.run = run;
