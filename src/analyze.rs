use std::io::{self, Write};
use std::rc::Rc;

use instant::{Duration, Instant};
use web_sys::HtmlCanvasElement;

use crate::options::{AnalysisOption, StatsType};
use crate::record::Record;
use crate::session::Session;
use crate::time::{AsSeconds, HumanReadable};

/// Calculates a percentage.
fn percentage(count: usize, total: usize) -> f32 {
    if total > 0 {
        (count as f32 * 100.0) / total as f32
    } else {
        f32::NAN
    }
}

/// Returns the letter 's' for a plural form
/// in en-us, or nothing for a single form.
fn plural_form(count: usize) -> String {
    if count <= 1 {
        String::new()
    } else {
        String::from("s")
    }
}

/// Appends a markdown heading with the specified level.
fn append_heading<W: Write>(writer: &mut W, level: usize, title: &str) -> io::Result<()> {
    writeln!(writer, "{} {}\n", "#".repeat(level), title)
}

/// Appends information about the dataset and parsed options.
pub fn append_analysis_info<W: Write>(
    writer: &mut W,
    sessions: &[Session],
    options: &[AnalysisOption],
) -> io::Result<bool> {
    append_heading(writer, 3, "Dataset")?;

    if sessions.is_empty() {
        writeln!(writer, "No session parsed successfully.\n")?;
        return Ok(true);
    }

    let session_count = sessions.len();
    let record_count = sessions
        .iter()
        .map(super::session::Session::record_count)
        .sum::<usize>();

    writeln!(
        writer,
        "Parsed `{}` session{} (`{}` record{}).\n",
        session_count,
        plural_form(session_count),
        record_count,
        plural_form(record_count),
    )?;

    for session in sessions {
        writeln!(
            writer,
            "- [[#{}] **{}** (`{}` records)](#session{})",
            session.rank(),
            session.name(),
            session.record_count(),
            session.rank()
        )?;
    }
    writeln!(writer)?;

    append_heading(writer, 3, "Analysis Options")?;

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

/// Appends information about days practiced on a session.
fn append_session_date_time<W: Write>(writer: &mut W, session: &Session) -> io::Result<()> {
    let (start, end) = session.date_time();
    let (start, end) = (start.date_naive(), end.date_naive());
    let days = session.days_with_record();
    let total_days = end.signed_duration_since(start).num_days() + 1;

    writeln!(
        writer,
        "- {} ~ {} ({} days)\n- `{}` day{} actually practiced (`{:.1}%` out of {} days)\n",
        start,
        end,
        total_days,
        days,
        plural_form(days),
        percentage(days, total_days as usize),
        total_days,
    )
}

/// Appends the details of some `Record`s, a HTML collapsible
/// element will be added when there are more than one `Record`.
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
        best.to_readable_string(),
        worst.to_readable_string(),
        mean.to_readable_string(),
        average.map_or_else(|| String::from("DNF"), |avg| avg.to_readable_string())
    );

    let (ok, plus2, dnf) = session.solve_states();
    let record_count = session.record_count();
    let solve_states = format!(
        r"| OK | +2 | DNF |
| :-: | :-: | :-: |
| `{}` | `{}` `({:.2}%)` | `{}` `({:.2}%)` |",
        ok,
        plus2,
        percentage(plus2, record_count),
        dnf,
        percentage(dnf, record_count),
    );

    writeln!(writer, "{}\n\n{}\n", summary, solve_states)
}

/// Appends a quote with a label and a message.
fn append_message<W: Write>(writer: &mut W, label: &str, content: &str) -> io::Result<()> {
    writeln!(writer, "> **{}**: {}\n", label, content)
}

/// Appends an image data url.
fn append_image_data_url<W: Write>(
    writer: &mut W,
    canvas: &HtmlCanvasElement,
    desc: &str,
) -> io::Result<()> {
    writeln!(
        writer,
        "![{}]({})\n",
        desc,
        canvas.to_data_url().unwrap_or_default()
    )
}

