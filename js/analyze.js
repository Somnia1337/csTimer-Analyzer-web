import init, {
  init_analysis,
  analysis_info,
  get_session_count,
  analyze_session,
  get_timings,
} from "../pkg/cstimer_analyzer_web.js";
import { sanitizeInput } from "./ui-manager.js";
import { CONFIG } from "./constants.js";
import { renderMarkdown } from "./scripts.js";

let analysisCanvas;

function getAnalysisCanvas() {
  if (!analysisCanvas) {
    analysisCanvas = document.createElement("canvas");
    analysisCanvas.style.display = "none";
    analysisCanvas.width = CONFIG.CANVAS.WIDTH;
    analysisCanvas.height = CONFIG.CANVAS.HEIGHT;
    document.body.appendChild(analysisCanvas);
  }
  return analysisCanvas;
}

export async function analyzeTimerData(optionsText, file) {
  await init();

  const encoder = new TextEncoder();
  const optionsData = encoder.encode(sanitizeInput(optionsText));
  const fileData = await file.arrayBuffer();

  await init_analysis(
    new Uint8Array(optionsData),
    new Uint8Array(fileData),
    getAnalysisCanvas()
  );

  const chunks = [];
  const markdownContent = document.getElementById("markdown-content");

  const infoChunk = await analysis_info();
  chunks.push(infoChunk);
  await renderMarkdown(infoChunk);

  if (infoChunk.includes("Analysis aborted")) {
    markdownContent.scrollIntoView({
      behavior: "smooth",
      block: "start",
    });

    return;
  }

  const sessionCount = get_session_count();

  for (let i = 0; i < sessionCount; i++) {
    const sessionChunk = analyze_session(i);
    chunks.push(sessionChunk);
    await renderMarkdown(sessionChunk);
    await new Promise((r) => requestAnimationFrame(r));

    if (i == 0) {
      markdownContent.scrollIntoView({
        behavior: "smooth",
        block: "start",
      });
    }
  }

  const timingsChunk = get_timings();
  chunks.push(timingsChunk);
  await renderMarkdown(timingsChunk);
}
