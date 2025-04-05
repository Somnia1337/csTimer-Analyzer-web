use std::collections::HashSet;
use std::rc::Rc;

use chrono::NaiveDate;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;

use crate::options::StatsType;
use crate::record::{Record, SolveState};
use crate::session::{GroupTime, Session};
use crate::time::{HumanReadable, Milliseconds, Seconds, ToSeconds};

const CUT_OFF: f32 = 0.05;
const MAGNIFICATION: f32 = 1.1;

fn arithmetic_mean(sum: Milliseconds, count: usize) -> Milliseconds {
    (sum as f32 / count as f32).round() as Milliseconds
}

impl Session {
    /// Best and worst solve times that are not DNF in milliseconds.
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

        arithmetic_mean(sum, self.record_not_dnf_count())
    }

    /// Count of `Record`s that has the specified `SolveState`.
    fn count_solve_state(&self, is_state: &dyn Fn(SolveState) -> bool) -> usize {
        self.records()
            .iter()
            .filter(|r| is_state(r.solve_state()))
            .count()
    }

    /// Stats a single solve, or the mean or average over a chunk of solves.
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
                    Some(arithmetic_mean(
                        chunk.iter().map(|r| r.time()).sum(),
                        *s_scale,
                    ))
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

                    Some(arithmetic_mean(
                        chunk.iter().skip(cut_off).take(take).sum(),
                        take,
                    ))
                }
            }
        }
    }

    fn stats_data(&self, s_type: &StatsType) -> Vec<Milliseconds> {
        (0..self.record_count())
            .skip(s_type.scale() - 1)
            .filter_map(|i| self.stats(i, s_type))
            .collect()
    }
}

impl Session {
    /// Counts days that has at least a record in the session.
    pub fn days_with_record(&self) -> usize {
        self.records()
            .iter()
            .map(|r| r.date_time().date_naive())
            .collect::<HashSet<NaiveDate>>()
            .len()
    }

    /// The best, worst, mean and average solve times of a `Session`,
    /// where average could be `None` representing a DNF.
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

    /// The counts of solve states of a session.
    pub fn solve_states(&self) -> (usize, usize, usize) {
        (
            self.count_solve_state(&SolveState::is_ok),
            self.count_solve_state(&SolveState::is_plus2),
            self.count_solve_state(&SolveState::is_dnf),
        )
    }

    /// Every `Record` that breaked the personal best
    /// of the specified type, along with the new PB.
    pub fn pbs(&self, s_type: &StatsType) -> Vec<(usize, Milliseconds, Rc<Record>)> {
        let s_scale = s_type.scale();
        let mut pb = u32::MAX;
        let mut pbs = Vec::new();

        for (i, record) in self.records().iter().enumerate().skip(s_scale - 1) {
            if let Some(stats) = self.stats(i, s_type) {
                if stats < pb {
                    pb = stats;
                    pbs.push((i + 1, pb, record.clone()));
                }
            }
        }

        pbs
    }

    /// Splits times of the specified type into groups, by a fixed interval.
    pub fn group(&self, interval: Milliseconds, s_type: &StatsType) -> Vec<GroupTime> {
        let data = self.stats_data(s_type);
        let (mut min, mut max) = (
            data.iter().min().copied().unwrap_or_default(),
            data.iter().max().copied().unwrap_or_default(),
        );
        min = min / interval * interval;
        max = min + (max - min).div_ceil(interval) * interval;

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
    pub fn trend(&self, s_type: &StatsType) -> Vec<u32> {
        let s_scale = s_type.scale();
        let mut trends = vec![0; self.record_count()];

        for (i, _) in self.records().iter().enumerate().skip(s_scale - 1) {
            trends[i] = self.stats(i, s_type).unwrap_or_default();
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

        if count_max == 0 {
            return Err(Box::from("all groups are empty"));
        }

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

        let mut chart = ChartBuilder::on(&root)
            .caption(caption, ("Consolas", 48).into_font())
            .margin(20)
            .x_label_area_size(160)
            .y_label_area_size(160)
            .build_cartesian_2d(
                (secs_min.max(1.0) - 1.0)..(secs_max + 1.0),
                0u32..(count_max as f32 * MAGNIFICATION) as u32,
            )?;

        chart
            .configure_mesh()
            .label_style(("Consolas", 32).into_font())
            .axis_desc_style(("Consolas", 40).into_font())
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

            Rectangle::new([(x0, y0), (x1, y1)], RGBColor(91, 169, 253).filled())
        }))?;

        root.present()?;

        Ok(())
    }

    /// Draws an image on canvas, visualizes trending results.
    pub fn draw_trending(
        &self,
        canvas: &HtmlCanvasElement,
        times: &[u32],
        desc: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let real_point_segments = |times: &[u32]| -> Vec<(usize, usize)> {
            let mut segments = Vec::new();
            let mut start = None;

            for (i, t) in times.iter().enumerate() {
                if *t > 0 {
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

        let n = times.len();
        let real_point_count = times.iter().filter(|x| **x > 0).count();
        let max = times.iter().max().copied().unwrap_or_default();

        let root = CanvasBackend::with_canvas_object(canvas.clone())
            .ok_or("Failed to acquire canvas backend")?
            .into_drawing_area();
        root.fill(&WHITE)?;

        let caption = format!(
            "[#{}] {} {} TRENDS ({} plots)",
            self.rank(),
            self.name(),
            desc,
            real_point_count
        );

        let mut chart = ChartBuilder::on(&root)
            .caption(&caption, ("Consolas", 48).into_font())
            .margin(20)
            .x_label_area_size(160)
            .y_label_area_size(160)
            .build_cartesian_2d(0..n, 0.0..max.as_seconds() * MAGNIFICATION)?;

        chart
            .configure_mesh()
            .label_style(("Consolas", 32).into_font())
            .axis_desc_style(("Consolas", 40).into_font())
            .x_desc("Solves")
            .y_label_formatter(&Seconds::to_readable_string)
            .draw()?;

        for (start, end) in real_point_segments(times) {
            chart.draw_series(LineSeries::new(
                (start..end).map(|i| (i, times[i].as_seconds())),
                RGBColor(91, 169, 253).stroke_width(3),
            ))?;
        }

        root.present()?;

        Ok(())
    }

    /// Every `Record` that has a comment.
    pub fn commented_records(&self) -> Vec<(usize, Rc<Record>)> {
        self.records()
            .iter()
            .enumerate()
            .filter(|(_, r)| !r.comment().is_empty())
            .map(|(i, r)| (i + 1, Rc::clone(r)))
            .collect()
    }
}
