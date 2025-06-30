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

/// Writes a markdown heading with the specified level.
fn write_heading<W: Write>(writer: &mut W, level: usize, title: &str) -> io::Result<()> {
    writeln!(writer, "{} {}\n", "#".repeat(level), title)
}

/// Writes information about the dataset and parsed options.
pub fn write_analysis_info<W: Write>(
    writer: &mut W,
    sessions: &[Session],
    options: &[AnalysisOption],
) -> io::Result<bool> {
    write_heading(writer, 3, &t!("title.dataset"))?;

    if sessions.is_empty() {
        let info = t!("info.no-session-parsed");
        writeln!(writer, "{}\n", info)?;
        return Ok(true);
    }

    let session_count = sessions.len();
    let record_count = sessions
        .iter()
        .map(super::session::Session::record_count)
        .sum::<usize>();

    let session_info = t!(
        "session.info",
        session_count = session_count,
        session_count_plural = plural_form(session_count),
        record_count = record_count,
        record_plural = t!(
            "record.plural",
            record_count_plural = plural_form(record_count)
        ),
    );
    writeln!(writer, "{}\n", session_info)?;

    for session in sessions {
        writeln!(
            writer,
            "- [[#{}] **{}** (`{}` {})](#session{})",
            session.rank(),
            session.name(),
            session.record_count(),
            t!(
                "record.plural",
                record_count_plural = plural_form(record_count)
            ),
            session.rank(),
        )?;
    }
    writeln!(writer)?;

    write_heading(writer, 3, &t!("title.analysis-options"))?;

    if options.is_empty() {
        let info = t!("info.no-option-parsed");
        writeln!(writer, "{}\n", info)?;
        return Ok(true);
    }

    let option_count = options.len();
    let option_info = t!(
        "option.info",
        option_count = option_count,
        option_count_plural = plural_form(option_count)
    );
    writeln!(writer, "{}\n", option_info)?;

    for option in options {
        writeln!(writer, "- {}", option)?;
    }
    writeln!(writer)?;

    Ok(false)
}

/// Writes information about days practiced on a session.
fn write_session_date_time<W: Write>(writer: &mut W, session: &Session) -> io::Result<()> {
    let (start, end) = session.date_time();
    let (start, end) = (start.date_naive(), end.date_naive());
    let days = session.days_with_record();
    let total_days = end.signed_duration_since(start).num_days() as usize + 1;

    let t_days_total = t!(
        "session.days-total",
        start = start,
        end = end,
        total_days = total_days,
        total_days_plural = plural_form(total_days),
    );
    let t_days_practiced = t!(
        "session.days-practiced",
        days = days,
        days_plural = plural_form(days),
        percentage = format!("{:.1}%", percentage(days, total_days)),
        total_days = total_days
    );

    writeln!(writer, "- {}\n- {}\n", t_days_total, t_days_practiced)
}

/// Writes the details of some `Record`s, a HTML collapsible
/// element will be added when there are more than one `Record`.
fn write_records_detail<W: Write>(
    writer: &mut W,
    records: &[(usize, Rc<Record>)],
) -> io::Result<()> {
    writeln!(writer, "[#{}] {}", records[0].0, records[0].1)?;

    if records.len() > 1 {
        writeln!(
            writer,
            "<details>\n<summary>... {}</summary>\n",
            t!("session.more-records")
        )?;
        for pair in records.iter().skip(1) {
            writeln!(writer, "[#{}] {}", pair.0, pair.1)?;
        }
        writeln!(writer, "</details>\n")?;
    }

    Ok(())
}

