import init, {
  analyze_from_files,
  render_markdown,
} from "../pkg/cstimer_analyzer_web.js";

async function run() {
  document.body.classList.add("loading");
  await init();
  const optionsText = document.getElementById("options").value;
  const file2 = document.getElementById("file2").files[0];
  const resultDiv = document.getElementById("result");
  const loader = document.getElementById("loader");
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
    document.body.classList.remove("loading");
    return;
  }

  if (!file2) {
    errorText.textContent = "Please select a csTimer data file.";
    errorMessage.classList.add("active");
    canvas.remove();
    document.body.classList.remove("loading");
    return;
  }

  loader.classList.add("active");

  const encoder = new TextEncoder();
  const data1 = encoder.encode(optionsText);

  try {
    const data2 = await file2.arrayBuffer();

    try {
      const result = analyze_from_files(
        new Uint8Array(data1),
        new Uint8Array(data2),
        canvas
      );

      resultDiv.innerHTML = render_markdown(result);
      loader.classList.remove("active");
      resultDiv.scrollIntoView({ behavior: "smooth", block: "start" });

      canvas.remove();
    } catch (e) {
      loader.classList.remove("active");
      errorText.textContent = "Analysis error: " + e.message;
      errorMessage.classList.add("active");
      canvas.remove();
    }
  } catch (e) {
    loader.classList.remove("active");
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
