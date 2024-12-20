use util::res::Result;
use util::file::GenericParseError;

struct Report {
    levels: Vec<u32>
}

struct ReadingDiff {
    curr: u32,
    sign: i32,
}

impl ReadingDiff {
    const MIN_DIFF: i64 = 1;
    const MAX_DIFF: i64 = 3;

    fn from_readings(last: u32, curr: u32) -> Option<ReadingDiff> {
        let diff = i64::from(curr) - i64::from(last);
        let abs = diff.abs();

        if Self::MIN_DIFF > abs || Self::MAX_DIFF < abs {
            None
        } else {
            Some(ReadingDiff{curr, sign: diff.signum() as i32})
        }
    }

    fn from_diff_and_reading(last: &ReadingDiff, curr: u32) -> Option<ReadingDiff> {
        let diff = i64::from(curr) - i64::from(last.curr);
        let abs = diff.abs();
        let sign = diff.signum() as i32;

        if Self::MIN_DIFF > abs || Self::MAX_DIFF < abs || sign != last.sign {
            None
        } else {
            Some(ReadingDiff{curr, sign})
        }
    }
}

impl std::str::FromStr for Report {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Report{
            levels: s.split_whitespace().map(|raw_num|
                raw_num.parse::<u32>().map_err(|e| e.into())
            ).collect::<std::result::Result<Vec<u32>, Self::Err>>()?,
        })
    }
}

impl Report {
    fn is_safe(&self, with_tolerance: bool) -> bool {
        if with_tolerance {
            let safe_fwd = self.check_safety(true, true);
            let safe_bwd = self.check_safety(false, true);

            safe_fwd || safe_bwd
        } else {
            self.check_safety(true, false)
        }
    }

    fn check_safety(&self, forwards: bool, with_tolerance: bool) -> bool {
        let it: Box<dyn Iterator<Item = &u32>> = if forwards {
            Box::new(self.levels.iter())
        } else {
            Box::new(self.levels.iter().rev())
        };

        let mut skip_available = with_tolerance;
        let mut last_reading: Option<u32> = None;
        let mut last_diff: Option<ReadingDiff> = None;
        for reading in it {
            if last_reading.is_none() {
                last_reading = Some(*reading);
                continue;
            }

            let curr_diff = match &last_diff {
                Some(reading_diff) => {
                    ReadingDiff::from_diff_and_reading(reading_diff, *reading)
                },
                None => {
                    ReadingDiff::from_readings(last_reading.unwrap(), *reading)
                }
            };

            match (&curr_diff, skip_available) {
                (Some(_), _) => last_diff = curr_diff,
                (None, true) => skip_available = false,
                _ => return false
            }
        }

        true
    }
}

fn get_num_safe_reports(reports: &Vec<Report>, with_tolerance: bool) -> usize {
    reports.iter().filter_map(|report| {
        if report.is_safe(with_tolerance) { Some(()) } else { None }
    }).count()
}

fn part1(reports: &Vec<Report>) {
    println!("# safe reports WITHOUT tolerance: {:?}", get_num_safe_reports(reports, false));
}

fn part2(reports: &Vec<Report>) {
    println!("# safe reports WITH tolerance: {:?}", get_num_safe_reports(reports, true));
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let reports = util::file::read_lines_to_type::<Report>(file_path)?;

    part1(&reports);
    part2(&reports);

    Ok(())
}