/// Writes two tables in the summary section.
fn write_summary_table<W: Write>(writer: &mut W, session: &Session) -> io::Result<()> {
    let (best, worst, mean, average) = session.summary();
    let summary = format!(
        r"| {} | {} | {} | {} |
| :-: | :-: | :-: | :-: |
| `{}` | `{}` | `{}` | `{}` |",
        t!("stats.best"),
        t!("stats.worst"),
        t!("stats.mean"),
        t!("stats.average"),
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

/// Writes a quote with a label and a message.
fn write_message<W: Write>(writer: &mut W, label: &str, content: &str) -> io::Result<()> {
    let cs = t!("colon-space");
    writeln!(writer, "> **{}**{cs}{}\n", label, content)
}

/// Writes an image data url.
fn write_image_data_url<W: Write>(
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

/// Writes debug information about analysis timings.
pub fn write_timings<W: Write>(
    writer: &mut W,
    parsing_time: Duration,
    timings: &[(usize, Duration)],
    overall_time: Duration,
) -> io::Result<()> {
    let cs = t!("colon-space");

    write_heading(writer, 3, &t!("title.timings"))?;

    writeln!(
        writer,
        "- {}{cs}{:.1?}",
        t!("timings.data-parsing"),
        parsing_time
    )?;
    writeln!(
        writer,
        "- {}{cs}{:.1?}",
        t!("timings.analyzing"),
        overall_time
    )?;
    for (rank, timing) in timings {
        writeln!(
            writer,
            "\t- {} [#{}]{cs}{:.1?}",
            t!("timings.session"),
            rank,
            timing
        )?;
    }

    Ok(())
}

/// Writes an analysis section.
fn write_section<W: Write>(
    writer: &mut W,
    session: &Session,
    op: &AnalysisOption,
    canvas: &HtmlCanvasElement,
) -> io::Result<()> {
    write_heading(writer, 4, &format!("{}", op))?;

    if let Some(s_type) = op.stats_type() {
        let s_scale = s_type.scale();
        if session.record_count() < s_scale {
            return write_message(
                writer,
                &t!("label.info"),
                &t!("info.records-not-enough", s_type = s_type),
            );
        }
    }

    match op {
        AnalysisOption::Summary => write_summary_table(writer, session),

        AnalysisOption::Pbs(s_type) => {
            let pbs = session.pbs(s_type);

            if pbs.is_empty() {
                return write_message(
                    writer,
                    &t!("label.info"),
                    &t!("info.no-pb-history", s_type = s_type),
                );
            }

            let (first_pb, last_pb) = (pbs[0].1, pbs[pbs.len() - 1].1);
            let pb_count = pbs.len();
            let pbs_desc = pbs
                .iter()
                .map(|pair| pair.1.to_readable_string())
                .collect::<Vec<_>>()
                .join(" -> ");

            writeln!(
                writer,
                r"<details>
<summary><code>{} -> {}</code> {}</summary>

```
{}
```

</details>
",
                first_pb.to_readable_string(),
                last_pb.to_readable_string(),
                t!(
                    "stats.pbs",
                    pb_count = pb_count,
                    pb_count_plural = plural_form(pb_count),
                ),
                pbs_desc,
            )?;

            if pb_count > 1 {
                let cs = t!("colon-space");
                let trends = session.pbs_trends(&pbs);
                let desc = format!("{}{cs}{} {}", session, s_type, t!("stats.pbs-desc"));
                match session.draw_trending(canvas, &trends, &desc) {
                    Ok(()) => write_image_data_url(writer, canvas, &desc)?,
                    Err(e) => write_message(
                        writer,
                        &t!("label.error"),
                        &t!("error.trending-chart-fail", error_info = e),
                    )?,
                }
            }

            if matches!(s_type, StatsType::Single) {
                write_records_detail(
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
            let mut interval = *interval;
            if interval == 0 {
                interval = session.decide_interval();
            }

            let groups = session.group(interval, s_type);

            let cs = t!("colon-space");
            let desc = format!(
                "{}{cs}{} {} {}",
                session,
                s_type,
                t!("stats.groups"),
                t!("stats.groups-interval", interval = interval.as_seconds()),
            );

            match session.draw_grouping(canvas, &groups, interval, &desc) {
                Ok(()) => write_image_data_url(writer, canvas, &desc),
                Err(e) => write_message(
                    writer,
                    &t!("label.error"),
                    &t!("error.grouping-chart-fail", error_info = e),
                ),
            }
        }

        AnalysisOption::Trend(s_type) => {
            let trends = session.trend(s_type);

            if trends.iter().all(|p| p.1 == 0) {
                return write_message(writer, &t!("label.info"), &t!("info.all-dnf"));
            }

            let cs = t!("colon-space");
            let desc = format!("{}{cs}{} {}", session, s_type, t!("stats.trends"));

            match session.draw_trending(canvas, &trends, &desc) {
                Ok(()) => {
                    write_image_data_url(writer, canvas, &desc)?;
                    write_message(writer, &t!("label.tips"), &t!("info.empty-points"))
                }
                Err(e) => write_message(
                    writer,
                    &t!("label.error"),
                    &t!("error.trending-chart-fail", error_info = e),
                ),
            }
        }

        AnalysisOption::Recent(target) => match session.try_from_target_range(target) {
            Some(sub_session) => {
                if sub_session.records_not_dnf().is_empty() {
                    return write_message(writer, &t!("label.info"), &t!("info.all-dnf"));
                }

                let record_count = sub_session.record_count();
                let t_recent_record_count = t!(
                    "stats.recent-record-count",
                    record_count = record_count,
                    record_count_plural = plural_form(record_count)
                );
                writeln!(writer, "{}\n", t_recent_record_count)?;
                write_summary_table(writer, &sub_session)
            }
            None => write_message(writer, &t!("label.info"), &t!("info.no-recent-record")),
        },

        AnalysisOption::Commented => {
            let commented = session.commented_records();

            if commented.is_empty() {
                write_message(writer, &t!("label.info"), &t!("info.no-commented-record"))
            } else {
                write_records_detail(writer, &commented)
            }
        }
    }
}

/// Analyzes a single session with parsed options.
pub fn analyze_session<W: Write>(
    session: &Session,
    options: &[AnalysisOption],
    writer: &mut W,
    canvas: &HtmlCanvasElement,
) -> io::Result<Duration> {
    let session_timer = Instant::now();

    let record_count = session.record_count();
    let session_heading = format!(
        "<a id=\"session{}\">[#{}] **{}** (`{}` {})</a>",
        session.rank(),
        session.rank(),
        session.name(),
        record_count,
        t!(
            "record.plural",
            record_count_plural = plural_form(record_count)
        ),
    );
    write_heading(writer, 3, &session_heading)?;
    write_session_date_time(writer, session)?;

    if session.records_not_dnf().is_empty() {
        write_message(writer, &t!("label.info"), &t!("info.all-dnf"))?;
    } else {
        for a_type in options {
            write_section(writer, session, a_type, canvas)?;
        }
    }

    Ok(session_timer.elapsed())
}
