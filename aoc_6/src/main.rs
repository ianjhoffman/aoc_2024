use util::res::Result;
use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq)]
enum GridSquare {
    Obstacle,
    // Coordinates of next open blocked tile in each
    // direction (0 = up, 1 = right, 2 = down, 3 = left)
    // or None if unblocked until exiting the grid
    Open([Option<(usize, usize)>; 4])
}

struct Grid {
    width: usize,
    height: usize,
    guard_coord: (usize, usize),
    squares: Vec<Vec<GridSquare>>,
}

struct ObstacleUpdate {
    update_coord: (usize, usize),
    dir: usize,
    stop_coord: (usize, usize),
}

impl GridSquare {
    fn set_next_open(&mut self, self_pos: (usize, usize), dir: usize, coord: (usize, usize)) {
        if dir > 3 { return; } // invalid direction
        match self {
            GridSquare::Obstacle => {},
            GridSquare::Open(coords) => {
                coords[dir] = match coords[dir] {
                    Some((row, col)) => {
                        let curr_dist = self_pos.0.abs_diff(row) + self_pos.1.abs_diff(col);
                        let coord_dist = self_pos.0.abs_diff(coord.0) + self_pos.1.abs_diff(coord.1);
                        if coord_dist < curr_dist { Some(coord) } else { coords[dir] }
                    },
                    None => Some(coord)
                };
            }
        }
    }
}

impl Grid {
    fn from_lines(lines: &Vec<String>) -> Result<Grid> {
        let width = lines[0].len();
        let height = lines.len();
        let mut guard_coord: Option<(usize, usize)> = None;
        let mut squares = vec![vec![GridSquare::Open([None, None, None, None]); width]; height];

        for row_idx in 0..height {
            for (col_idx, c) in lines[row_idx].chars().enumerate() {
                match c {
                    '#' => {
                        squares[row_idx][col_idx] = GridSquare::Obstacle;

                        for update in Grid::get_updates_for_obstacle(width, height, (row_idx, col_idx)).into_iter() {
                            let update_coord = update.update_coord;
                            squares[update_coord.0][update_coord.1].set_next_open(update_coord, update.dir, update.stop_coord);
                        }
                    },
                    '^' => {
                        if guard_coord.is_some() {
                            return Err(From::from("More than 1 guard start position found".to_owned()));
                        }

                        guard_coord = Some((row_idx, col_idx));
                    }
                    _ => {}, // Nothing to do for open spaces
                }
            }
        }

        if guard_coord.is_none() {
            return Err(From::from("No guard start position found".to_owned()));
        }

        Ok(Grid{width, height, guard_coord: guard_coord.unwrap(), squares})
    }

    fn get_updates_for_obstacle(width: usize, height: usize, obstacle_coords: (usize, usize)) -> Vec<ObstacleUpdate> {
        let mut updates = Vec::with_capacity(width + height);
        let (row_idx, col_idx) = obstacle_coords;

        // Up propagation (traveling up towards this obstacle i.e. squares below it)
        for r in (row_idx + 1)..height {
            updates.push(ObstacleUpdate{update_coord: (r, col_idx), dir: 0, stop_coord: (row_idx + 1, col_idx)});
        }

        // Right propagation (traveling to the right towards this obstacle i.e. squares to the left)
        for c in 0..col_idx {
            updates.push(ObstacleUpdate{update_coord: (row_idx, c), dir: 1, stop_coord: (row_idx, col_idx - 1)});
        }

        // Down propagation (traveling down towards this obstacle i.e. squares above it)
        for r in 0..row_idx {
            updates.push(ObstacleUpdate{update_coord: (r, col_idx), dir: 2, stop_coord: (row_idx - 1, col_idx)});
        }

        // Left propagation (traveling to the left towards this obstacle i.e. squares to the right)
        for c in (col_idx + 1)..width {
            updates.push(ObstacleUpdate{update_coord: (row_idx, c), dir: 3, stop_coord: (row_idx, col_idx + 1)});
        }

        updates
    }

