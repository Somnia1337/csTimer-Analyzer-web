use std::collections::HashSet;
use std::rc::Rc;

use chrono::NaiveDate;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;

use crate::options::StatsType;
use crate::record::{Record, SolveState};
use crate::session::{GroupTime, Session};
use crate::time::{AsSeconds, HumanReadable, Milliseconds, Seconds};

const CUT_OFF: f32 = 0.05;
const MAGNIFICATION: f32 = 1.05;
const MONOSPACE: &str = "JetBrains Mono, Consolas, Courier New";
const CAPTION_FONT_SIZE: i32 = 48;
const AXIS_DESC_FONT_SIZE: i32 = 40;
const LABEL_FONT_SIZE: i32 = 32;
const MARGIN: i32 = 20;
const LABEL_AREA_SIZE: i32 = 160;
const PLOT_COLOR: RGBColor = RGBColor(91, 169, 253);
const STROKE_WIDTH: u32 = 3;

/// The plain arithmetic mean over a sum of
/// Milliseconds, rounds at 1 millis.
fn round_mean(sum: Milliseconds, count: usize) -> Milliseconds {
    (sum as f32 / count as f32).round() as Milliseconds
}

impl Session {
    /// Best and worst solve times that are not DNF.
    fn best_and_worst(&self) -> (Milliseconds, Milliseconds) {
        self.records_not_dnf()
            .iter()
            .map(|r| r.time())
            .fold((u32::MAX, u32::MIN), |(best, worst), time| {
                (best.min(time), worst.max(time))
            })
    }

    /// Mean of solve times that are not DNF.
    fn mean(&self) -> Milliseconds {
        let sum: Milliseconds = self.records_not_dnf().iter().map(|r| r.time()).sum();

        round_mean(sum, self.record_not_dnf_count())
    }

    /// Count of `Record`s with the specified `SolveState`.
    fn count_solve_state(&self, is_state: &dyn Fn(SolveState) -> bool) -> usize {
        self.records()
            .iter()
            .filter(|r| is_state(r.solve_state()))
            .count()
    }

    /// Stats a single solve, or the mean or average over some solves.
    fn stats(&self, pos: usize, s_type: &StatsType) -> Option<Milliseconds> {
        match s_type {
            StatsType::Single => {
                let r = &self.records()[pos];

                if r.solve_state().is_dnf() {
                    None
                } else {
                    Some(r.time())
                }
            }

            StatsType::Mean(s_scale) => {
                let chunk = &self.records()[(pos + 1).saturating_sub(*s_scale)..=pos];

                if chunk.iter().any(|r| r.solve_state().is_dnf()) {
                    None
                } else {
                    Some(round_mean(chunk.iter().map(|r| r.time()).sum(), *s_scale))
                }
            }

            StatsType::Average(s_scale) => {
                let chunk = &self.records()[(pos + 1).saturating_sub(*s_scale)..=pos];
                let cut_off = (*s_scale as f32 * CUT_OFF).ceil() as usize;
                let take = s_scale.saturating_sub(cut_off * 2);

                if take == 0 || chunk.iter().filter(|r| r.solve_state().is_dnf()).count() > cut_off
                {
                    None
                } else {
                    let mut chunk: Vec<Milliseconds> = chunk
                        .iter()
                        .map(|r| {
                            if r.solve_state().is_dnf() {
                                u32::MAX
                            } else {
                                r.time()
                            }
                        })
                        .collect();
                    chunk.sort_unstable();

                    Some(round_mean(
                        chunk.iter().skip(cut_off).take(take).sum(),
                        take,
                    ))
                }
            }
        }
    }

    /// All non-DNF data of the specified `StatsType` over the `Session`.
    fn stats_data(&self, s_type: &StatsType) -> Vec<Milliseconds> {
        (0..self.record_count())
            .skip(s_type.scale() - 1)
            .filter_map(|i| self.stats(i, s_type))
            .collect()
    }
}

impl Session {
    /// Count of days that has at least a `Record` in the `Session`.
    pub fn days_with_record(&self) -> usize {
        self.records()
            .iter()
            .map(|r| r.date_time().date_naive())
            .collect::<HashSet<NaiveDate>>()
            .len()
    }

