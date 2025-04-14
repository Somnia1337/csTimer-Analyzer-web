export const ERROR_TYPES = {
  FILE: "file",
  ANALYSIS: "analysis",
  NETWORK: "network",
  DATABASE: "database",
  VALIDATION: "validation",
};

const messages = {
  [ERROR_TYPES.FILE]: {
    default: "文件处理错误",
    notFound: "找不到文件",
    invalidType: "无效的文件类型",
    tooLarge: "文件太大",
    readError: "文件读取错误",
  },
  [ERROR_TYPES.ANALYSIS]: {
    default: "分析过程出错",
  },
  [ERROR_TYPES.NETWORK]: {
    default: "网络请求错误",
  },
  [ERROR_TYPES.DATABASE]: {
    default: "数据库操作错误",
  },
  [ERROR_TYPES.VALIDATION]: {
    default: "输入验证错误",
    emptyOptions: "请输入分析选项",
    noFile: "请选择csTimer数据文件",
    invalidFileType: "请选择.txt文件",
  },
};

export class AppError extends Error {
  constructor(type, code = "default", originalError = null) {
    const message =
      messages[type]?.[code] || messages[type]?.default || "未知错误";
    super(message);
    this.name = "AppError";
    this.type = type;
    this.code = code;
    this.originalError = originalError;
  }
}

export function handleError(error, elements, resetContent = true) {
  console.error(error);

  let errorMessage = "";

  if (error instanceof AppError) {
    errorMessage = error.message;
    if (error.originalError) {
      console.error("Original error:", error.originalError);
    }
  } else {
    errorMessage = error.message || "发生未知错误";
  }

  elements.errorText.textContent = errorMessage;
  elements.errorMessage.classList.add("active");

  if (resetContent) {
    elements.markdownContent.innerHTML = `<strong>Waiting for data file selection...</strong>`;
  }

  return errorMessage;
}
