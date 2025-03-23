use std::fmt;
use std::rc::Rc;

use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;

use crate::record::*;
use crate::types::*;

/// A training session, same as the "session" in csTimer.
#[derive(Debug, Clone)]
pub struct Session {
    id: u8,
    records: Vec<Rc<Record>>,
    non_dnf_records: Vec<Rc<Record>>,
}

impl Session {
    pub fn new(id: u8, records: &[Record]) -> Self {
        let records: Vec<Rc<Record>> = records.iter().cloned().map(Rc::new).collect();
        let non_dnf_records = records
            .iter()
            .filter(|r| !r.solve_state().is_dnf())
            .cloned()
            .collect();

        Self {
            id,
            records,
            non_dnf_records,
        }
    }

    pub fn from(id: u8, records: &[Rc<Record>]) -> Self {
        let records = records.to_vec();
        let non_dnf_records = records
            .iter()
            .filter(|r| !r.solve_state().is_dnf())
            .cloned()
            .collect();

        Self {
            id,
            records,
            non_dnf_records,
        }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn records(&self) -> &[Rc<Record>] {
        &self.records
    }

    pub fn non_dnf_records(&self) -> &[Rc<Record>] {
        &self.non_dnf_records
    }
}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Session {} ({} records)", self.id, self.records.len())
    }
}

impl Session {
    /// Best and worst non-DNF solve times,
    /// in milliseconds.
    fn best_and_worst(&self) -> (Milliseconds, Milliseconds) {
        self.non_dnf_records
            .iter()
            .map(|r| r.time())
            .fold((u32::MAX, u32::MIN), |(best, worst), time| {
                (best.min(time), worst.max(time))
            })
    }

    /// Bounds of non-DNF solve times in seconds,
    /// padding at least 1 second at both ends.
    fn time_bounds(&self) -> (Milliseconds, Milliseconds) {
        let (best, worst) = self.best_and_worst();

        ((best / 1000 - 1) * 1000, (worst / 1000 + 1) * 1000)
    }

    /// Mean of non-DNF solve times.
    fn mean(&self) -> Option<Milliseconds> {
        if self.non_dnf_records.is_empty() {
            return None;
        }

        let sum: Milliseconds = self.non_dnf_records.iter().map(|r| r.time()).sum();

        Some((sum as f32 / self.non_dnf_records.len() as f32) as Milliseconds)
    }

    /// Count of solves that has
    /// the specified `SolveState`.
    fn count_solve_state(&self, is_state: &dyn Fn(&SolveState) -> bool) -> usize {
        self.records
            .iter()
            .filter(|r| is_state(r.solve_state()))
            .count()
    }

    /// Stats a single solve, or the
    /// mean or average of solves.
    fn stats(&self, pos: usize, s_type: &StatsType) -> Option<Milliseconds> {
        match s_type {
            StatsType::Single => {
                let r = &self.records[pos];

                if r.solve_state().is_dnf() {
                    None
                } else {
                    Some(r.time())
                }
            }

            StatsType::Mean(s_scale) => {
                let chunk = &self.records[pos - s_scale + 1..=pos];

                if chunk.iter().any(|r| r.solve_state().is_dnf()) {
                    None
                } else {
                    Some(
                        (chunk.iter().map(|r| r.time()).sum::<Milliseconds>() as f32
                            / *s_scale as f32)
                            .round() as Milliseconds,
                    )
                }
            }

            StatsType::Average(s_scale) => {
                let chunk = &self.records[pos + 1 - s_scale..=pos];
                let cut_off = (*s_scale as f32 * 0.05).ceil() as usize;
                let take = s_scale - cut_off * 2;

                if chunk.iter().filter(|r| r.solve_state().is_dnf()).count() > cut_off {
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
                    chunk.sort();

                    Some(
                        chunk.iter().skip(cut_off).take(take).sum::<Milliseconds>()
                            / take as Milliseconds,
                    )
                }
            }
        }
    }

    /// Splits records into groups, by a fixed interval.
    /// Returns `None` if the given interval is not divisible
    /// by 1000, nor 1000 is divisible by it.
    pub fn try_group_by_interval(&self, interval: Milliseconds) -> Option<Vec<GroupRecord>> {
        if !(interval != 0 && (1000 % interval == 0 || interval % 1000 == 0)) {
            return None;
        }

        let (min, max) = self.time_bounds();

        let mut groups = Vec::new();

        for start in (min..max).step_by(interval as usize) {
            let records: Vec<Rc<Record>> = self
                .non_dnf_records
                .iter()
                .filter(|r| {
                    let t = r.time();
                    t >= start && t < start + interval
                })
                .cloned()
                .collect();

            groups.push(GroupRecord::new(start, &records));
        }

        Some(groups)
    }

    /// Gets all the records that breaks the specified
    /// kind of personal best, along with the new PB.
    pub fn pb_breakers(
        &self,
        s_type: &StatsType,
    ) -> Option<Vec<(usize, Milliseconds, Rc<Record>)>> {
        let s_scale = match s_type {
            StatsType::Single => 1,
            StatsType::Average(scale) => *scale,
            StatsType::Mean(scale) => *scale,
        };

        if self.records.len() < s_scale {
            return None;
        }

        let mut pb = u32::MAX;

        let mut result = Vec::new();
        for (i, record) in self.records.iter().enumerate().skip(s_scale) {
            if let Some(stats) = self.stats(i, s_type) {
                if stats < pb {
                    pb = stats;
                    result.push((i + 1, pb, record.clone()));
                }
            }
        }

        Some(result)
    }