    /// The best, worst, mean and average solve times of the `Session`,
    /// where average could be DNF represented by `None`.
    pub fn summary(
        &self,
    ) -> (
        Milliseconds,
        Milliseconds,
        Milliseconds,
        Option<Milliseconds>,
    ) {
        let record_count = self.record_count();
        let (best, worst) = self.best_and_worst();
        let mean = self.mean();
        let average = self.stats(record_count - 1, &StatsType::Average(record_count));

        (best, worst, mean, average)
    }

    /// The counts of solve states of the `Session`.
    pub fn solve_states(&self) -> (usize, usize, usize) {
        (
            self.count_solve_state(&SolveState::is_ok),
            self.count_solve_state(&SolveState::is_plus2),
            self.count_solve_state(&SolveState::is_dnf),
        )
    }

    /// Every `Record` that breaked the personal best
    /// of the specified `StatsType`, with the new PB.
    pub fn pbs(&self, s_type: &StatsType) -> Vec<(usize, Milliseconds, Rc<Record>)> {
        let s_scale = s_type.scale();
        let mut pb = u32::MAX;
        let mut pbs = Vec::new();

        for (i, record) in self.records().iter().enumerate().skip(s_scale - 1) {
            if let Some(stats) = self.stats(i, s_type) {
                if stats < pb {
                    pb = stats;
                    pbs.push((i, pb, record.clone()));
                }
            }
        }

        pbs
    }

    /// Converts pbs into plots, prepare for trending image generation.
    pub fn pbs_to_plots(&self, pbs: &[(usize, Milliseconds, Rc<Record>)]) -> Vec<(usize, u32)> {
        let n = self.record_count();
        let mut plots: Vec<(usize, u32)> = (1..=n).map(|i| (i, 0)).collect();

        for window in pbs.windows(2) {
            let (start, pb) = (window[0].0, window[0].1);
            let end = window[1].0;
            plots[start..end].iter_mut().for_each(|p| p.1 = pb);
        }

        if let Some(&(last, pb, _)) = pbs.last() {
            plots[last..n].iter_mut().for_each(|p| p.1 = pb);
        }

        plots
    }

    /// Splits times of the specified `StatsType`
    /// into groups, by a fixed interval.
    pub fn group(&self, interval: Milliseconds, s_type: &StatsType) -> Vec<GroupTime> {
        let data = self.stats_data(s_type);
        let (mut min, mut max) = (
            data.iter().min().copied().unwrap_or_default(),
            data.iter().max().copied().unwrap_or_default(),
        );
        min = min / interval * interval;
        max = max.div_ceil(interval) * interval;

        let mut groups = Vec::with_capacity(((max - min) / interval + 1) as usize);

        for start in (min..max).step_by(interval as usize) {
            let record_count = data
                .iter()
                .filter(|t| **t >= start && **t < start + interval)
                .count();

            groups.push((start, record_count));
        }

        groups
    }

    /// A trend of time of the specified type over solves.
    pub fn trend(&self, s_type: &StatsType) -> Vec<(usize, u32)> {
        let s_scale = s_type.scale();
        let mut trends: Vec<(usize, u32)> = (0..self.record_count()).map(|i| (i + 1, 0)).collect();

        for (i, _) in self.records().iter().enumerate().skip(s_scale - 1) {
            trends[i].1 = self.stats(i, s_type).unwrap_or_default();
        }

        trends
    }

