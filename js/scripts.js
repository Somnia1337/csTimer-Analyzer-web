import init, { render_markdown } from "../pkg/cstimer_analyzer_web.js";

document.addEventListener("DOMContentLoaded", function () {
  initializeDocsButton("./README-ZH.md", "readme-button", "README");
  initializeDocsButton("./CHANGELOG.md", "changelog-button", "Changelog");
  initializeExampleButton();
  initializeFileSelection();
});

function initializeDocsButton(path, id, desc) {
  const docsButton = document.getElementById(id);
  const markdownContent = document.getElementById("markdown-content");

  docsButton.addEventListener("click", async function () {
    try {
      markdownContent.innerHTML = `<div class="loader active"><div class="loader-spinner"></div><p>Loading ${desc}...</p></div>`;

      const response = await fetch(path);
      if (!response.ok) {
        throw new Error(`Failed to load ${desc}: ${response.statusText}`);
      }

      const docs = await response.text();
      await init();
      markdownContent.innerHTML = render_markdown(docs);
      loader.classList.remove("active");
      markdownContent.scrollIntoView({ behavior: "smooth", block: "start" });
    } catch (error) {
      markdownContent.innerHTML = `<div class="error-message active">Error loading Changelog: ${error.message}</div>`;
    }
  });
}

function initializeExampleButton() {
  const useExampleButton = document.getElementById("use-example");
  useExampleButton.addEventListener("click", async function () {
    const fileSelected = document.getElementById("file-selected");
    const filenameDisplay = document.getElementById("filename-display");
    const label = document.getElementById("file2-label");

    try {
      label.textContent = "Loading example file...";

      const response = await fetch("./example.txt");

      if (!response.ok) {
        throw new Error(`Failed to load example file: ${response.status}`);
      }

      const text = await response.text();
      const blob = new Blob([text], { type: "text/plain" });

      const file = new File([blob], "example.txt", {
        type: "text/plain",
      });

      const dataTransfer = new DataTransfer();
      dataTransfer.items.add(file);

      const fileInput = document.getElementById("file2");
      fileInput.files = dataTransfer.files;

      filenameDisplay.textContent = "example.txt";
      fileSelected.classList.add("active");
      label.textContent = "Example file loaded";

      run();
    } catch (error) {
      const errorMessage = document.getElementById("error-message");
      const errorText = document.getElementById("error-text");
      errorText.textContent = error.message;
      errorMessage.classList.add("active");
      label.textContent = "Error loading example";
    }
  });
}

function initializeFileSelection() {
  document.getElementById("file2").addEventListener("change", function (e) {
    const file = e.target.files[0];
    const fileSelected = document.getElementById("file-selected");
    const filenameDisplay = document.getElementById("filename-display");
    const label = document.getElementById("file2-label");

    if (file) {
      filenameDisplay.textContent = file.name;
      fileSelected.classList.add("active");
      label.textContent = "File selected";
      run();
    } else {
      fileSelected.classList.remove("active");
      label.textContent = "Select csTimer Data";
    }
  });
}

window.run = async function () {
  console.log("Waiting for WASM module to load...");
};
