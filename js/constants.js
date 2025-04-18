export const CONFIG = {
  STORAGE: {
    OPTIONS_KEY: "analysisOptions",
    FILE_LABEL_KEY: "fileLabel",
    NAV_HEADER_KEY: "navheader",
    MARKDOWN_KEY: "markdownInnerHTML",
  },
  CANVAS: {
    WIDTH: 1920,
    HEIGHT: 1080,
  },
  FILE: {
    MAX_SIZE_MB: 10,
    VALID_TYPES: ["text/plain"],
    VALID_EXTENSIONS: [".txt"],
  },
  DB: {
    NAME: "MarkdownContentCache",
    VERSION: 1,
    STORE_NAME: "renderedHTML",
    KEY: "markdown",
  },
  UI: {
    DEFAULT_LABEL: "Select csTimer Data",
  },
};