    /// Draws an image on canvas, visualizes grouping results.
    pub fn draw_grouping(
        &self,
        canvas: &HtmlCanvasElement,
        groups: &[GroupTime],
        interval: Milliseconds,
        desc: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let count_max = groups.iter().map(|g| g.1).max().unwrap_or_default();

        let (secs_min, secs_max) = (
            groups[0].0.as_seconds(),
            groups[groups.len() - 1].0.as_seconds(),
        );

        let root = CanvasBackend::with_canvas_object(canvas.clone())
            .ok_or("Failed to acquire canvas backend")?
            .into_drawing_area();
        root.fill(&WHITE)?;

        let caption = format!(
            "[#{}] {} {} GROUPS (by {}s)",
            self.rank(),
            self.name(),
            desc,
            interval.as_seconds(),
        );

        let x_spec = (secs_min.max(1.0) - 1.0)..(secs_max + 1.0);
        let y_spec = 0u32..(count_max as f32 * MAGNIFICATION) as u32;
        let mut chart = ChartBuilder::on(&root)
            .caption(caption, (MONOSPACE, CAPTION_FONT_SIZE).into_font())
            .margin(MARGIN)
            .x_label_area_size(LABEL_AREA_SIZE)
            .y_label_area_size(LABEL_AREA_SIZE)
            .build_cartesian_2d(x_spec, y_spec)?;

        chart
            .configure_mesh()
            .label_style((MONOSPACE, LABEL_FONT_SIZE).into_font())
            .axis_desc_style((MONOSPACE, AXIS_DESC_FONT_SIZE).into_font())
            .x_desc("Range / time")
            .y_desc("Count")
            .x_label_formatter(&Seconds::to_readable_string)
            .draw()?;

        let coords: Vec<(f32, u32)> = groups
            .iter()
            .map(|g| (g.0.as_seconds(), g.1 as u32))
            .collect();

        chart.draw_series(coords.iter().map(|(x, y)| {
            let x0 = *x;
            let x1 = x0 + interval.as_seconds();
            let y0 = 0;
            let y1 = *y;

            Rectangle::new([(x0, y0), (x1, y1)], PLOT_COLOR.filled())
        }))?;

        root.present()?;

        Ok(())
    }

    /// Draws an image on canvas, visualizes trending results.
    pub fn draw_trending(
        &self,
        canvas: &HtmlCanvasElement,
        plots: &[(usize, u32)],
        desc: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let real_point_segments = |times: &[(usize, u32)]| -> Vec<(usize, usize)> {
            let mut segments = Vec::new();
            let mut start = None;

            for (i, t) in times.iter().enumerate() {
                if t.1 > 0 {
                    if start.is_none() {
                        start = Some(i);
                    }
                } else if let Some(s) = start.take() {
                    segments.push((s, i));
                }
            }

            if let Some(s) = start {
                segments.push((s, times.len()));
            }

            segments
        };

        let n = self.record_count();
        let min = plots
            .iter()
            .map(|data| data.1)
            .filter(|t| *t > 0)
            .min()
            .unwrap_or_default();
        let max = plots.iter().map(|data| data.1).max().unwrap_or_default();
        let (min, max) = (min.as_seconds(), max.as_seconds());

        let root = CanvasBackend::with_canvas_object(canvas.clone())
            .ok_or("Failed to acquire canvas backend")?
            .into_drawing_area();
        root.fill(&WHITE)?;

        let caption = format!("[#{}] {} {} TRENDS", self.rank(), self.name(), desc,);

        let x_spec = 1..n + 1;
        let y_spec = min / MAGNIFICATION..max * MAGNIFICATION;
        let mut chart = ChartBuilder::on(&root)
            .caption(&caption, (MONOSPACE, CAPTION_FONT_SIZE).into_font())
            .margin(MARGIN)
            .x_label_area_size(LABEL_AREA_SIZE)
            .y_label_area_size(LABEL_AREA_SIZE)
            .build_cartesian_2d(x_spec, y_spec)?;

        chart
            .configure_mesh()
            .label_style((MONOSPACE, LABEL_FONT_SIZE).into_font())
            .axis_desc_style((MONOSPACE, AXIS_DESC_FONT_SIZE).into_font())
            .x_desc("Solves")
            .y_label_formatter(&Seconds::to_readable_string)
            .draw()?;

        for (start, end) in real_point_segments(plots) {
            chart.draw_series(LineSeries::new(
                (start..end).map(|i| (plots[i].0, plots[i].1.as_seconds())),
                PLOT_COLOR.stroke_width(STROKE_WIDTH),
            ))?;
        }

        root.present()?;

        Ok(())
    }

    /// `Record`s with a comment.
    pub fn commented_records(&self) -> Vec<(usize, Rc<Record>)> {
        self.records()
            .iter()
            .enumerate()
            .filter(|(_, r)| !r.comment().is_empty())
            .map(|(i, r)| (i + 1, Rc::clone(r)))
            .collect()
    }
}
