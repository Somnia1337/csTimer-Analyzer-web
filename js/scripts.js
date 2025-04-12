import init, { render_markdown } from "../pkg/cstimer_analyzer_web.js";

document.addEventListener("DOMContentLoaded", function () {
  initializeDocsButton("./README-ZH.md", "readme-button", "README");
  initializeDocsButton("./CHANGELOG.md", "changelog-button", "Changelog");
  initializeGitHubButton();
  initializeExampleButton();
  initializeFileSelection();
});

function initializeDocsButton(path, id, desc) {
  const docsButton = document.getElementById(id);
  const markdownContent = document.getElementById("markdown-content");
  const label = document.getElementById("file2-label");

  docsButton.addEventListener("click", async function () {
    try {
      label.textContent = desc;
      markdownContent.innerHTML = `<div class="loader active"><div class="loader-spinner"></div><p>Loading ${desc}...</p></div>`;

      const response = await fetch(path);
      if (!response.ok) {
        throw new Error(`Failed to load ${desc}: ${response.statusText}`);
      }

      const docs = await response.text();
      await init();
      markdownContent.innerHTML = render_markdown(docs);
      markdownContent.scrollIntoView({ behavior: "smooth", block: "start" });
    } catch (error) {
      markdownContent.innerHTML = `<div class="error-message active">Error loading Changelog: ${error.message}</div>`;
    }
  });
}

function initializeGitHubButton() {
  document
    .getElementById("github-button")
    .addEventListener("click", function () {
      window.open(
        "https://github.com/Somnia1337/csTimer-Analyzer-web",
        "_blank"
      );
    });
}

function initializeExampleButton() {
  const useExampleButton = document.getElementById("use-example");
  useExampleButton.addEventListener("click", async function () {
    const label = document.getElementById("file2-label");
    const buttonLabel = document.getElementById("use-example");

    try {
      buttonLabel.textContent = "Loading...";

      const response = await fetch("./example.txt");

      if (!response.ok) {
        throw new Error(`Failed to load example file: ${response.status}`);
      }

      buttonLabel.textContent = "use example file";

      const text = await response.text();
      const blob = new Blob([text], { type: "text/plain" });

      const file = new File([blob], "example.txt", {
        type: "text/plain",
      });

      label.textContent = file.name;

      const dataTransfer = new DataTransfer();
      dataTransfer.items.add(file);

      const fileInput = document.getElementById("file2");
      fileInput.files = dataTransfer.files;

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
  document
    .getElementById("file2")
    .addEventListener("change", async function (e) {
      const file = e.target.files[0];
      const label = document.getElementById("file2-label");
      const errorMessage = document.getElementById("error-message");
      const errorText = document.getElementById("error-text");

      if (file) {
        label.textContent = file.name;

        const isTxt =
          file.name.toLowerCase().endsWith(".txt") &&
          file.type === "text/plain";

        if (!isTxt) {
          errorText.textContent = "Please choose a .txt file.";
          errorMessage.classList.add("active");
          return;
        }

        await run();
      } else {
        label.textContent = "Select csTimer Data";
      }
    });
}

window.run = async function () {
  console.log("Waiting for WASM module to load...");
};