    /// Stats the whole session by the specified
    /// `StatsType`, providing a trend over time.
    pub fn trend(&self, s_type: &StatsType) -> Option<Vec<i32>> {
        let s_scale = match s_type {
            StatsType::Single => 1,
            StatsType::Average(scale) => *scale,
            StatsType::Mean(scale) => *scale,
        };

        if self.records.len() < s_scale {
            return None;
        }

        let mut result = Vec::new();
        for (i, _) in self.records.iter().enumerate().skip(s_scale) {
            if let Some(stats) = self.stats(i, s_type) {
                result.push(stats as i32);
            }
        }

        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    }
}

impl Session {
    /// Returns the best, worst, mean and average
    /// solve times of a session, which could
    /// be "DNF", so they're returned in strings.
    pub fn overview(&self) -> (String, String, String, String) {
        let n = self.records.len();
        let (best, worst) = self.best_and_worst();
        let mean = match self.mean() {
            Some(m) => m.readable(),
            None => String::from("DNF"),
        };
        let average = match self.stats(n - 1, &StatsType::Average(n)) {
            Some(avg) => avg.readable(),
            None => String::from("DNF"),
        };

        (best.readable(), worst.readable(), mean, average)
    }

    /// Returns the counts of
    /// solve states in a session.
    pub fn solve_states(&self) -> (usize, usize, usize) {
        (
            self.count_solve_state(&SolveState::is_ok),
            self.count_solve_state(&SolveState::is_plus2),
            self.count_solve_state(&SolveState::is_dnf),
        )
    }

    /// Draws a png, visualizes grouping results.
    pub fn draw_group_by_interval(
        &self,
        canvas: &HtmlCanvasElement,
        groups: Vec<GroupRecord>,
        interval: Milliseconds,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let count_max = groups
            .iter()
            .map(|g| g.records().len())
            .fold(usize::MIN, |max, count| (max.max(count)));

        let bounds = self.time_bounds();
        let (secs_min, secs_max) = (bounds.0 / 1000, bounds.1 / 1000);

        let root = CanvasBackend::with_canvas_object(canvas.clone())
            .ok_or("Failed to acquire canvas backend")?
            .into_drawing_area();
        root.fill(&WHITE)?;

        let caption = format!(
            "Session {} grouping by {}s",
            self.id,
            interval as f32 / 1000.0
        );

        let mut chart = ChartBuilder::on(&root)
            .caption(caption, ("Consolas", 48).into_font())
            .margin(20)
            .x_label_area_size(160)
            .y_label_area_size(160)
            .build_cartesian_2d(
                (secs_min - 1) as f32..(secs_max + 1) as f32,
                0u32..(count_max as f32 * 1.1) as u32,
            )?;

        chart
            .configure_mesh()
            .label_style(("Consolas", 32).into_font())
            .axis_desc_style(("Consolas", 40).into_font())
            .x_desc("Range / sec")
            .y_desc("Count")
            .draw()?;

        let coords: Vec<(f32, u32)> = groups
            .iter()
            .map(|g| (g.interval() as f32, g.records().len() as u32))
            .collect();

        chart.draw_series(coords.iter().map(|(x, y)| {
            let x0 = { *x } / 1000.0;
            let x1 = x0 + interval as f32 / 1000.0;
            let y0 = 0;
            let y1 = *y;

            Rectangle::new([(x0, y0), (x1, y1)], RGBColor(91, 169, 253).filled())
        }))?;

        root.present()?;

        Ok(())
    }

    /// Draws a png, visualizes trending results.
    pub fn draw_trending(
        &self,
        canvas: &HtmlCanvasElement,
        data: Vec<i32>,
        desc: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let n = data.len();
        let max = data.iter().max().unwrap();

        let root = CanvasBackend::with_canvas_object(canvas.clone())
            .ok_or("Failed to acquire canvas backend")?
            .into_drawing_area();
        root.fill(&WHITE)?;

        let caption = format!("Session {} {} trending ({} plots)", self.id, desc, n);

        let mut chart = ChartBuilder::on(&root)
            .caption(&caption, ("Consolas", 48).into_font())
            .margin(20)
            .x_label_area_size(160)
            .y_label_area_size(160)
            .build_cartesian_2d(0..n, 0.0..*max as f32 * 1.1 / 1000.0)?;

        chart
            .configure_mesh()
            .label_style(("Consolas", 32).into_font())
            .axis_desc_style(("Consolas", 40).into_font())
            .x_desc("Stats Number")
            .y_desc("Time / secs")
            .draw()?;

        chart.draw_series(LineSeries::new(
            (0..n).map(|x| (x, data[x] as f32 / 1000.0)),
            RGBColor(91, 169, 253).stroke_width(3),
        ))?;

        root.present()?;

        Ok(())
    }

    /// Filters the records with a comment.
    pub fn commented_records(&self) -> Vec<(usize, Rc<Record>)> {
        self.records
            .iter()
            .enumerate()
            .filter(|(_, r)| !r.comment().is_empty())
            .map(|(i, r)| (i + 1, Rc::clone(r)))
            .collect()
    }
}
