import { CONFIG } from "./constants.js";
import { AppError, ERROR_TYPES, handleError } from "./error-handler.js";

export const loadingManager = {
  start(elements, message = "Loading...") {
    document.body.classList.add("loading");
    elements.markdownContent.innerHTML = `<div class="loader active"><div class="loader-spinner"></div><p>${message}</p></div>`;
  },

  end() {
    document.body.classList.remove("loading");
  },
};

export function validateFile(file) {
  if (!file) {
    throw new AppError(ERROR_TYPES.VALIDATION, "noFile");
  }

  const maxSize = CONFIG.FILE.MAX_SIZE_MB * 1024 * 1024;
  if (file.size > maxSize) {
    throw new AppError(ERROR_TYPES.FILE, "tooLarge");
  }

  const isTxt =
    file.name.toLowerCase().endsWith(".txt") && file.type === "text/plain";
  if (!isTxt) {
    throw new AppError(ERROR_TYPES.VALIDATION, "invalidFileType");
  }

  return true;
}

export function sanitizeInput(input) {
  if (!input || input.trim() === "") {
    throw new AppError(ERROR_TYPES.VALIDATION, "emptyOptions");
  }
  return input.trim();
}
