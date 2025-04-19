# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Support for incremental rendering of the analysis report: the UI now updates after each session is analyzed, instead of waiting for the entire report to complete.

## [0.8.3] - 2025-04-18

### Added

- A floating navigator to jump between sessions (`#4`).

### Changed

- Combine the back-to-top button into the navigator.

### Documentation

- Fixed heading levels in `feedback.md`.

## [0.8.2] - 2025-04-18

### Added

- A back-to-top button (`#5`).

### Fixed

- Selecting the same data file does not trigger a rerun. This is helpful when you modify the options and try to re-analyze the same data file.

### Documentation

- `feedback.md` documenting user feedbacks.

## [0.8.1] - 2025-04-14

### Changed

- A total refactor of JS scripts by Claude (I hate frontend).

## [0.8.0] - 2025-04-13

### Added

- Leverage browser local storage and IndexedDB to keep user-modified contents, this includes:
  - analysis options.
  - selected file / docs name.
  - rendered analysis report / docs.
- A button resets analysis options to default.

### Fixed

- Grouping chart have a y-margin of 0 with too few records.

## [0.7.8] - 2025-04-12

### Changed

- Update cargo deps.
- Use a button element for GitHub link.
- The selected file/docs name now displays in the file selection box element.
- Optimize JS scripts.
- Optimize CSS styles.

### Removed

- Current version display at the page title.
- The `file-selected` element and related CSS styles.

### Fixed

- Session name displays as "\*\*\*\*" when the name is empty (now it falls back to `rank`).
- Undesired behaviors when an invalid file is selected.

### Documentation

- Fix "upload" "上传" in READMEs since the user data literally never uploads.
- Auto-format READMEs and `CHANGELOG.md`.

## [0.7.7] - 2025-04-11

### Added

- A version bar on the top of the page, showing version histories.
- A new feedback solution powered by Tally.
- A button for changelog display.

### Changed

- Default analysis options descriptions.
  - Add a line "# The options for analysis | 分析选项".
  - Rewrite "Grouping histograms" as "Grouping charts" in en-us.
- Display all 3 markdown contents (README, Changelog, analysis report) in the same container.
  - It is now impossible to display docs (README, Changelog) while keeping the analysis report contents.
- Rename "analyzer.js" to "analyze.js".

### Removed

- The SMTP feedback solution since it's too erroneous.
- The modal element for displaying READMEs.

## [0.7.6] - 2025-04-09

### Changed

- Margins for charts.
- Descriptive texts for charts.
- Default analysis options.

## [0.7.5] - 2025-04-08

### Fixed

- `PBs` chart can be misleading.

## [0.7.4] - 2025-04-08

### Added

- Chart for `PBs`.

### Changed

- Introduce constants in `stats.rs` to clarify chart generators.
- Rename wasm entrance function to `wasm_analyze`.
- Accept code suggestions from Clippy.

### Documentation

- Update cover images.
- Add missing documentation for functions and traits.

## [0.7.3] - 2025-04-05

### Changed

- Encapsulate some redundant code.
- Accept code suggestions from Clippy.

## [0.7.2] - 2025-04-04

### Added

- Display the oldest and newest (slowest and fastest) PBs in `PBs`.

### Changed

- Tip for the collapsible element is now "... more records" instead of "Expand".

### Fixed

- Paths to fonts in `style.css`.

### Documentation

- Update `example.txt`.

## [0.7.1] - 2025-04-04

### Added

- Display of days actually practiced in a session.
- Auto line-wrapping for `PBs` code block.
- Various textual and visual tweaks.

### Changed

- Rename and reorganize modules.
- Move CSS styles into `style.css`.

### Fixed

- Duplicate analysis options not properly removed.
- `Session::group` not respecting the specified `StatsType`.

### Documentation

- Docs coverage for fields and functions reach 100%.
- "Todo" section in READMEs.

## [0.7.0] - 2025-04-01

### Changed

- Rename (shortened) some analysis options.
  - `Overview` is now `Summary`.
  - `PbHistory` is now `PBs`.
  - `Grouping` is now `Group`.
  - `Trending` is now `Trend`.

### Fixed

- Erroneous behaviors when stats results are empty.

