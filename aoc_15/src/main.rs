use util::res::Result;
use std::collections::HashSet;

#[derive(Clone)]
enum Direction {Up, Down, Left, Right}

#[derive(Clone, Hash, PartialEq, Eq)]
struct GridBox {
    coords: (usize, usize),
    double_wide: bool,
}

impl Direction {
    fn from_char(c: &char) -> Result<Direction> {
        match c {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(From::from(format!("Invalid direction: {}", c))),
        }
    }
}

impl GridBox {
    fn new(coords: (usize, usize), double_wide: bool) -> GridBox {
        GridBox{coords, double_wide}
    }

    fn to_check_in_direction(&self, dir: &Direction) -> Vec<GridBox> {
        let coords = self.coords;
        let mut to_check = vec![];
        match dir {
            Direction::Up | Direction::Down => {
                let right_col = if self.double_wide { coords.1 + 1 } else { coords.1 };
                let new_row = if let Direction::Up = dir { coords.0 - 1 } else { coords.0 + 1 };
                for new_col in coords.1..=right_col {
                    to_check.push(GridBox::new((new_row, new_col), false));
                    to_check.push(GridBox::new((new_row, new_col), true));
                }

                // Also push double-wide box up and to the left
                to_check.push(GridBox::new((new_row, coords.1 - 1), true));
            },
            Direction::Left => {
                to_check.push(GridBox::new((coords.0, coords.1 - 1), false));
                if coords.1 > 1 {
                    to_check.push(GridBox::new((coords.0, coords.1 - 2), true));
                }
            },
            Direction::Right => {
                let col_to_right = if self.double_wide { coords.1 + 2 } else { coords.1 + 1 };
                to_check.push(GridBox::new((coords.0, col_to_right), false));
                to_check.push(GridBox::new((coords.0, col_to_right), true));
            }
        }

        to_check
    }

    fn push(&self, dir: &Direction) -> GridBox {
        match dir {
            Direction::Up => GridBox::new((self.coords.0 - 1, self.coords.1), self.double_wide),
            Direction::Down => GridBox::new((self.coords.0 + 1, self.coords.1), self.double_wide),
            Direction::Left => GridBox::new((self.coords.0, self.coords.1 - 1), self.double_wide),
            Direction::Right => GridBox::new((self.coords.0, self.coords.1 + 1), self.double_wide),
        }
    }

    fn gps_coordinate(&self) -> usize {
        (100 * self.coords.0) + self.coords.1
    }
}

struct Warehouse {
    width: usize,
    height: usize,
    grid: Vec<Vec<bool>>, // true if wall, false if empty
    boxes: HashSet<GridBox>,
    robot_start: (usize, usize),
    movements: Vec<Direction>,
}

impl Warehouse {
    fn from_lines(lines: &Vec<String>) -> Result<Warehouse> {
        let width = lines[0].len();
        let height = lines.len();
        let mut robot_start: Option<(usize, usize)> = None;
        let mut grid: Vec<Vec<bool>> = vec![];
        let mut boxes: HashSet<GridBox> = HashSet::new();
        let mut movements: Vec<Direction> = vec![];

        // Parse lines into grid first, then movements
        let mut grid_row = 0usize;
        let mut parsing_grid = true;
        for line in lines {
            if line.is_empty() {
                parsing_grid = false;
                continue;
            }

            if parsing_grid {
                grid.push(line.chars().enumerate().map(|(col, c)| {
                    match c {
                        '@' => {
                            robot_start = Some((grid_row, col));
                            false
                        },
                        'O' => {
                            boxes.insert(GridBox::new((grid_row, col), false));
                            false
                        },
                        '#' => true,
                        _ => false,
                    }
                }).collect::<Vec<bool>>());

                grid_row += 1;
            } else {
                movements.extend(line.chars().map(|c| {
                    Direction::from_char(&c)
                }).collect::<Result<Vec<Direction>>>()?);
            }
        }

        match robot_start {
            Some(start_coords) => {
                Ok(Warehouse{width, height, grid, boxes, robot_start: start_coords, movements})
            },
            None => Err(From::from("No robot start position found!"))
        }
    }

