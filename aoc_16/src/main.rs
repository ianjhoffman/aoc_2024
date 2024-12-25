use util::res::Result;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::usize;
use std::cmp::Ordering;

// ((row, col), direction)
type State = ((usize, usize), u8);

#[derive(Copy, Clone, Eq, PartialEq)]
struct SearchState {
    state: State,
    score: usize,
}

struct Maze {
    width: usize,
    height: usize,
    grid: Vec<Vec<bool>>,
    non_wall_tiles: Vec<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
            .then_with(|| self.state.cmp(&other.state))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Maze {
    fn from_lines_raw(lines: &Vec<String>) -> Result<Maze> {
        let mut start: Option<(usize, usize)> = None;
        let mut end: Option<(usize, usize)> = None;
        let mut non_wall_tiles: Vec<(usize, usize)> = vec![];
        let grid= lines.iter().enumerate().map(|(row_idx, line)| {
            line.chars().enumerate().map(|(col_idx, c)| {
                let coord = (row_idx, col_idx);
                match c {
                    'S' => {
                        start = Some(coord);
                        non_wall_tiles.push(coord);
                        Ok(false)
                    },
                    'E' => {
                        end = Some(coord);
                        non_wall_tiles.push(coord);
                        Ok(false)
                    },
                    '#' => Ok(true),
                    '.' => {
                        non_wall_tiles.push(coord);
                        Ok(false)
                    },
                    _ => Err(From::from(format!("Invalid maze character: {}", c))),
                }
            }).collect::<Result<Vec<bool>>>()
        }).collect::<Result<Vec<Vec<bool>>>>()?;

        match (start, end) {
            (Some(start_coords), Some(end_coords)) => {
                Ok(Maze{
                    width: lines[0].len(),
                    height: lines.len(),
                    grid: grid,
                    non_wall_tiles: non_wall_tiles,
                    start: start_coords,
                    end: end_coords,
                })
            },
            _ => Err(From::from("Maze is missing start, end, or both!"))
        }
    }

    // Returns a tuple of `lowest_score` and `tiles_on_shortest_paths`
    fn find_shortest_path_info(&self) -> (usize, usize) {
        // Used for constructing best score/best previous state maps
        let all_possible_states: Vec<State> = self.non_wall_tiles.iter().flat_map(|coords| {
            (0..=3).map(|dir| (*coords, dir))
        }).collect();

        // Best score map
        let mut best_score: HashMap<State, usize> = all_possible_states.iter().map(|state| {
            (*state, usize::MAX)
        }).collect();

        // Best previous state map
        let mut best_prev: HashMap<State, [Option<State>; 4]> = all_possible_states.iter().map(|state| {
            (*state, [None, None, None, None])
        }).collect();

        // Start search off with just the start state
        let mut frontier = BinaryHeap::new();
        best_score.entry((self.start, 1)).or_insert(0);
        frontier.push(SearchState{state: (self.start, 1), score: 0});

        while let Some(search_state) = frontier.pop() {
            let coord = search_state.state.0;
            let dir = search_state.state.1;

            if coord == self.end { continue; }
            if search_state.score > *best_score.get(&search_state.state).unwrap() { continue; }

            // Get up to 4 possible next states (4 different directions)
            let mut next_states: Vec<SearchState> = vec![];

            // Try going north (dir = 0)
            if coord.0 > 0 && !self.grid[coord.0 - 1][coord.1] {
                let score_penalty = if dir != 0 { 1001 } else { 1 };
                next_states.push(SearchState{state: ((coord.0 - 1, coord.1), 0), score: search_state.score + score_penalty});
            }

            // Try going east (dir = 1)
            if coord.1 < (self.width - 1) && !self.grid[coord.0][coord.1 + 1] {
                let score_penalty = if dir != 1 { 1001 } else { 1 };
                next_states.push(SearchState{state: ((coord.0, coord.1 + 1), 1), score: search_state.score + score_penalty});
            }

            // Try going south (dir = 2)
            if coord.0 < (self.height - 1) && !self.grid[coord.0 + 1][coord.1] {
                let score_penalty = if dir != 2 { 1001 } else { 1 };
                next_states.push(SearchState{state: ((coord.0 + 1, coord.1), 2), score: search_state.score + score_penalty});
            }

            // Try going west (dir = 3)
            if coord.1 > 0 && !self.grid[coord.0][coord.1 - 1] {
                let score_penalty = if dir != 3 { 1001 } else { 1 };
                next_states.push(SearchState{state: ((coord.0, coord.1 - 1), 3), score: search_state.score + score_penalty});
            }

            // See if it's worth adding any of the next states to the frontier
            for next_state in next_states.iter() {
                let next_state_best_score = *best_score.get(&next_state.state).unwrap();
                if next_state.score < next_state_best_score {
                    frontier.push(*next_state);
                    best_score.entry(next_state.state).and_modify(|e| *e = next_state.score);
                    best_prev.entry(next_state.state).and_modify(|e| {
                        e[search_state.state.1 as usize] = Some(search_state.state);
                    });
                } else if next_state.score == next_state_best_score {
                    // We found an equally good path, set prev reference for alternate path
                    best_prev.entry(next_state.state).and_modify(|e| {
                        e[search_state.state.1 as usize] = Some(search_state.state);
                    });
                }
            }
        }

        let lowest_score = (0..=3).fold(usize::MAX, |acc, dir| {
            acc.min(*best_score.get(&(self.end, dir)).unwrap())
        });

        let mut tiles_on_path: HashSet<(usize, usize)> = HashSet::new();

        // Populate DFS stack with initial paths away from end tile
        let mut dfs_stack: Vec<State> = vec![];
        for dir in 0..=3 {
            if *best_score.get(&(self.end, dir)).unwrap() == lowest_score {
                dfs_stack.push((self.end, dir));
            }
        }

        while let Some(next_state) = dfs_stack.pop() {
            tiles_on_path.insert(next_state.0);
            for maybe_prev_state in best_prev.get(&next_state).unwrap() {
                if let Some(prev_state) = maybe_prev_state {
                    dfs_stack.push(*prev_state);
                }
            }
        }

        (lowest_score, tiles_on_path.len())
    }
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let lines = util::file::read_lines_raw(file_path)?;
    let maze = Maze::from_lines_raw(&lines)?;
    let (lowest_score, tiles_on_shortest_paths) = maze.find_shortest_path_info();

    println!("Lowest possible score: {}", lowest_score);
    println!("# tiles on shortest paths: {}", tiles_on_shortest_paths);

    Ok(())
}