### Security

- Eliminate bare `unwrap()`s.
- Provide more info when something goes wrong.

## [0.6.0] - 2025-03-31

### Added

- Treat DNFs as empty points in trending chart.
- "Timings" chapter, containing info about timings.

### Changed

- Rename and optimize functions.
- Accept code suggestions from Clippy.

### Fixed

- Average stats differ from csTimer.

## [0.5.3] - 2025-03-30

### Fixed

- Remove "UTC" after every date-time display, since it now respects local timezone.

## [0.5.2] - 2025-03-30

### Added

- The local time offset from UTC to every date-time, they now respect the local timezone.

## [0.5.1] - 2025-03-30

### Changed

- Contents in the table cells are now center aligned.

### Fixed

- Tables rendered as plaintext after replacing `marked.js`.

### Documentation

- Change paths to cover images in READMEs, using `./assets` instead of GitHub raw contents.

## [0.5.0] - 2025-03-30

### Changed

- Replace `marked.js` with Rust crate `pulldown-cmark`, improving rendering performance.

## [0.4.4] - 2025-03-30

### Added

- Generation timing info for each paragraph, session and the whole analysis.

## [0.4.3] - 2025-03-30

### Changed

- Update `favicon.ico`.

### Fixed

- Asterisks in comments can lead to undesired parsing results.

### Documentation

- Add cover images for the repo.
- Rename "分析器" to "分析师" in `README-ZH.md`.

## [0.4.2] - 2025-03-29

### Added

- Description for grouping chart.

## [0.4.1] - 2025-03-29

### Changed

- Replace "upload" with "select" in `index.html` since the user data literally never uploads.
- Move JS scripts into JS files under `./js`.
- Optimize CSS styles.

### Fixed

- Comments not redered as `<strong>` when there are padding whitespaces.
- Links to docs and `example.txt`.
- Headers in analysis report have wrong colors.
- Undesired behaviors while analyzing.
  - Cursor turns into a pointer.
  - Web page is horizontally draggable.

### Documentation

- Fix links to GitHub Pages.
- Time interval descriptions for `Grouping`.
- Update `example.txt`, adding aliases for sessions.

## [0.4.0] - 2025-03-28

### Added

- Use JSON instead of Regex for parsing data in different formats (like that of a bluetooth cube).
- Collapsible elements folding `PBs(single)` and `Commented` record details (since they can be way too long).
- Useful links to csTimer, READMEs and source repo.
- A feedback submission element.
- An example input textfile `example.txt`.

### Changed

- Rename `file.rs` to `json.rs` to better represent its functionalities.
- Rework fields and functions for `Session`.
- Swap the CSS styles of scramble and comment, with scramble being italic and comment being strong.
- Rename some functions in `impl Session`.
- Move icon files into `./assets`.

### Fixed

- Parsing bluetooth cube data fails.
- Time less than 10s differs from that in csTimer.
- Erroneous charts when the minimum time in a session is less than 1s.
- Time less than 1s displayed with redundant zeroes.

## [0.3.3] - 2025-03-23

### Removed

- All functions and types that are useless in web.
- `ObsidianFlavor` option since it's useless in web.

### Fixed

- Parameters passed to image generating functions.
- Link to `favicon.ico` in `index.html`.

## [0.3.2] - 2025-03-22

### Added

- Icon file `favicon.ico`.

## [0.3.1] - 2025-03-22

### Fixed

- Default options differ from docs.

## [0.3.0] - 2025-03-22

### Added

- HTML canvas element and JS scripts to port image generation feature.

### Documentation

- Add READMEs.

## [0.2.1] - 2025-03-22

### Fixed

- Rename wasm packages to match the project name.

## [0.2.0] - 2025-03-22

### Added

- The Rust source code.

## [0.1.2] - 2025-03-22

### Changed

- Improve CSS styles.

## [0.1.1] - 2025-03-22

### Added

- Simple CSS styles.

## [0.1.0] - 2025-03-22

### Added

- This project as the web port of `csTimer-Analyzer`, hopefully serve as a secure, fast, configurable, and flexible tool for analyzing cubing practice data from csTimer.
- A simple webpage interface `index.html`.
- wasm packages.
