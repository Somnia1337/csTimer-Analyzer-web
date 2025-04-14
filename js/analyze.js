import init, { wasm_analyze } from "../pkg/cstimer_analyzer_web.js";
import { sanitizeInput, validateFile } from "./ui-manager.js";
import { AppError, ERROR_TYPES } from "./error-handler.js";
import { CONFIG } from "./constants.js";

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
  try {
    await init();

    const sanitizedOptions = sanitizeInput(optionsText);
    validateFile(file);

    const encoder = new TextEncoder();
    const optionsData = encoder.encode(sanitizedOptions);
    const fileData = await file.arrayBuffer().catch((err) => {
      throw new AppError(ERROR_TYPES.FILE, "readError", err);
    });

    const canvas = getAnalysisCanvas();
    const analysisReport = wasm_analyze(
      new Uint8Array(optionsData),
      new Uint8Array(fileData),
      canvas
    );

    return analysisReport;
  } catch (error) {
    if (error instanceof AppError) {
      throw error;
    } else {
      throw new AppError(ERROR_TYPES.ANALYSIS, "default", error);
    }
  }
}
