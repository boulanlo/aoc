use std::{ops::Range, str::FromStr};

use color_eyre::{Report, Result};
use itertools::Itertools;

use crate::{runner::Messenger, Challenge, Dataset};

fn clamp_range(min: i32, max: i32, r: &mut Range<i32>) {
    r.start = r.start.max(min);
    r.end = r.end.min(max);
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct SensorBeacon {
    sensor: Point,
    beacon: Point,
}

impl SensorBeacon {
    pub fn width_at_y(&self, y: i32) -> Option<(i32, i32)> {
        let y_diff_between_sensor_and_beacon = self.sensor.y.abs_diff(self.beacon.y);

        let dist = self.sensor.x.abs_diff(self.beacon.x);

        let radius = dist + y_diff_between_sensor_and_beacon;

        let distance_to_query_y = self.sensor.y.abs_diff(y);

        radius.checked_sub(distance_to_query_y).map(|v| {
            let x1 = self.sensor.x - (v as i32);
            let x2 = self.sensor.x + (v as i32);

            (x1, x2)
        })
    }
}

impl FromStr for SensorBeacon {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn extract_number(s: &str) -> i32 {
            s.chars()
                .filter(|c| c.is_numeric() || *c == '-')
                .collect::<String>()
                .parse()
                .unwrap()
        }

        let (left, right) = s.split_once("beacon").unwrap();

        let (left_x, left_y) = left.split_once(',').unwrap();
        let (left_x, left_y) = (extract_number(left_x), extract_number(left_y));

        let (right_x, right_y) = right.split_once(',').unwrap();
        let (right_x, right_y) = (extract_number(right_x), extract_number(right_y));

        Ok(Self {
            sensor: (left_x, left_y).into(),
            beacon: (right_x, right_y).into(),
        })
    }
}

pub struct Day15 {
    dataset: Dataset,
}

impl Day15 {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

impl Challenge for Day15 {
    fn name(&self) -> &'static str {
        "Beacon Exclusion Zone"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let y = 2000000;

        let sensors = data
            .iter()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<SensorBeacon>>();

        let res = sensors.iter().filter_map(|s| s.width_at_y(y)).fold(
            Vec::new(),
            |mut v: Vec<Range<i32>>, (start, end)| {
                let mut x = start..end + 1;
                while let Some(r) = v.iter().position(|r| {
                    Range::contains(r, &x.start)
                        || Range::contains(r, &x.end)
                        || Range::contains(&x, &r.start)
                        || Range::contains(&x, &r.end)
                }) {
                    let r = v.swap_remove(r);
                    if Range::contains(&r, &x.start) && Range::contains(&r, &x.end) {
                        x = r;
                    } else if r.start < x.start {
                        x.start = r.start;
                    } else if r.end > x.end {
                        x.end = r.end;
                    }
                }
                v.push(x);

                v
            },
        );

        println!("{:?}", res);

        let res = res
            .into_iter()
            .map(|r| {
                let objects_on_line = sensors
                    .iter()
                    .flat_map(|s| vec![s.sensor, s.beacon])
                    .unique()
                    .filter(|p| (p.y == y && r.contains(&p.x)))
                    .count();
                r.len() - objects_on_line
            })
            .sum::<usize>();

        Ok(res.to_string())
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let min = 0;
        let max = 4000000;

        let sensors = data
            .iter()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<SensorBeacon>>();

        let (x, y) = (min..max + 1)
            .find_map(|y| {
                let res = sensors.iter().filter_map(|s| s.width_at_y(y)).fold(
                    Vec::new(),
                    |mut v: Vec<Range<i32>>, (start, end)| {
                        let mut x = start.max(min)..(end + 1).min(max);

                        while let Some(r) = v.iter().position(|r| {
                            Range::contains(r, &x.start)
                                || Range::contains(r, &x.end)
                                || Range::contains(&x, &r.start)
                                || Range::contains(&x, &r.end)
                        }) {
                            let r = v.swap_remove(r);
                            if Range::contains(&r, &x.start) && Range::contains(&r, &x.end) {
                                x = r;
                            } else if r.start < x.start {
                                x.start = r.start;
                            } else if r.end > x.end {
                                x.end = r.end;
                            }
                            clamp_range(min, max, &mut x);
                        }
                        v.push(x);

                        v
                    },
                );

                let width = res.iter().map(|r| r.len()).sum::<usize>();

                (width != max as usize).then(move || {
                    let x = match res.as_slice() {
                        [a] => {
                            if a.start != min {
                                min
                            } else {
                                max
                            }
                        }
                        [a, b] => {
                            if a.end < b.start {
                                a.end
                            } else {
                                b.end
                            }
                        }
                        [] | [_, _, ..] => unreachable!(),
                    };

                    (x, y)
                })
            })
            .unwrap();

        let tuning_multiplier = 4000000i64;
        let tuning_frequency = ((x as i64) * tuning_multiplier) + (y as i64);

        Ok(tuning_frequency.to_string())
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
