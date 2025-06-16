import init, {
  init_analysis,
  analysis_info,
  get_session_count,
  analyze_nth_session,
  get_timings,
} from "../pkg/cstimer_analyzer_web.js";
import { sanitizeInput } from "./ui-manager.js";
import { CONFIG } from "./constants.js";
import { scrollInto } from "./scripts.js";
import { locale, renderMarkdown } from "./index.js";

let canvas;

function getCanvas() {
  if (!canvas) {
    canvas = document.createElement("canvas");
    canvas.style.display = "none";
    canvas.width = CONFIG.CANVAS.WIDTH;
    canvas.height = CONFIG.CANVAS.HEIGHT;
    document.body.appendChild(canvas);
  }
  return canvas;
}

export async function analyze(optionsText, file) {
  await init();

  const encoder = new TextEncoder();
  const optionsData = encoder.encode(sanitizeInput(optionsText));
  const fileData = await file.arrayBuffer();

  await init_analysis(
    new Uint8Array(optionsData),
    new Uint8Array(fileData),
    getCanvas(),
    locale
  );

  const chunks = [];
  const markdownContent = document.getElementById("markdown-content");

  const infoChunk = await analysis_info();
  chunks.push(infoChunk);
  await renderMarkdown(infoChunk);

  if (infoChunk.includes("Analysis aborted")) {
    scrollInto(markdownContent);
    return;
  }

  const sessionCount = get_session_count();

  for (let i = 0; i < sessionCount; i++) {
    const sessionChunk = analyze_nth_session(i);
    chunks.push(sessionChunk);
    await renderMarkdown(sessionChunk);
    await new Promise((r) => requestAnimationFrame(r));

    if (i == 0) {
      scrollInto(markdownContent);
    }
  }

  const timingsChunk = get_timings();
  chunks.push(timingsChunk);
  await renderMarkdown(timingsChunk);
}
