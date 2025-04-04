use std::io::{self, Write};
use std::rc::Rc;

use instant::{Duration, Instant};
use web_sys::HtmlCanvasElement;

use crate::options::{AnalysisOption, StatsType};
use crate::record::Record;
use crate::session::Session;
use crate::time::HumanReadable;

/// Returns the letter 's' for a plural form
/// in en-us, or nothing for a single form.
fn plural_form(count: usize) -> String {
    if count == 1 {
        String::new()
    } else {
        String::from("s")
    }
}

/// Appends a markdown heading with specified level.
fn append_heading<W: Write>(writer: &mut W, title: &str, level: usize) -> io::Result<()> {
    writeln!(writer, "{} {}\n", "#".repeat(level), title)
}

/// Appends information of the dataset and parsed options.
fn append_analysis_info<W: Write>(
    writer: &mut W,
    sessions: &[Session],
    options: &[AnalysisOption],
) -> io::Result<bool> {
    append_heading(writer, "Dataset", 3)?;

    if sessions.is_empty() {
        writeln!(writer, "No session parsed successfully.\n")?;
        return Ok(true);
    }

    let session_count = sessions.len();
    let record_count = sessions.iter().map(|s| s.records().len()).sum::<usize>();

    writeln!(
        writer,
        "Parsed `{}` session{} (`{}` record{}).\n",
        session_count,
        plural_form(session_count),
        record_count,
        plural_form(record_count),
    )?;

    for session in sessions {
        writeln!(writer, "- [{}](#session-{})", session, session.rank(),)?;
    }
    writeln!(writer)?;

    append_heading(writer, "Analysis Options", 3)?;

    if options.is_empty() {
        writeln!(writer, "No option parsed successfully.\n",)?;
        return Ok(true);
    }

    let option_count = options.len();
    writeln!(
        writer,
        "Parsed `{}` option{} (failures ignored and duplicates removed).\n",
        option_count,
        plural_form(option_count)
    )?;

    for option in options {
        writeln!(writer, "- {}", option)?;
    }
    writeln!(writer)?;

    Ok(false)
}

/// Appends the start and end `date_times` of a session.
fn append_session_date_time<W: Write>(writer: &mut W, session: &Session) -> io::Result<()> {
    let (start, end) = session.date_time();
    let days = session.days_with_record();

    writeln!(
        writer,
        "- {} ~ {}\n- {} day{} practiced\n",
        start.to_string().strip_suffix(" UTC").unwrap_or_default(),
        end.to_string().strip_suffix(" UTC").unwrap_or_default(),
        days,
        plural_form(days)
    )
}

/// Appends the details of some `Record`s, a HTML collapsible
/// element is added when there are more than one `Record`.
fn append_records_detail<W: Write>(
    writer: &mut W,
    records: &[(usize, Rc<Record>)],
) -> io::Result<()> {
    writeln!(writer, "[#{}] {}", records[0].0, records[0].1)?;

    if records.len() > 1 {
        writeln!(writer, "<details>\n<summary>... more records</summary>\n")?;
        for pair in records.iter().skip(1) {
            writeln!(writer, "[#{}] {}", pair.0, pair.1)?;
        }
        writeln!(writer, "</details>\n")?;
    }

    Ok(())
}

/// Appends two tables in the summary section.
fn append_summary_table<W: Write>(writer: &mut W, session: &Session) -> io::Result<()> {
    let (best, worst, mean, average) = session.summary();
    let summary = format!(
        r"| best | worst | mean | average |
| :-: | :-: | :-: | :-: |
| `{}` | `{}` | `{}` | `{}` |",
        best, worst, mean, average,
    );

    let (ok, plus2, dnf) = session.solve_states();
    let record_count = session.records().len() as f64;
    let solve_states = format!(
        r"| Ok | +2 | DNF |
| :-: | :-: | :-: |
| `{}` | `{}` `({:.2}%)` | `{}` `({:.2}%)` |",
        ok,
        plus2,
        (plus2 * 100) as f64 / record_count,
        dnf,
        (dnf * 100) as f64 / record_count
    );

    writeln!(writer, "{}\n\n{}\n", summary, solve_states)
}

/// Appends a quote.
fn append_message<W: Write>(writer: &mut W, callout_type: &str, content: &str) -> io::Result<()> {
    writeln!(writer, "> **{}**: {}\n", callout_type, content)
}

