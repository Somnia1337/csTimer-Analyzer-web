use std::io::{self, Write};

use instant::{Duration, Instant};
use web_sys::HtmlCanvasElement;

use crate::options::{AnalysisOption, StatsType};
use crate::session::Session;
use crate::types::TimeReadable;

/// Appends the data writer path
/// and the parsed options.
fn append_analysis_info<W: Write>(
    writer: &mut W,
    sessions: &[Session],
    record_count: usize,
    options: &[AnalysisOption],
) -> io::Result<bool> {
    writeln!(writer, "### Dataset\n")?;

    if sessions.is_empty() {
        writeln!(writer, "No session parsed successfully.\n")?;
        return Ok(true);
    }

    writeln!(
        writer,
        "Parsed `{}` sessions (`{}` records).\n",
        sessions.len(),
        record_count
    )?;

    for session in sessions {
        writeln!(
            writer,
            "- [#[{}] {} (`{}` records)](#session-{})",
            session.rank(),
            session.name(),
            session.records().len(),
            session.rank(),
        )?;
    }
    writeln!(writer)?;

    writeln!(writer, "### Analysis Options\n")?;
    if options.is_empty() {
        writeln!(writer, "No option parsed successfully.\n",)?;
        return Ok(true);
    }

    let n = options.len();
    writeln!(
        writer,
        "Successfully parsed `{}` option{} (failures ignored and duplicates removed).\n",
        n,
        if n == 1 { "" } else { "s" }
    )?;

    for a_type in options {
        writeln!(writer, "- {}", a_type)?;
    }
    writeln!(writer)?;

    Ok(false)
}

/// Appends session as a title.
fn append_session_title<W: Write>(writer: &mut W, session: &Session) -> io::Result<()> {
    writeln!(
        writer,
        "### <a id=\"session-{}\"></a>{}\n",
        session.rank(),
        session,
    )
}

/// Appends the start and end `date_times` of a session.
fn append_session_date_time<W: Write>(writer: &mut W, session: &Session) -> io::Result<()> {
    let (start, end) = session.date_time();
    writeln!(
        writer,
        "{} ~ {}\n",
        start.to_string().strip_suffix(" UTC").unwrap_or_default(),
        end.to_string().strip_suffix(" UTC").unwrap_or_default(),
    )
}

/// Appends a quote.
fn append_message<W: Write>(writer: &mut W, callout_type: &str, content: &str) -> io::Result<()> {
    writeln!(writer, "> **{}**: {}\n", callout_type, content)
}

/// Appends an image data url.
fn append_image_url<W: Write>(writer: &mut W, url: &str) -> io::Result<()> {
    writeln!(writer, "![Generated Chart]({})\n", url)
}

/// Converts canvas to data url.
fn canvas_to_data_url(canvas: &HtmlCanvasElement) -> String {
    canvas.to_data_url().unwrap_or_else(|_| String::new())
}

/// Appends an analysis timing section.
fn append_timings<W: Write>(
    writer: &mut W,
    parsing_time: Duration,
    timings: &[(usize, Duration)],
    overall_time: Duration,
) -> io::Result<()> {
    writeln!(writer, "### Debug Info\n")?;

    writeln!(writer, "- Parsing data: {:.01?}", parsing_time)?;
    writeln!(writer, "- Analyzing: {:.01?}", overall_time)?;
    for (rank, timing) in timings {
        writeln!(writer, "\t- Session [#{}]: {:.01?}", rank, timing)?;
    }

    Ok(())
}

