[English](https://github.com/Somnia1337/csTimer-Analyzer-web/blob/main/README.md) | 中文

## [csTimer 分析师(网页版)](https://somnia1337.github.io/csTimer-Analyzer-web/)

[csTimer 分析师](https://github.com/Somnia1337/csTimer-Analyzer) 的网页版，利用强大的 [WebAssembly](https://developer.mozilla.org/zh-CN/docs/WebAssembly) 和 [Rust](https://www.rust-lang.org/zh-CN) 工具链实现。

<div align=center>
  <img src="https://github.com/Somnia1337/csTimer-Analyzer-web/blob/main/assets/csTimer-Analyzer-ZH.png?raw=true" width="1280px">
</div>

功能特点：

- **安全**。你的数据将不会被上传，一切计算都由你的浏览器完成（托 WebAssembly 的福）。
- **快速**。能够在一秒内读取并分析数千条记录。
- **可配置**。分析器从网页的输入框读取配置，非常简单易用。
- **灵活**。分析器生成并渲染一个 Markdown 文件作为分析报告，你可以立即在浏览器中阅读报告。

### 使用方法

1. 访问 [csTimer](https://www.cstimer.net/)，点击“导出”按钮，然后点击“导出到文件”。一个名为 `cstimer_YYYYMMDD_hhmmss.txt` 的数据文件将会下载到本地。
2. 打开 [csTimer 分析师(网页版)](https://somnia1337.github.io/csTimer-Analyzer-web/)，在输入框中编写你的分析选项，配置教程见下文。
3. 点击网页的上传按钮，选择刚才下载的数据文件，浏览器将自动开始分析，随后展示分析报告。

### 分析选项

分析器会从输入框读取分析选项，以下是配置说明。

- `Overview`：提供分组的概览，包括最好和最差的时间、平均值、均值，和 `+2` 及 `DNF` 的次数统计。
- `stats`：统计的指标，从以下选项中选择一个：
  - `single`：单次成绩
  - `mo{n}`：`n` 次平均成绩
  - `ao{n}`：`n` 次去头去尾平均成绩
- `PbHistory(stats)`：指标 `stats` 的个人最佳成绩的历史记录。
- `Grouping(stats, millis)`：将指标 `stats` 以 `millis` 毫秒为间隔进行分组，生成直方图。
- `Trending(stats)`：跟踪指标 `stats` 的趋势，生成趋势图。
- `Commented`：筛选有注释的记录（如果你在跳 O / 跳 P 时写注释，这可能有帮助）。

下面是分析选项的实际示例。

```text
# 注释以 '#' 开头

# 分组概览
Overview

# 个人最佳成绩历史
PbHistory(single)
PbHistory(mo5)
PbHistory(ao50)

# 直方图
Grouping(single, 500)  # 500ms
Grouping(single, 1000) # 1000ms

# 趋势图
Trending(mo5)
Trending(ao200)

# 有注释的记录
Commented
```
