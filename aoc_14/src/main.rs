#[macro_use] extern crate lazy_static;
use core::f32;
use util::res::Result;
use util::file::GenericParseError;
use regex::Regex;

struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl std::str::FromStr for Robot {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            static ref ROBOT_REGEX: Regex = Regex::new(r"^p=(\d+),(\d+) v=(\-?\d+),(\-?\d+)$").unwrap();
        }

        match ROBOT_REGEX.captures(s) {
            Some(caps) => {
                Ok(Robot{
                    position: (caps.get(1).unwrap().as_str().parse::<i32>()?, caps.get(2).unwrap().as_str().parse::<i32>()?),
                    velocity: (caps.get(3).unwrap().as_str().parse::<i32>()?, caps.get(4).unwrap().as_str().parse::<i32>()?),
                })
            },
            None => return Err(GenericParseError::ValueError(format!("Invalid line: {}", s).to_owned()))
        }
    }
}

impl Robot {
    fn position_after_n_seconds(&self, num_seconds: usize, region_width: usize, region_height: usize) -> (i32, i32) {
        let (seconds_signed, width_signed, height_signed) = (num_seconds as i32, region_width as i32, region_height as i32);
        let position_incr = (self.velocity.0 * seconds_signed, self.velocity.1 * seconds_signed);

        let new_x = (self.position.0 + position_incr.0).rem_euclid(width_signed);
        let new_y = (self.position.1 + position_incr.1).rem_euclid(height_signed);
        (new_x, new_y)
    }
}

fn part1(robots: &Vec<Robot>) {
    let (width, height): (usize, usize) = (101, 103);
    let (half_width, half_height) = ((width as i32) / 2, (height as i32) / 2);

    // Quadrant index gets bit 0 set if on right half, bit 1 set if on bottom half, so:
    // top-left = 0, top-right = 1, bottom-left = 2, bottom-right = 3
    let mut quadrant_counts: [usize; 4] = [0; 4];
    for robot in robots {
        let position = robot.position_after_n_seconds(100, width, height);
        
        // On quadrant border, don't consider
        if position.0 == half_width || position.1 == half_height { continue }

        let left_right_component: usize = if position.0 < half_width { 0 } else { 1 };
        let up_down_component: usize = if position.1 < half_height { 0 } else { 2 };
        quadrant_counts[left_right_component + up_down_component] += 1;
    }

    let safety_factor = quadrant_counts.iter().fold(1, |acc, count| acc * count);

    println!("Safety factor after 100 seconds: {}", safety_factor);
}

fn part2(robots: &Vec<Robot>) {
    let (width, height, num_robots_f) = (101, 103, robots.len() as f32);

    // Search up to 10000 seconds into the future for an
    // unusually low-variance arrangement of robot positions
    let mut lowest_variance = f32::INFINITY;
    let mut lowest_variance_num_seconds = 0;
    for num_seconds in 0..10000 {
        let mut position_sum = (0, 0);
        let new_positions = robots.iter().map(|robot| {
            let new_position = robot.position_after_n_seconds(num_seconds, width, height);
            position_sum = (position_sum.0 + new_position.0, position_sum.1 + new_position.1);
            new_position
        }).collect::<Vec<(i32, i32)>>();

        let mean_position = ((position_sum.0 as f32) / (num_robots_f), (position_sum.1 as f32) / (num_robots_f));
        let variance = new_positions.iter().map(|new_position| {
            let diff = ((new_position.0 as f32) - mean_position.0, (new_position.1 as f32) - mean_position.1);
            (diff.0 * diff.0) + (diff.1 * diff.1)
        }).fold(0f32, |acc, diff_magnitude| acc + diff_magnitude) / num_robots_f;

        // Update the # seconds with lowest variance if we need to
        if variance < lowest_variance {
            lowest_variance = variance;
            lowest_variance_num_seconds = num_seconds;
        }
    }

    println!(
        "After 10000 seconds, the # seconds with lowest variance is {} (variance = {})",
        lowest_variance_num_seconds, lowest_variance
    );
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let robots = util::file::read_lines_to_type::<Robot>(file_path)?;

    part1(&robots);
    part2(&robots);

    Ok(())
}