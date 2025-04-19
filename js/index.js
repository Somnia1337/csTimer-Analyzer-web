import { renderMarkdown } from "./scripts.js";
import init from "../pkg/cstimer_analyzer_web.js";

document.addEventListener("DOMContentLoaded", () => {
  console.log("csTimer-Analyzer-Web initialized");
});

init().then(() => {
  console.log("WASM is ready!");
});

export { renderMarkdown };