    fn simulate_one_movement(
        &self,
        boxes: &mut HashSet<GridBox>,
        robot_position: &mut (usize, usize),
        dir: &Direction,
    ) {
        let mut to_push: Vec<HashSet<GridBox>> = vec![];
        let robot_dummy_grid_box = GridBox::new(*robot_position, false);
        let mut to_check: HashSet<GridBox> = HashSet::from_iter(
            robot_dummy_grid_box.to_check_in_direction(dir).into_iter()
        );

        while !to_check.is_empty() {
            let mut curr_layer: HashSet<GridBox> = HashSet::new();
            let mut to_check_next_layer: HashSet<GridBox> = HashSet::new();
            for grid_box in to_check.drain() {
                // We're checking if there's a grid box like this
                // that we'd need to push but there might not be
                if boxes.contains(&grid_box) {
                    curr_layer.insert(grid_box.clone());
                    to_check_next_layer.extend(grid_box.to_check_in_direction(dir));

                    continue;
                }

                // Hitting a wall means we're done (only check walls for single-wide grid box locations)
                if !grid_box.double_wide && grid_box.coords.0 < self.height && grid_box.coords.1 < self.width {
                    if self.grid[grid_box.coords.0][grid_box.coords.1] {
                        // We hit a wall
                        return
                    }
                }
            }

            to_push.push(curr_layer);
            to_check = to_check_next_layer;
        }

        // Push furthest boxes first
        for layer in to_push.iter().rev() {
            for to_push_box in layer {
                boxes.remove(to_push_box);
                boxes.insert(to_push_box.push(dir));
            }
        }

        // Move robot
        *robot_position = robot_dummy_grid_box.push(dir).coords;
    }

    fn simulate_movements(&self) -> HashSet<GridBox> {
        let mut boxes = self.boxes.clone();
        let mut robot_position = self.robot_start;
        
        for dir in &self.movements {
            self.simulate_one_movement(&mut boxes, &mut robot_position, dir);
        }

        boxes
    }

    fn calculate_gps_sum(boxes: &HashSet<GridBox>) -> usize {
        boxes.iter().fold(0, |acc, grid_box| acc + grid_box.gps_coordinate())
    }

    fn with_double_width(&self) -> Warehouse {
        let mut new_warehouse = Warehouse{
            width: self.width * 2,
            height: self.height,
            grid: vec![],
            boxes: HashSet::new(),
            robot_start: (self.robot_start.0, self.robot_start.1 * 2),
            movements: self.movements.clone(),
        };

        for row in self.grid.iter() {
            new_warehouse.grid.push(row.iter().flat_map(|&square| {
                [square, square]
            }).collect::<Vec<bool>>());
        }

        for grid_box in self.boxes.iter() {
            new_warehouse.boxes.insert(GridBox::new((grid_box.coords.0, grid_box.coords.1 * 2), true));
        }

        new_warehouse
    }
}

fn part1(warehouse: &Warehouse) {
    let boxes_after_movements = warehouse.simulate_movements();
    let gps_sum = Warehouse::calculate_gps_sum(&boxes_after_movements);

    println!("GPS sum after all robot movements: {}", gps_sum);
}

fn part2(warehouse: &Warehouse) {
    let new_warehouse = warehouse.with_double_width();
    let grid_after_movements = new_warehouse.simulate_movements();
    let gps_sum = Warehouse::calculate_gps_sum(&grid_after_movements);

    println!("GPS sum after all robot movements in double-wide warehouse: {}", gps_sum);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let lines = util::file::read_lines_raw(file_path)?;
    let warehouse = Warehouse::from_lines(&lines)?;

    part1(&warehouse);
    part2(&warehouse);

    Ok(())
}