/// Appends an image data url.
fn append_image_data_url<W: Write>(writer: &mut W, canvas: &HtmlCanvasElement) -> io::Result<()> {
    writeln!(
        writer,
        "![Chart]({})\n",
        canvas.to_data_url().unwrap_or_default()
    )
}

/// Appends debug information of analysis timings.
fn append_timings<W: Write>(
    writer: &mut W,
    parsing_time: Duration,
    timings: &[(usize, Duration)],
    overall_time: Duration,
) -> io::Result<()> {
    append_heading(writer, "Timings", 3)?;

    writeln!(writer, "- Data parsing: {:.1?}", parsing_time)?;
    writeln!(writer, "- Analyzing: {:.1?}", overall_time)?;
    for (rank, timing) in timings {
        writeln!(writer, "\t- Session [#{}]: {:.1?}", rank, timing)?;
    }

    Ok(())
}

/// Appends an analysis section.
fn append_section<W: Write>(
    writer: &mut W,
    session: &Session,
    op: &AnalysisOption,
    canvas: &HtmlCanvasElement,
) -> io::Result<()> {
    append_heading(writer, &format!("{}", op), 4)?;

    if let Some(s_type) = op.stats_type() {
        let s_scale = s_type.scale();
        if session.records().len() < s_scale {
            return append_message(
                writer,
                "INFO",
                &format!("Records not enough for {}.", s_type),
            );
        }
    }

    match op {
        AnalysisOption::Summary => append_summary_table(writer, session),

        AnalysisOption::Pbs(stats_type) => {
            let pbs = session.pbs(stats_type);

            if pbs.is_empty() {
                append_message(
                    writer,
                    "INFO",
                    &format!("No PB histories of {}.", stats_type),
                )
            } else {
                let (first_pb, last_pb) = (
                    pbs[0].1.to_readable_string(),
                    pbs[pbs.len() - 1].1.to_readable_string(),
                );
                let pbs_desc = pbs
                    .iter()
                    .map(|pair| pair.1.to_readable_string())
                    .collect::<Vec<_>>()
                    .join(" -> ");

                writeln!(
                    writer,
                    r"<details>
<summary><code>{} -> {}</code></summary>

```
{}
```

</details>
",
                    first_pb, last_pb, pbs_desc
                )?;

                if matches!(stats_type, StatsType::Single) {
                    append_records_detail(
                        writer,
                        &pbs.iter().map(|r| (r.0, r.2.clone())).collect::<Vec<_>>(),
                    )?;
                }

                Ok(())
            }
        }

        AnalysisOption::Group(s_type, interval) => {
            let groups = session.group(*interval, s_type);

            match session.draw_grouping(canvas, &groups, *interval, &s_type.to_string()) {
                Ok(()) => append_image_data_url(writer, canvas),
                Err(e) => append_message(
                    writer,
                    "ERROR",
                    &format!("Generating grouping chart failed: {}.", e),
                ),
            }
        }

        AnalysisOption::Trend(stats_type) => {
            let data = session.trend(stats_type);

            match session.draw_trending(canvas, &data, &stats_type.to_string()) {
                Ok(()) => {
                    append_image_data_url(writer, canvas)?;
                    append_message(writer, "TIPS", "DNF & N/A are treated as empty points.")
                }
                Err(e) => append_message(
                    writer,
                    "ERROR",
                    &format!("Generating trending chart failed: {}.", e),
                ),
            }
        }

        AnalysisOption::Commented => {
            let commented = session.commented_records();

            if commented.is_empty() {
                append_message(writer, "INFO", "No commented record.")
            } else {
                append_records_detail(writer, &commented)
            }
        }
    }
}

/// Analyzes each session with parsed options.
pub fn analyze<W: Write>(
    sessions: &[Session],
    options: &[AnalysisOption],
    writer: &mut W,
    canvas: &HtmlCanvasElement,
    parsing_time: Duration,
) -> io::Result<()> {
    let analysis_timer = Instant::now();

    let empty = append_analysis_info(writer, sessions, options)?;
    if empty {
        return Ok(());
    }

    let mut timings = Vec::with_capacity(sessions.len());

    for session in sessions {
        let session_timer = Instant::now();

        append_heading(
            writer,
            &format!("<a id=\"session-{}\">{}</a>", session.rank(), session),
            3,
        )?;
        append_session_date_time(writer, session)?;

        if session.records_not_dnf().is_empty() {
            append_message(writer, "Info", "Every record is DNF.")?;
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
