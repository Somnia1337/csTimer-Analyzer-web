import init, { render_markdown } from "../pkg/cstimer_analyzer_web.js";
import { CONFIG } from "./constants.js";
import { saveRenderedHTML, loadRenderedHTML } from "./db.js";
import { analyze } from "./analyze.js";
import { validateFile } from "./ui-manager.js";
import { AppError, ERROR_TYPES, handleError } from "./error-handler.js";
import { locales, defaultOptions } from "./locale.js";
import { locale, dictionary, getLocale } from "./index.js";

let elements;

const isZh = getLocale() === "zh-cn";

function initElements() {
  elements = {
    optionsText: document.getElementById("options"),
    fileInput: document.getElementById("file2"),
    localeButton: document.getElementById("locale-button"),
    markdownContent: document.getElementById("markdown-content"),
    errorMessage: document.getElementById("error-message"),
    errorText: document.getElementById("error-text"),
    githubButton: document.getElementById("github-button"),
    useExampleButton: document.getElementById("use-example"),
    resetButton: document.getElementById("reset-options"),
    navHeader: document.getElementById("navigator-header"),
    backToTopButton: document.getElementById("back-to-top"),
    label: document.getElementById("file2-label"),
  };
}

document.addEventListener("DOMContentLoaded", function () {
  initElements();
  initDocsButton(
    isZh ? "./README-ZH.md" : "./README.md",
    "readme-button",
    "README"
  );
  initDocsButton("./docs/CHANGELOG.md", "changelog-button", "Changelog");
  initDocsButton("./docs/feedback.md", "feedback-button", "Feedback");
  initLocaleButton();
  initGitHubButton();
  initOptionsTextarea();
  initExampleButton();
  initFileSelection();
  initFileLabel();
  initNavigator();
  initNavHeader();
  initBackToTopButton();
  initContentDB();
});

export function scrollInto(element) {
  element.scrollIntoView({
    behavior: "smooth",
    block: "start",
  });
}

function initDocsButton(path, id, desc) {
  const docsButton = document.getElementById(id);

  docsButton.addEventListener("click", async function () {
    try {
      elements.label.textContent = desc;
      localStorage.setItem(
        CONFIG.STORAGE.FILE_LABEL_KEY,
        elements.label.textContent
      );

      elements.markdownContent.innerHTML = "";

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

      elements.navHeader.textContent = desc;
      localStorage.setItem(
        CONFIG.STORAGE.NAV_HEADER_KEY,
        elements.navHeader.textContent
      );

      scrollInto(elements.markdownContent);

      saveRenderedHTML(elements.markdownContent.innerHTML);
    } catch (error) {
      handleError(error, elements);
    }
  });
}

function initLocaleButton() {
  elements.localeButton.addEventListener("click", function () {
    const newLocale = isZh ? "en" : "zh-CN";
    const newDictionary = locales[newLocale] || locales["en"];
    const message = newDictionary["new-locale"];

    alert(message);
    localStorage.setItem(CONFIG.STORAGE.LOCALE_KEY, newLocale);
    localStorage.setItem(
      CONFIG.STORAGE.FILE_LABEL_KEY,
      newDictionary["no-upload"]
    );
    saveRenderedHTML(newDictionary["waiting-for-file"]);
    window.location.reload();
  });
}

function initGitHubButton() {
  elements.githubButton.addEventListener("click", function () {
    window.open("https://github.com/Somnia1337/csTimer-Analyzer-web", "_blank");
  });
}

function initExampleButton() {
  elements.useExampleButton.addEventListener("click", async function () {
    try {
      elements.useExampleButton.textContent =
        dictionary["example-file-loading"];

      const response = await fetch("./example.txt");
      if (!response.ok) {
        throw new AppError(
          ERROR_TYPES.NETWORK,
          "fetch",
          new Error(`Failed to load example file: ${response.status}`)
        );
      }

      elements.useExampleButton.textContent = dictionary["example-file"];

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

      elements.navHeader.textContent = dictionary["report-example"];
      localStorage.setItem(
        CONFIG.STORAGE.NAV_HEADER_KEY,
        elements.navHeader.textContent
      );
    } catch (error) {
      handleError(error, elements);
      elements.label.textContent = dictionary["example-file-error"];
      localStorage.setItem(
        CONFIG.STORAGE.FILE_LABEL_KEY,
        elements.label.textContent
      );
    } finally {
      elements.useExampleButton.textContent = dictionary["example-file"];
    }
  });
}

