import init from "../pkg/cstimer_analyzer_web.js";
import { CONFIG } from "./constants.js";
import { renderMarkdown } from "./scripts.js";
import { locales } from "./locale.js";

var locale = "zh-CN";
var dictionary;

init();

document.addEventListener("DOMContentLoaded", () => {
  locale = localStorage.getItem(CONFIG.STORAGE.LOCALE_KEY) || "zh-CN";
  applyTranslations(locale);
  console.log("locale: ", locale);
});

function applyTranslations(locale) {
  dictionary = locales[locale] || locales["zh-CN"];
  document.querySelectorAll("[i18n]").forEach((el) => {
    const key = el.getAttribute("i18n");
    if (dictionary[key]) {
      el.textContent = dictionary[key];
    }
  });
}

function getLocale() {
  const raw = localStorage.getItem(CONFIG.STORAGE.LOCALE_KEY);
  return raw ? raw.trim().toLowerCase() : "en";
}

export { renderMarkdown };
export { locale, dictionary, getLocale };
