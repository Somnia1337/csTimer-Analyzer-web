import init, { render_markdown } from "../pkg/cstimer_analyzer_web.js";
import { saveRenderedHTML, loadRenderedHTML } from "./db.js";

document.addEventListener("DOMContentLoaded", function () {
  initializeDocsButton("./README-ZH.md", "readme-button", "README");
  initializeDocsButton("./CHANGELOG.md", "changelog-button", "Changelog");
  initializeGitHubButton();
  initializeExampleButton();
  initializeFileSelection();
  initializeOptionsTextarea();
  initializeFileLabel();
  initializeContentDB();
});

function initializeDocsButton(path, id, desc) {
  const docsButton = document.getElementById(id);
  const markdownContent = document.getElementById("markdown-content");
  const label = document.getElementById("file2-label");

  docsButton.addEventListener("click", async function () {
    try {
      label.textContent = desc;
      localStorage.setItem("fileLabel", label.textContent);

      markdownContent.innerHTML = `<div class="loader active"><div class="loader-spinner"></div><p>Loading ${desc}...</p></div>`;

      const response = await fetch(path);
      if (!response.ok) {
        throw new Error(`Failed to load ${desc}: ${response.statusText}`);
      }

      const docs = await response.text();
      await init();

      const rendered = render_markdown(docs);
      markdownContent.innerHTML = rendered;
      markdownContent.scrollIntoView({ behavior: "smooth", block: "start" });

      await saveRenderedHTML(rendered);
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
      localStorage.setItem("fileLabel", label.textContent);

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
      localStorage.setItem("fileLabel", label.textContent);
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
        localStorage.setItem("fileLabel", label.textContent);

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
        localStorage.setItem("fileLabel", label.textContent);
      }
    });
}

function initializeOptionsTextarea() {
  const textarea = document.getElementById("options");
  const STORAGE_KEY = "analysisOptions";
  const resetButton = document.getElementById("reset-options");

  window.addEventListener("DOMContentLoaded", () => {
    const savedContent = localStorage.getItem(STORAGE_KEY);
    if (savedContent !== null) {
      textarea.value = savedContent;
    } else {
      textarea.value = textarea.dataset.default;
    }
  });

  textarea.addEventListener("input", () => {
    localStorage.setItem(STORAGE_KEY, textarea.value);
  });

  resetButton.addEventListener("click", () => {
    const defaultContent = textarea.dataset.default;
    textarea.value = defaultContent;
    localStorage.removeItem(STORAGE_KEY);
  });
}

function initializeFileLabel() {
  const label = document.getElementById("file2-label");
  const savedLabel = localStorage.getItem("fileLabel");

  if (savedLabel !== null) {
    label.textContent = savedLabel;
  }
}

function initializeContentDB() {
  const markdownContent = document.getElementById("markdown-content");
  loadRenderedHTML()
    .then((savedHTML) => {
      if (savedHTML) {
        markdownContent.innerHTML = savedHTML;
      }
    })
    .catch(console.error);
}