/// Appends debug information about analysis timings.
pub fn append_timings<W: Write>(
    writer: &mut W,
    parsing_time: Duration,
    timings: &[(usize, Duration)],
    overall_time: Duration,
) -> io::Result<()> {
    append_heading(writer, 3, "Timings")?;

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
    append_heading(writer, 4, &format!("{}", op))?;

    if let Some(s_type) = op.stats_type() {
        let s_scale = s_type.scale();
        if session.record_count() < s_scale {
            return append_message(
                writer,
                "INFO",
                &format!("Records not enough for {}.", s_type),
            );
        }
    }

    match op {
        AnalysisOption::Summary => append_summary_table(writer, session),

        AnalysisOption::Pbs(s_type) => {
            let pbs = session.pbs(s_type);

            if pbs.is_empty() {
                return append_message(writer, "INFO", &format!("No PB histories of {}.", s_type));
            }

            let (first_pb, last_pb) = (pbs[0].1, pbs[pbs.len() - 1].1);
            let pbs_desc = pbs
                .iter()
                .map(|pair| pair.1.to_readable_string())
                .collect::<Vec<_>>()
                .join(" -> ");

            writeln!(
                writer,
                r"<details>
<summary><code>{} -> {}</code> (<code>{}</code> PBs)</summary>

```
{}
```

</details>
",
                first_pb.to_readable_string(),
                last_pb.to_readable_string(),
                pbs.len(),
                pbs_desc,
            )?;

            if pbs.len() > 1 {
                let trends = session.pbs_trends(&pbs);
                let desc = format!("{}: {} PBs", session, s_type);
                match session.draw_trending(canvas, &trends, &desc) {
                    Ok(()) => append_image_data_url(writer, canvas, &desc)?,
                    Err(e) => append_message(
                        writer,
                        "ERROR",
                        &format!("Generating trending chart failed: {}.", e),
                    )?,
                }
            }

            if matches!(s_type, StatsType::Single) {
                append_records_detail(
                    writer,
                    &pbs.iter()
                        .map(|r| (r.0 + 1, r.2.clone()))
                        .collect::<Vec<_>>(),
                )
            } else {
                Ok(())
            }
        }

        AnalysisOption::Group(s_type, interval) => {
            let groups = session.group(*interval, s_type);

            let desc = format!(
                "{}: {} GROUPS (by {}s)",
                session,
                s_type,
                interval.as_seconds()
            );
            match session.draw_grouping(canvas, &groups, *interval, &desc) {
                Ok(()) => append_image_data_url(writer, canvas, &desc),
                Err(e) => append_message(
                    writer,
                    "ERROR",
                    &format!("Generating grouping chart failed: {}.", e),
                ),
            }
        }

        AnalysisOption::Trend(s_type) => {
            let trends = session.trend(s_type);

            if trends.iter().all(|p| p.1 == 0) {
                return append_message(writer, "INFO", &format!("Every {} is DNF.", s_type));
            }

            let desc = format!("{}: {} TRENDS", session, s_type);
            match session.draw_trending(canvas, &trends, &desc) {
                Ok(()) => {
                    append_image_data_url(writer, canvas, &desc)?;
                    append_message(writer, "TIPS", "DNF & N/A are treated as empty points.")
                }
                Err(e) => append_message(
                    writer,
                    "ERROR",
                    &format!("Generating trending chart failed: {}.", e),
                ),
            }
        }

        AnalysisOption::Recent(target) => match session.try_from_target_range(target) {
            Some(sub_session) => {
                if sub_session.records_not_dnf().is_empty() {
                    return append_message(writer, "Info", "Every record is DNF.");
                }

                let record_count = sub_session.record_count();
                writeln!(
                    writer,
                    "`{}` record{} within this range.\n",
                    record_count,
                    plural_form(record_count)
                )?;
                append_summary_table(writer, &sub_session)
            }
            None => append_message(writer, "INFO", "No records within this range."),
        },

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

/// Analyzes a single session with parsed options.
pub fn analyze_single_session<W: Write>(
    session: &Session,
    options: &[AnalysisOption],
    writer: &mut W,
    canvas: &HtmlCanvasElement,
) -> io::Result<Duration> {
    let session_timer = Instant::now();

    let session_heading = format!(
        "<a id=\"session{}\">[#{}] **{}** (`{}` records)</a>",
        session.rank(),
        session.rank(),
        session.name(),
        session.record_count(),
    );
    append_heading(writer, 3, &session_heading)?;
    append_session_date_time(writer, session)?;

    if session.records_not_dnf().is_empty() {
        append_message(writer, "Info", "Every record is DNF.")?;
    } else {
        for a_type in options {
            append_section(writer, session, a_type, canvas)?;
        }
    }

    Ok(session_timer.elapsed())
}
