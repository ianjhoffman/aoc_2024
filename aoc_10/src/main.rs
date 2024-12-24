use util::res::Result;
use std::collections::HashSet;

struct Map {
    width: usize,
    height: usize,
    elevations: Vec<Vec<u8>>,
    trailheads: Vec<(usize, usize)>,
}

impl Map {
    fn from_lines(lines: &Vec<String>) -> Result<Map> {
        let height = lines.len();
        let width = lines[0].len();
        let mut trailheads: Vec<(usize, usize)> = vec![];
        let elevations: Vec<Vec<u8>> = lines.iter().enumerate().map(|(row, line)| {
            line.chars().enumerate().map(|(col, c)| {
                if c == '0' {
                    trailheads.push((row, col));
                }

                c.to_digit(10).map_or_else(
                    || Err(From::from(format!("Invalid digit: {}", c).to_owned())),
                |digit| Ok(digit as u8),
                )
            }).collect::<Result<Vec<u8>>>()
        }).collect::<Result<Vec<Vec<u8>>>>()?;

        Ok(Map{width, height, elevations, trailheads})
    }

    fn get_trailhead_score_and_rating(&self, trailhead_coords: (usize, usize)) -> (usize, usize) {
        let mut dfs_stack: Vec<(u8, (usize, usize))> = vec![(0, trailhead_coords)];
        let mut reached_summits: HashSet<(usize, usize)> = HashSet::new();
        let mut rating: usize = 0;

        while !dfs_stack.is_empty() {
            let (desired_elevation, coord) = dfs_stack.pop().unwrap();
            let curr_elevation = self.elevations[coord.0][coord.1];
            if curr_elevation != desired_elevation { continue }

            if desired_elevation == 9 {
                rating += 1;
                reached_summits.insert(coord);
                continue;
            }

            // Up state
            if coord.0 > 0 {
                dfs_stack.push((desired_elevation + 1, (coord.0 - 1, coord.1)));
            }

            // Left state
            if coord.1 > 0 {
                dfs_stack.push((desired_elevation + 1, (coord.0, coord.1 - 1)));
            }

            // Down state
            if coord.0 < self.height - 1 {
                dfs_stack.push((desired_elevation + 1, (coord.0 + 1, coord.1)));
            }

            // Right
            if coord.1 < self.width - 1 {
                dfs_stack.push((desired_elevation + 1, (coord.0, coord.1 + 1)));
            }
        }

        (reached_summits.len(), rating)
    }

    fn get_overall_trailhead_score_and_rating(&self) -> (usize, usize) {
        self.trailheads.iter().map(|trailhead_coords| self.get_trailhead_score_and_rating(*trailhead_coords))
            .fold((0, 0), |acc, (score, rating)| (acc.0 + score, acc.1 + rating))
    }
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let lines = util::file::read_lines_raw(file_path)?;
    let map = Map::from_lines(&lines)?;

    let (overall_trailhead_score, overall_trailhead_rating) = map.get_overall_trailhead_score_and_rating();

    println!("Overall trailhead score: {}", overall_trailhead_score);
    println!("Overall trailhead rating: {}", overall_trailhead_rating);

    Ok(())
}