/// Appends an analysis section.
fn append_section<W: Write>(
    writer: &mut W,
    session: &Session,
    a_type: &AnalysisOption,
    canvas: &HtmlCanvasElement,
) -> io::Result<()> {
    write!(writer, "#### ")?;

    match a_type {
        AnalysisOption::Summary => {
            let (best, worst, mean, average) = session.summary();
            let summary = format!(
                r"| best | worst | mean | avg |
| :-: | :-: | :-: | :-: |
| `{}` | `{}` | `{}` | `{}` |",
                best, worst, mean, average,
            );
            let (ok, plus2, dnf) = session.solve_states();
            let total = session.records().len() as f64;
            let solve_states = format!(
                r"| Ok | +2 | DNF |
| :-: | :-: | :-: |
| `{}` | `{}` `({:.2}%)` | `{}` `({:.2}%)` |",
                ok,
                plus2,
                (plus2 * 100) as f64 / total,
                dnf,
                (dnf * 100) as f64 / total
            );
            writeln!(writer, "{}\n\n{}\n\n{}\n", a_type, summary, solve_states)
        }

        AnalysisOption::Pbs(stats_type) => {
            writeln!(writer, "**{}** PB History\n", stats_type)?;

            if let Some(pairs) = session.pbs(stats_type) {
                if pairs.is_empty() {
                    append_message(
                        writer,
                        "error",
                        &format!("NO PB HISTORIES OF **{}**.", stats_type),
                    )
                } else {
                    let pb_history = pairs
                        .iter()
                        .map(|p| p.1.to_readable_string())
                        .collect::<Vec<_>>()
                        .join(" -> ");
                    writeln!(writer, "```\n{}\n```\n", pb_history)?;

                    if matches!(stats_type, StatsType::Single) {
                        writeln!(writer, "[#{}] {}", pairs[0].0, pairs[0].2)?;

                        if pairs.len() > 1 {
                            writeln!(writer, "<details>\n<summary>Expand</summary>\n")?;
                            for pair in pairs.iter().skip(1) {
                                writeln!(writer, "[#{}] {}", pair.0, pair.2)?;
                            }
                            writeln!(writer, "</details>\n")?;
                        }
                    }

                    Ok(())
                }
            } else {
                append_message(
                    writer,
                    "error",
                    &format!("NOT ENOUGH RECORDS FOR **{}** STATISTICS.", stats_type),
                )
            }
        }

        AnalysisOption::Group(stats_type, interval) => {
            writeln!(
                writer,
                "**{}** Grouping by {}s\n",
                stats_type,
                *interval as f32 / 1000.0
            )?;

            let groups = session.group(*interval);
            let desc = stats_type.to_string();

            match session.draw_grouping(canvas, &groups, *interval, &desc) {
                Ok(()) => {
                    let data_url = canvas_to_data_url(canvas);
                    append_image_url(writer, &data_url)
                }
                Err(e) => append_message(
                    writer,
                    "error",
                    &format!("GROUPING BY {}s FAILED: {}.", interval, e),
                ),
            }
        }

        AnalysisOption::Trend(stats_type) => {
            writeln!(writer, "**{}** Trending\n", stats_type)?;
            if let Some(data) = session.trend(stats_type) {
                let desc = stats_type.to_string();

                match session.draw_trending(canvas, &data, &desc) {
                    Ok(has_inconsistency) => {
                        let data_url = canvas_to_data_url(canvas);
                        append_image_url(writer, &data_url)?;

                        if has_inconsistency {
                            append_message(
                                writer,
                                "info",
                                "The inconsistencies are due to DNFs treated as empty points. 断点是由于 DNF 被绘制为空点。",
                            )
                        } else {
                            Ok(())
                        }
                    }
                    Err(e) => append_message(
                        writer,
                        "error",
                        &format!("SAVING TRENDING CHART FAILED: {}.", e),
                    ),
                }
            } else {
                append_message(
                    writer,
                    "error",
                    &format!("NOT ENOUGH RECORDS FOR **{}** STATISTICS.", stats_type),
                )
            }
        }

        AnalysisOption::Commented => {
            writeln!(writer, "Commented Records\n")?;

            let commented = session.commented_records();
            if commented.is_empty() {
                append_message(writer, "error", "NO COMMENTED RECORD.")?;
            } else {
                writeln!(writer, "[#{}] {}", commented[0].0, commented[0].1)?;

                if commented.len() > 1 {
                    writeln!(writer, "<details>\n<summary>Expand</summary>\n")?;
                    for (i, r) in commented.iter().skip(1) {
                        writeln!(writer, "[#{}] {}", i, r)?;
                    }
                    writeln!(writer, "</details>\n")?;
                }
            }

            Ok(())
        }
    }
}

/// Analyzes each session, using parsed options.
pub fn analyze<W: Write>(
    sessions: &[Session],
    options: &[AnalysisOption],
    writer: &mut W,
    canvas: &HtmlCanvasElement,
    parsing_time: Duration,
) -> io::Result<()> {
    let analysis_timer = Instant::now();

    match append_analysis_info(
        writer,
        sessions,
        sessions.iter().map(|s| s.records().len()).sum::<usize>(),
        options,
    ) {
        Ok(empty) => {
            if empty {
                return Ok(());
            }
        }
        Err(e) => return Err(e),
    }

    let mut timings = Vec::with_capacity(sessions.len());

    for session in sessions {
        let session_timer = Instant::now();

        append_session_title(writer, session)?;
        append_session_date_time(writer, session)?;

        if session.non_dnf_records().is_empty() {
            append_message(writer, "error", "Every record is DNF")?;
        } else {
            for a_type in options {
                append_section(writer, session, a_type, canvas)?;
            }
        }

        timings.push((session.rank(), session_timer.elapsed()));
    }

    append_timings(writer, parsing_time, &timings, analysis_timer.elapsed())?;

    Ok(())
}
