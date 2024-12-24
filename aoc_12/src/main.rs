use util::res::Result;
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Clone, Hash, PartialEq, Eq)]
struct Edge {
    // the row & column of the top/left point of the edge
    coord: (usize, usize),
    // true if the edge is oriented vertically, false otherwise
    vertical: bool,
    // true if the plot this is an edge of is to the right/below it, false otherwise
    side: bool,
}

struct Region {
    plots: Vec<(usize, usize)>,
}

struct Farm {
    regions: Vec<Region>,
}

impl Edge {
    fn new(coord: (usize, usize), vertical: bool, side: bool) -> Edge {
        Edge{coord, vertical, side}
    }
}

impl Region {
    fn new(plots: Vec<(usize, usize)>) -> Region {
        Region{plots}
    }

    fn get_edges(&self) -> HashSet<Edge> {
        //
        // Maps from an "edge coordinate" (start row, start col, vertical) to
        // two fields:
        //
        //     1. How many times we've seen a tile with it at the border:
        //           No entry or 0 = this edge isn't touching the region at all
        //                       1 = this edge is on the perimeter of the region
        //                       2 = this edge is inside the region
        //     2. The 'side' of this edge (for vertical edges, if the face the
        //        edge is connected to is to its right; for horizontal edges,
        //        if the face the edge is connected to is below it)
        //
        let mut edges: HashMap<(usize, usize, bool), (usize, bool)> = HashMap::new();

        for (row, col) in &self.plots {
            edges.entry((*row, *col, true)).or_insert((0, true)).0 += 1;
            edges.entry((*row, *col, false)).or_insert((0, true)).0 += 1;
            edges.entry((*row + 1, *col, false)).or_insert((0, false)).0 += 1;
            edges.entry((*row, *col + 1, true)).or_insert((0, false)).0 += 1;
        }

        edges.iter().filter_map(|(&(row, col, vertical), &(count, side))| {
            if count == 1 { Some(Edge::new((row, col), vertical, side)) } else { None }
        }).collect()
    }

    fn perimeter(&self) -> usize {
        self.get_edges().len()
    }

    fn num_sides(&self) -> usize {
        let mut remaining_edges = self.get_edges();
        let mut num_sides: usize = 0;

        while !remaining_edges.is_empty() {
            let seed_edge = remaining_edges.iter().next().unwrap();
            let mut dfs_stack: Vec<Edge> = vec![seed_edge.clone()];
            while !dfs_stack.is_empty() {
                let curr_edge = dfs_stack.pop().unwrap();
                if !remaining_edges.remove(&curr_edge) { continue }

                let coord = curr_edge.coord;
                if curr_edge.vertical {
                    // Vertical edge, maybe try searching up/down
                    if coord.0 > 0 {
                        let up_candidate = Edge::new((coord.0 - 1, coord.1), true, curr_edge.side);
                        if remaining_edges.contains(&up_candidate) {
                            dfs_stack.push(up_candidate);
                        }
                    }

                    let down_candidate = Edge::new((coord.0 + 1, coord.1), true, curr_edge.side);
                    if remaining_edges.contains(&down_candidate) {
                        dfs_stack.push(down_candidate);
                    }
                } else {
                    // Horizontal edge, maybe try searching left/right
                    if coord.1 > 0 {
                        let left_candidate = Edge::new((coord.0, coord.1 - 1), false, curr_edge.side);
                        if remaining_edges.contains(&left_candidate) {
                            dfs_stack.push(left_candidate);
                        }
                    }

                    let right_candidate = Edge::new((coord.0, coord.1 + 1), false, curr_edge.side);
                    if remaining_edges.contains(&right_candidate) {
                        dfs_stack.push(right_candidate);
                    }
                }
            }

            num_sides += 1;
        }

        num_sides
    }

    fn area(&self) -> usize {
        self.plots.len()
    }
}

impl Farm {
    fn from_lines(lines: &Vec<String>) -> Farm {
        let height = lines.len();
        let width = lines[0].len();
        let rows: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
        let mut regions = vec![];

        // Floodfill regions
        let mut ungrouped_plots: BTreeSet<(usize, usize)> = (0..height).fold(BTreeSet::new(), |mut acc, row| {
            acc.extend((0..width).map(|col| (row, col)));
            acc
        });
        while !ungrouped_plots.is_empty() {
            let start_plot = ungrouped_plots.first().unwrap();
            let crop = rows[start_plot.0][start_plot.1];

            let mut region: Vec<(usize, usize)> = vec![];
            let mut dfs_stack: Vec<(usize, usize)> = vec![*start_plot];
            while !dfs_stack.is_empty() {
                let curr_plot = dfs_stack.pop().unwrap();
                if rows[curr_plot.0][curr_plot.1] != crop { continue }

                // Move from ungrouped plots to current region
                if !ungrouped_plots.remove(&curr_plot) { continue }
                region.push(curr_plot);

                // Maybe try searching up for this crop
                if curr_plot.0 > 0 && ungrouped_plots.contains(&(curr_plot.0 - 1, curr_plot.1)) {
                    dfs_stack.push((curr_plot.0 - 1, curr_plot.1));
                }

                // Maybe try searching left for this crop
                if curr_plot.1 > 0 && ungrouped_plots.contains(&(curr_plot.0, curr_plot.1 - 1)) {
                    dfs_stack.push((curr_plot.0, curr_plot.1 - 1));
                }

                // Maybe try searching down for this crop
                if curr_plot.0 < height - 1 && ungrouped_plots.contains(&(curr_plot.0 + 1, curr_plot.1)) {
                    dfs_stack.push((curr_plot.0 + 1, curr_plot.1));
                }

                // Maybe try searching right for this crop
                if curr_plot.1 < width - 1 && ungrouped_plots.contains(&(curr_plot.0, curr_plot.1 + 1)) {
                    dfs_stack.push((curr_plot.0, curr_plot.1 + 1));
                }
            }

            regions.push(Region::new(region));
        }

        Farm{regions}
    }

    fn calculate_all_region_prices(&self, bulk_discount: bool) -> usize {
        self.regions.iter().map(|region| {
            let fence_multipler = if bulk_discount { region.num_sides() } else { region.perimeter() };
            fence_multipler * region.area()
        }).sum()
    }
}

fn part1(farm: &Farm) {
    let all_region_prices = farm.calculate_all_region_prices(false);

    println!("All region prices WITHOUT bulk discount: {}", all_region_prices);
}

fn part2(farm: &Farm) {
    let all_region_prices = farm.calculate_all_region_prices(true);

    println!("All region prices WITH bulk discount: {}", all_region_prices);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let lines = util::file::read_lines_raw(file_path)?;
    let farm = Farm::from_lines(&lines);

    part1(&farm);
    part2(&farm);
    
    Ok(())
}