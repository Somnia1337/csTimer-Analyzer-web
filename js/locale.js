export const locales = {
  en: {
    title: "csTimer Analyzer",
    versions: "Versions",
    "earlier-versions": "earlier",
    "analysis-options": "Analysis Options",
    "default-options": "use default options",
    "cstimer-data": "csTimer Data",
    "example-file": "use example file",
    "example-file-loading": "Loading...",
    "example-file-error": "Error loading example",
    "no-upload":
      "Your data will not be uploaded.<br/>All computations are done in your browser.",
    "waiting-for-file": "Waiting for data file selection...",
    outline: "Outline",
    "textarea-placeholder": "Enter your analysis options here...",
    "new-locale": "Language set to English, please re-analyze manually",
    report: "Report",
    "report-example": "Report (example file)",
  },
  "zh-CN": {
    title: "csTimer 分析师",
    versions: "版本",
    "earlier-versions": "早期版本",
    "analysis-options": "分析选项",
    "default-options": "使用默认选项",
    "cstimer-data": "csTimer 数据",
    "example-file": "使用示例数据",
    "example-file-loading": "加载中...",
    "example-file-error": "示例加载错误",
    "no-upload": "你的数据不会被上传<br/>浏览器完成所有计算",
    "waiting-for-file": "等待选择数据文件...",
    outline: "大纲",
    "textarea-placeholder": "输入你的分析选项...",
    "new-locale": "语言切换为中文，请手动重新分析",
    report: "分析报告",
    "report-example": "分析报告 (示例文件)",
  },
};

export const defaultOptions = {
  en: `# The options for analysis
# Comment starts with '#'

# Summary
summary

# PB histories
pbs(single)
pbs(mo3)

# Grouping charts
group(single, 500) # 500ms

# Trending charts
trend(ao12)
trend(ao100)

# Recent solves ✨
recent(200) # 200 solves
recent(10%) # 10% solves
recent(2025-01-01) # start date
recent(2024-01-01, 2024-12-31) # date range

# Commented records
commented`,
  "zh-CN": `# 分析选项
# 注释以 '#' 开头

# 分组概览
summary

# 个人最佳成绩历史
pbs(single)
pbs(mo3)

# 直方图
group(single, 500) # 500毫秒

# 趋势图
trend(ao12)
trend(ao100)

# 最近记录 ✨
recent(200) # 200 次复原
recent(10%) # 10% 的复原
recent(2025-01-01) # 起始日期
recent(2024-01-01, 2024-12-31) # 日期范围

# 有注释的记录
commented`,
};