function initFileSelection() {
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

        elements.navHeader.textContent = dictionary["report"];
        localStorage.setItem(
          CONFIG.STORAGE.NAV_HEADER_KEY,
          elements.navHeader.textContent
        );
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

function initOptionsTextarea() {
  window.addEventListener("DOMContentLoaded", () => {
    const savedContent = localStorage.getItem(CONFIG.STORAGE.OPTIONS_KEY);
    elements.optionsText.value =
      savedContent !== null
        ? savedContent
        : defaultOptions[locale] || defaultOptions["en"];

    const placeholderKey = elements.optionsText.getAttribute("i18n");
    elements.optionsText.placeholder = dictionary[placeholderKey];
  });

  elements.optionsText.addEventListener(
    "input",
    debounce(() => {
      localStorage.setItem(
        CONFIG.STORAGE.OPTIONS_KEY,
        elements.optionsText.value
      );
    }, 300)
  );

  elements.resetButton.addEventListener("click", () => {
    elements.optionsText.value = defaultOptions[locale] || defaultOptions["en"];
    localStorage.removeItem(CONFIG.STORAGE.OPTIONS_KEY);
  });
}

function initBackToTopButton() {
  const scrollThreshold = 1400;

  function toggleBackToTopButton() {
    if (window.scrollY > scrollThreshold) {
      elements.backToTopButton.classList.add("visible");
    } else {
      elements.backToTopButton.classList.remove("visible");
    }
  }

  toggleBackToTopButton();

  window.addEventListener("scroll", toggleBackToTopButton);

  elements.backToTopButton.addEventListener("click", function () {
    scrollInto(elements.markdownContent);
  });
}

function initNavigator() {
  const tocId = "toc-navigator";
  const tocListId = "toc-list";
  const markdownId = "markdown-content";
  const collapsedClass = "collapsed";

  if (!document.getElementById(tocId)) {
    const wrapper = document.createElement("div");
    wrapper.innerHTML = `
      <div id="${tocId}" class="toc-navigator">
        <div class="toc-header">
          <button id="toggle-toc" class="toggle-toc" aria-label="Toggle navigator">
            <i class="fas fa-chevron-right"></i>
          </button>
          <span id="navigator-header" i18n="outline"></span>
          <button class="back-to-top" id="back-to-top" aria-label="Back to top">
            <i class="fas fa-arrow-up"></i>
          </button>
        </div>
        <div class="toc-content">
          <ul id="${tocListId}"></ul>
        </div>
      </div>
    `;
    document.body.appendChild(wrapper.firstElementChild);
  }

  const tocNavigator = document.getElementById(tocId);
  const tocList = document.getElementById(tocListId);
  const markdownContent = document.getElementById(markdownId);
  const toggleButton = document.getElementById("toggle-toc");

  function toggleTOC() {
    tocNavigator.classList.toggle(collapsedClass);
    localStorage.setItem(
      "tocCollapsed",
      tocNavigator.classList.contains(collapsedClass)
    );
  }

  function applyCollapsedState() {
    if (localStorage.getItem("tocCollapsed") === "true") {
      tocNavigator.classList.add(collapsedClass);
    }
  }

  function setActiveLink(id) {
    document.querySelectorAll(`#${tocListId} a`).forEach((a) => {
      a.classList.toggle("active", a.getAttribute("href") === `#${id}`);
    });
  }

  function generateTOC() {
    const target = markdownContent.querySelector("h1") ? "h2" : "h3";
    const headings = markdownContent.querySelectorAll(target);

    if (headings.length === 0) {
      tocNavigator.classList.remove("visible");
      return;
    }

    tocList.innerHTML = "";
    headings.forEach((heading, index) => {
      if (!heading.id) heading.id = `heading-${index}`;

      const link = document.createElement("a");
      link.href = `#${heading.id}`;
      link.textContent = heading.textContent;
      link.addEventListener("click", (e) => {
        e.preventDefault();
        scrollInto(heading);
        setActiveLink(heading.id);
      });

      const listItem = document.createElement("li");
      listItem.appendChild(link);
      tocList.appendChild(listItem);
    });

    tocNavigator.classList.add("visible");
  }

  function updateActiveHeadingOnScroll() {
    const target = markdownContent.querySelector("h1") ? "h2" : "h3";
    const headings = markdownContent.querySelectorAll(target);
    let current = "";
    headings.forEach((heading) => {
      if (heading.getBoundingClientRect().top <= 100) {
        current = heading.id;
      }
    });
    if (current) setActiveLink(current);
  }

  toggleButton.addEventListener("click", toggleTOC);
  applyCollapsedState();
  generateTOC();

  new MutationObserver(generateTOC).observe(markdownContent, {
    childList: true,
    subtree: true,
  });
  window.addEventListener("scroll", updateActiveHeadingOnScroll);
}

function initFileLabel() {
  const savedLabel = localStorage.getItem(CONFIG.STORAGE.FILE_LABEL_KEY);

  if (savedLabel !== null) {
    elements.label.innerHTML = savedLabel;
  }
}

function initNavHeader() {
  const savedHeader = localStorage.getItem(CONFIG.STORAGE.NAV_HEADER_KEY);

  if (savedHeader !== null) {
    elements.navHeader.textContent = savedHeader;
  }
}

function initContentDB() {
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

export async function renderMarkdown(markdown) {
  try {
    await init();
    const rendered = render_markdown(markdown);
    const temp = document.createElement("div");
    temp.innerHTML = rendered;
    while (temp.firstChild) {
      elements.markdownContent.appendChild(temp.firstChild);
    }
  } catch (error) {
    throw new AppError(ERROR_TYPES.ANALYSIS, "render", error);
  }
}

async function run() {
  try {
    elements.errorMessage.classList.remove("active");

    const optionsText = elements.optionsText.value;
    const file = elements.fileInput.files[0];

    elements.markdownContent.innerHTML = "";

    await analyze(optionsText, file);
    await saveRenderedHTML(elements.markdownContent.innerHTML);
  } catch (error) {
    handleError(error, elements);

    if (error instanceof AppError && error.type === ERROR_TYPES.VALIDATION) {
      elements.markdownContent.innerHTML = dictionary["waiting-for-file"];
    }
  }
}

window.run = run;
