use util::res::Result;
use std::collections::{HashMap, HashSet};

struct Antenna {
    frequency: char,
    location: (usize, usize),
}

struct AntennaNetwork {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<Antenna>>, 
}

// Returns gcd, x, y such that ax + by = gcd
pub fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
	if a == 0 { return (b, 0, 1); }
	let (gcd, x, y) = extended_gcd(b % a, a);
    (gcd, (y - (b/a) * x), x)
}

impl Antenna {
    fn get_antinodes(&self, other: &Antenna, y_limit: usize, x_limit: usize, allow_resonant_harmonics: bool) -> Vec<(usize, usize)> {
        // Ideally we won't be comparing antennas of different frequencies in the
        // first place, but this safety mechanism doesn't hurt to have
        if self.frequency != other.frequency { return vec![]; }

        let mut antinodes_signed: Vec<(i32, i32)> = vec![];
        let self_signed = (self.location.0 as i32, self.location.1 as i32);
        let other_signed = (other.location.0 as i32, other.location.1 as i32);

        // Prepare bounds ranges
        let y_range = 0..(y_limit as i32);
        let x_range = 0..(x_limit as i32);

        // There are always 2 antinodes on either side of the pair of antennas
        let diff = (self_signed.0 - other_signed.0, self_signed.1 - other_signed.1);
        if allow_resonant_harmonics {
            let diff_gcd = extended_gcd(diff.0.abs(), diff.1.abs()).0;
            let reduced_diff = (diff.0 / diff_gcd, diff.1 / diff_gcd);

            // Collect antinodes in both directions, including in between
            // First loop is from `other` towards `self`, starting 1-step in
            // Second loop is from `other` away from `self` (including `other`)
            let mut position = (other_signed.0 + reduced_diff.0, other_signed.1 + reduced_diff.1);
            while y_range.contains(&position.0) && x_range.contains(&position.1) {
                antinodes_signed.push(position);
                position = (position.0 + reduced_diff.0, position.1 + reduced_diff.1);
            }

            position = other_signed;
            while y_range.contains(&position.0) && x_range.contains(&position.1) {
                antinodes_signed.push(position);
                position = (position.0 - reduced_diff.0, position.1 - reduced_diff.1);
            }
        } else {
            antinodes_signed.push((self_signed.0 + diff.0, self_signed.1 + diff.1));
            antinodes_signed.push((other_signed.0 - diff.0, other_signed.1 - diff.1));
        }

        antinodes_signed.into_iter().filter_map(|antinode| {
            if y_range.contains(&antinode.0) && x_range.contains(&antinode.1) {
                Some((antinode.0 as usize, antinode.1 as usize))
            } else {
                None // discard antinodes with negative coordinates or out of bounds
            }
        }).collect()
    }
}

impl AntennaNetwork {
    fn from_lines(lines: &Vec<String>) -> AntennaNetwork {
        let mut network = AntennaNetwork{
            height: lines.len(),
            width: lines[0].len(),
            antennas: HashMap::new(),
        };

        for (row, line) in lines.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '.' { continue }

                network.antennas.entry(c).or_insert(vec![]).push(Antenna{
                    frequency: c,
                    location: (row, col)
                });
            }
        }

        network
    }

    fn get_antinodes_in_bounds(&self, allow_resonant_harmonics: bool) -> HashSet<(char, (usize, usize))> {
        let mut antinodes_in_bounds: HashSet<(char, (usize, usize))> = HashSet::new();
        for (frequency, antennas) in &self.antennas {
            // Have to go over all pairs of antennas with the same frequency
            for (i, first_antenna) in antennas.iter().enumerate() {
                for (j, second_antenna) in antennas.iter().enumerate() {
                    if i == j { continue }
                    antinodes_in_bounds.extend(
                        first_antenna.get_antinodes(second_antenna, self.height, self.width, allow_resonant_harmonics)
                            .into_iter().map(|antinode| (*frequency, antinode))
                            .collect::<Vec<(char, (usize, usize))>>()
                    );
                }
            }
        }
        
        antinodes_in_bounds
    }
}

fn part1(network: &AntennaNetwork) {
    let antinodes_in_bounds = network.get_antinodes_in_bounds(false);
    let unique_locations = antinodes_in_bounds.iter().map(|(_, loc)| *loc)
        .collect::<HashSet<(usize, usize)>>();

    println!("# unique antinode locations in bounds (without resonant harmonics): {}", unique_locations.len());
}

fn part2(network: &AntennaNetwork) {
    let antinodes_in_bounds = network.get_antinodes_in_bounds(true);
    let unique_locations = antinodes_in_bounds.iter().map(|(_, loc)| *loc)
        .collect::<HashSet<(usize, usize)>>();

    println!("# unique antinode locations in bounds (with resonant harmonics): {}", unique_locations.len());
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let lines = util::file::read_lines_raw(file_path)?;
    let network = AntennaNetwork::from_lines(&lines);

    part1(&network);
    part2(&network);

    Ok(())
}