    fn get_guard_path_size(&self) -> usize {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut guard_pos = self.guard_coord;
        let mut guard_dir = 0; // up
        loop {
            let next_guard_pos = match self.squares[guard_pos.0][guard_pos.1] {
                GridSquare::Open(coords) => coords[guard_dir],
                GridSquare::Obstacle => return 0 // guard can't start on an obstacle
            };

            // Add visited states
            visited.extend(self.get_tile_range(guard_pos, guard_dir, next_guard_pos));

            // Decide whether we've exited or not
            match next_guard_pos {
                Some(pos) => {
                    guard_pos = pos;
                    guard_dir = (guard_dir + 1) & 0b11;
                },
                None => break
            }
        }

        visited.len()
    }

    fn does_guard_loop_with_added_obstacle(&self, obstacle_coords: (usize, usize)) -> bool {
        let mut seen_states: HashSet<(usize, usize, usize)> = HashSet::new();
        let mut guard_pos = self.guard_coord;
        let mut guard_dir = 0; // up

        // Calculate pathfinding overrides when adding this one obstacle
        let obstacle_updates = Grid::get_updates_for_obstacle(self.width, self.height, obstacle_coords);
        let overrides: HashMap<(usize, usize), GridSquare> = obstacle_updates.into_iter().map(|update| {
            let update_coord = update.update_coord;
            let mut existing_square = self.squares[update_coord.0][update_coord.1].clone();
            existing_square.set_next_open(update_coord, update.dir, update.stop_coord);

            (update_coord, existing_square)
        }).collect();

        loop {
            if !seen_states.insert((guard_pos.0, guard_pos.1, guard_dir)) {
                return true; // we're in a loop!
            }

            let next_guard_pos_candidate = overrides.get(&guard_pos).unwrap_or(&self.squares[guard_pos.0][guard_pos.1]);
            let next_guard_pos = match next_guard_pos_candidate {
                GridSquare::Open(coords) => coords[guard_dir],
                GridSquare::Obstacle => return false // guard can't start on an obstacle
            };

            // Decide whether we've exited or not
            match next_guard_pos {
                Some(pos) => {
                    guard_pos = pos;
                    guard_dir = (guard_dir + 1) & 0b11;
                },
                None => break
            }
        }

        false
    }

    fn get_num_loops_with_added_obstacle(&self) -> usize {
        let mut count = 0usize;
        for row in 0..self.height {
            for col in 0..self.width {
                if (row, col) == self.guard_coord { continue }
                if self.squares[row][col] == GridSquare::Obstacle { continue }

                if self.does_guard_loop_with_added_obstacle((row, col)) {
                    count += 1
                }
            } 
        }

        count
    }

    fn get_tile_range(&self, start: (usize, usize), dir: usize, end: Option<(usize, usize)>) -> Vec<(usize, usize)> {
        match dir {
            0 => { // up
                let end_col = end.unwrap_or((0, 0)).0;
                (end_col..=start.0).map(|col| (col, start.1)).collect()
            },
            1 => { // right
                let end_row = end.unwrap_or((0, self.width - 1)).1;
                (start.1..=end_row).map(|row| (start.0, row)).collect()
            },
            2 => { // down
                let end_col = end.unwrap_or((self.height - 1, 0)).0;
                (start.0..=end_col).map(|col| (col, start.1)).collect()
            },
            3 => { // left
                let end_row = end.unwrap_or((0, 0)).1;
                (end_row..=start.1).map(|row| (start.0, row)).collect()
            },
            _ => vec![]
        }
    }
}

fn part1(grid: &Grid) {
    let guard_path_size = grid.get_guard_path_size();

    println!("# of tiles visited by guard: {}", guard_path_size);
}

fn part2(grid: &Grid) {
    let num_loops = grid.get_num_loops_with_added_obstacle();

    println!("# loops with 1 added obstacle: {}", num_loops);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let lines = util::file::read_lines_raw(file_path)?;

    let grid = Grid::from_lines(&lines)?;

    part1(&grid);
    part2(&grid);

    Ok(())
}