import init, { render_markdown } from "../pkg/cstimer_analyzer_web.js";
import { CONFIG } from "./constants.js";
import { saveRenderedHTML, loadRenderedHTML } from "./db.js";
import { analyzeTimerData } from "./analyze.js";
import { loadingManager, validateFile } from "./ui-manager.js";
import { AppError, ERROR_TYPES, handleError } from "./error-handler.js";

let elements;

function initializeElements() {
  elements = {
    optionsText: document.getElementById("options"),
    fileInput: document.getElementById("file2"),
    markdownContent: document.getElementById("markdown-content"),
    errorMessage: document.getElementById("error-message"),
    errorText: document.getElementById("error-text"),
    githubButton: document.getElementById("github-button"),
    useExampleButton: document.getElementById("use-example"),
    resetButton: document.getElementById("reset-options"),
    backToTopButton: document.getElementById("back-to-top"),
    label: document.getElementById("file2-label"),
  };
}

document.addEventListener("DOMContentLoaded", function () {
  initializeElements();
  initializeDocsButton("./README-ZH.md", "readme-button", "README");
  initializeDocsButton("./docs/CHANGELOG.md", "changelog-button", "Changelog");
  initializeDocsButton("./docs/feedback.md", "feedback-button", "Feedback");
  initializeGitHubButton();
  initializeExampleButton();
  initializeFileSelection();
  initializeOptionsTextarea();
  initializeFileLabel();
  initializeBackToTopButton();
  initializeContentDB();
});

function initializeDocsButton(path, id, desc) {
  const docsButton = document.getElementById(id);

  docsButton.addEventListener("click", async function () {
    try {
      elements.label.textContent = desc;
      localStorage.setItem(
        CONFIG.STORAGE.FILE_LABEL_KEY,
        elements.label.textContent
      );

      loadingManager.start(elements, `Loading ${desc}...`);

      const response = await fetch(path);
      if (!response.ok) {
        throw new AppError(
          ERROR_TYPES.NETWORK,
          "fetch",
          new Error(`Failed to load ${desc}: ${response.statusText}`)
        );
      }

      const docs = await response.text();
      await renderMarkdown(docs);
    } catch (error) {
      handleError(error, elements);
    } finally {
      loadingManager.end();
    }
  });
}

function initializeGitHubButton() {
  elements.githubButton.addEventListener("click", function () {
    window.open("https://github.com/Somnia1337/csTimer-Analyzer-web", "_blank");
  });
}

function initializeExampleButton() {
  elements.useExampleButton.addEventListener("click", async function () {
    try {
      elements.useExampleButton.textContent = "Loading...";

      const response = await fetch("./example.txt");
      if (!response.ok) {
        throw new AppError(
          ERROR_TYPES.NETWORK,
          "fetch",
          new Error(`Failed to load example file: ${response.status}`)
        );
      }

      elements.useExampleButton.textContent = "use example file";

      const text = await response.text();
      const blob = new Blob([text], { type: "text/plain" });

      const file = new File([blob], "example.txt", { type: "text/plain" });

      elements.label.textContent = file.name;
      localStorage.setItem(
        CONFIG.STORAGE.FILE_LABEL_KEY,
        elements.label.textContent
      );

      const dataTransfer = new DataTransfer();
      dataTransfer.items.add(file);

      elements.fileInput.files = dataTransfer.files;

      await run();
    } catch (error) {
      handleError(error, elements);
      elements.label.textContent = "Error loading example";
      localStorage.setItem(
        CONFIG.STORAGE.FILE_LABEL_KEY,
        elements.label.textContent
      );
    }
  });
}

function initializeFileSelection() {
  elements.fileInput.addEventListener("click", function () {
    this.value = "";
  });

  elements.fileInput.addEventListener("change", async function (e) {
    const file = e.target.files[0];

    if (file) {
      try {
        validateFile(file);
        elements.label.textContent = file.name;
        localStorage.setItem(
          CONFIG.STORAGE.FILE_LABEL_KEY,
          elements.label.textContent
        );
        await run();
      } catch (error) {
        handleError(error, elements);
      }
    } else {
      elements.label.textContent = CONFIG.UI.DEFAULT_LABEL;
      localStorage.setItem(
        CONFIG.STORAGE.FILE_LABEL_KEY,
        elements.label.textContent
      );
    }
  });
}

function debounce(fn, delay = 1000) {
  let timeoutId;
  return function (...args) {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(() => fn.apply(this, args), delay);
  };
}

function initializeOptionsTextarea() {
  window.addEventListener("DOMContentLoaded", () => {
    const savedContent = localStorage.getItem(CONFIG.STORAGE.OPTIONS_KEY);
    if (savedContent !== null) {
      elements.optionsText.value = savedContent;
    } else {
      elements.optionsText.value = elements.optionsText.dataset.default;
    }
  });

  elements.optionsText.addEventListener(
    "input",
    debounce(() => {
      localStorage.setItem(
        CONFIG.STORAGE.OPTIONS_KEY,
        elements.optionsText.value
      );
    }, 3000)
  );

  elements.resetButton.addEventListener("click", () => {
    const defaultContent = elements.optionsText.dataset.default;
    elements.optionsText.value = defaultContent;
    localStorage.removeItem(CONFIG.STORAGE.OPTIONS_KEY);
  });
}

function initializeBackToTopButton() {
  const scrollThreshold = 1600;

  function toggleBackToTopButton() {
    if (window.scrollY > scrollThreshold) {
      elements.backToTopButton.classList.add("visible");
    } else {
      elements.backToTopButton.classList.remove("visible");
    }
  }

  toggleBackToTopButton(); // Initial check on load

  window.addEventListener("scroll", toggleBackToTopButton);

  elements.backToTopButton.addEventListener("click", function () {
    elements.markdownContent.scrollIntoView({
      behavior: "smooth",
      block: "start",
    });
  });
}

function initializeFileLabel() {
  const savedLabel = localStorage.getItem(CONFIG.STORAGE.FILE_LABEL_KEY);

  if (savedLabel !== null) {
    elements.label.textContent = savedLabel;
  }
}

function initializeContentDB() {
  loadRenderedHTML()
    .then((savedHTML) => {
      if (savedHTML) {
        elements.markdownContent.innerHTML = savedHTML;
      }
    })
    .catch((error) => {
      console.error("Failed to load saved content:", error);
    });
}

async function renderMarkdown(markdown) {
  try {
    await init();
    const rendered = render_markdown(markdown);
    elements.markdownContent.innerHTML = rendered;
    elements.markdownContent.scrollIntoView({
      behavior: "smooth",
      block: "start",
    });
    await saveRenderedHTML(rendered);
    return rendered;
  } catch (error) {
    throw new AppError(ERROR_TYPES.ANALYSIS, "render", error);
  }
}

async function run() {
  try {
    elements.errorMessage.classList.remove("active");

    const optionsText = elements.optionsText.value;
    const file = elements.fileInput.files[0];

    loadingManager.start(elements, "Analyzing...");

    const analysisReport = await analyzeTimerData(optionsText, file);

    await renderMarkdown(analysisReport);
  } catch (error) {
    handleError(error, elements);

    if (error instanceof AppError && error.type === ERROR_TYPES.VALIDATION) {
      elements.markdownContent.innerHTML = `<strong>Waiting for data file selection...</strong>`;
    }
  } finally {
    loadingManager.end();
  }
}

window.run = run;
export { renderMarkdown };
