use std::io::{self, Write};

use web_sys::HtmlCanvasElement;

use crate::session::*;
use crate::types::*;

/// Appends the data writer path
/// and the parsed options.
fn append_analysis_info<W: Write>(
    writer: &mut W,
    sessions: &[Session],
    record_count: usize,
    options: &[Analyze],
) -> io::Result<()> {
    writeln!(writer, "### Dataset\n")?;
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
    writeln!(
        writer,
        "Successfully parsed `{}` options (failures ignored and duplicates removed).\n",
        options.len(),
    )?;
    for a_type in options.iter() {
        writeln!(writer, "- {}", a_type)?;
    }
    writeln!(writer)?;

    Ok(())
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

/// Appends the start and end date_times of a session.
fn append_session_date_time<W: Write>(writer: &mut W, session: &Session) -> io::Result<()> {
    let (start, end) = session.date_time();
    writeln!(writer, "{} ~ {}\n", start, end)
}

/// Appends a quote.
fn append_message<W: Write>(writer: &mut W, callout_type: &str, content: String) -> io::Result<()> {
    writeln!(writer, "> **{}**: {}", callout_type, content)
}

/// Appends an image data url.
fn append_image_url<W: Write>(writer: &mut W, url: &str) -> io::Result<()> {
    writeln!(writer, "![Generated Chart]({})\n", url)
}

/// Converts canvas to data url.
fn canvas_to_data_url(canvas: &HtmlCanvasElement) -> String {
    canvas.to_data_url().unwrap_or_else(|_| String::from(""))
}

/// Appends an analysis section.
fn append_section<W: Write>(
    writer: &mut W,
    session: &Session,
    a_type: &Analyze,
    canvas: &HtmlCanvasElement,
) -> io::Result<()> {
    write!(writer, "#### ")?;

    match a_type {
        Analyze::Overview => {
            let (best, worst, mean, average) = session.overview();
            let overview = format!(
                r#"| best | worst | mean | avg |
| --- | --- | --- | --- |
| `{}` | `{}` | `{}` | `{}` |"#,
                best, worst, mean, average,
            );
            let (ok, plus2, dnf) = session.solve_states();
            let total = session.records().len() as f64;
            let solve_states = format!(
                r#"| Ok | +2 | DNF |
| --- | --- | --- |
| `{}` | `{}` `({:.2}%)` | `{}` `({:.2}%)` |"#,
                ok,
                plus2,
                (plus2 * 100) as f64 / total,
                dnf,
                (dnf * 100) as f64 / total
            );
            writeln!(writer, "Overview\n\n{}\n\n{}\n", overview, solve_states)
        }

        Analyze::PbHistory(stats_type) => {
            writeln!(writer, "**{}** PB History\n", stats_type)?;

            if let Some(pairs) = session.pb_breakers(stats_type) {
                if !pairs.is_empty() {
                    let pb_history = pairs
                        .iter()
                        .map(|p| p.1.readable())
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
                } else {
                    append_message(
                        writer,
                        "error",
                        format!("NO PB HISTORIES OF **{}**.", stats_type),
                    )
                }
            } else {
                append_message(
                    writer,
                    "error",
                    format!("NOT ENOUGH RECORDS FOR **{}** STATISTICS.", stats_type),
                )
            }
        }

        Analyze::Grouping(stats_type, interval) => {
            writeln!(
                writer,
                "**{}** Grouping by {}s\n",
                stats_type,
                *interval as f32 / 1000.0
            )?;

            if let Some(groups) = session.try_grouping(*interval) {
                let desc = stats_type.to_string();

                match session.draw_grouping(canvas, groups, *interval, &desc) {
                    Ok(()) => {
                        let data_url = canvas_to_data_url(canvas);
                        append_image_url(writer, &data_url)
                    }
                    Err(e) => append_message(
                        writer,
                        "error",
                        format!("GROUPING BY {}s FAILED: {}.", interval, e),
                    ),
                }
            } else {
                append_message(
                    writer,
                    "error",
                    format!("NO DATA FOR GROUPING BY {}s.", interval),
                )
            }
        }

        Analyze::Trending(stats_type) => {
            writeln!(writer, "**{}** Trending\n", stats_type)?;
            if let Some(data) = session.trend(stats_type) {
                let desc = stats_type.to_string();

                match session.draw_trending(canvas, data, &desc) {
                    Ok(()) => {
                        let data_url = canvas_to_data_url(canvas);
                        append_image_url(writer, &data_url)
                    }
                    Err(e) => append_message(
                        writer,
                        "error",
                        format!("SAVING TRENDING CHART FAILED: {}.", e),
                    ),
                }
            } else {
                append_message(
                    writer,
                    "error",
                    format!("NOT ENOUGH RECORDS FOR **{}** STATISTICS.", stats_type),
                )
            }
        }

        Analyze::Commented => {
            writeln!(writer, "Commented Records\n")?;

            let commented = session.commented_records();
            if !commented.is_empty() {
                writeln!(writer, "[#{}] {}", commented[0].0, commented[0].1)?;

                if commented.len() > 1 {
                    writeln!(writer, "<details>\n<summary>Expand</summary>\n")?;
                    for (i, r) in commented.iter().skip(1) {
                        writeln!(writer, "[#{}] {}", i, r)?;
                    }
                    writeln!(writer, "</details>\n")?;
                }
            } else {
                append_message(writer, "error", String::from("NO COMMENTED RECORD."))?;
            }

            Ok(())
        }
    }
}

/// Analyzes each session, using parsed options.
pub fn analyze<W: Write>(
    sessions: &[Session],
    options: &[Analyze],
    writer: &mut W,
    canvas: HtmlCanvasElement,
) -> io::Result<()> {
    append_analysis_info(
        writer,
        sessions,
        sessions.iter().map(|s| s.records().len()).sum::<usize>(),
        options,
    )?;

    for session in sessions {
        append_session_title(writer, session)?;
        append_session_date_time(writer, session)?;

        for a_type in options {
            append_section(writer, session, a_type, &canvas)?;
        }
    }

    Ok(())
}
