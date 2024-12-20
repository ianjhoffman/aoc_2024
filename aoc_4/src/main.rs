use util::res::Result;

struct Crossword {
    lines: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Crossword {
    // (column, row) pairs of spaces to check for XMAS/SAMX sequences
    const XMAS_CHECKS: [[(usize, usize); 4]; 4] = [
        // Horizontal
        [(0, 0), (0, 1), (0, 2), (0, 3)],

        // Vertical
        [(0, 0), (1, 0), (2, 0), (3, 0)],

        // Diagonals (down-right, up-left)
        [(0, 0), (1, 1), (2, 2), (3, 3)],
        [(3, 0), (2, 1), (1, 2), (0, 3)],
    ];

    // (column, row) pairs of spaces to check for MAS crosses
    // 5 elements: center A, M/S of one diagonal, M/S of other diagonal
    const MAS_X_CHECKS: [[(usize, usize); 5]; 2] = [
        // Diagonals are the same (top to bottom)
        [(1, 1), (0, 0), (2, 2), (0, 2), (2, 0)],

        // Diagonals are in opposite order (top to bottom)
        [(1, 1), (0, 0), (2, 2), (2, 0), (0, 2)],
    ];

    fn new(lines: &Vec<String>) -> Crossword {
        let width = lines[0].len();
        let height = lines.len();

        // Pad rows and columns with 3 extra dummy characters to make 4x4 kernel application easier
        let mut char_lines: Vec<Vec<char>> = lines.iter().map(|line| {
            let mut char_line = line.chars().collect::<Vec<char>>();
            char_line.extend_from_slice(&['.', '.', '.']);
            char_line
        }).collect();
        char_lines.extend(vec![vec!['.'; width + 3]; 3]);

        Crossword{lines: char_lines, width: width + 3, height: height + 3}
    }

    fn count_xmas_in_4_by_4(&self, top_left_row: usize, top_left_col: usize) -> usize {
        // Out of bounds
        if (top_left_row + 4) > self.height || (top_left_col + 4) > self.width {
            return 0;
        }

        Crossword::XMAS_CHECKS.iter().map(|check| {
            let chars: [char; 4] = [
                self.lines[top_left_row + check[0].0][top_left_col + check[0].1],
                self.lines[top_left_row + check[1].0][top_left_col + check[1].1],
                self.lines[top_left_row + check[2].0][top_left_col + check[2].1],
                self.lines[top_left_row + check[3].0][top_left_col + check[3].1],
            ];

            match chars {
                ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'] => 1usize,
                _ => 0usize,
            }
        }).sum()
    }

    fn count_xmas_in_grid(&self) -> usize {
        let mut count: usize = 0;
        for row in 0..=(self.height - 4) {
            for col in 0..=(self.width - 4) {
                count += self.count_xmas_in_4_by_4(row, col);
            }
        }

        count
    }

    fn count_max_s_in_3_by_3(&self, top_left_row: usize, top_left_col: usize) -> usize {
        // Out of bounds
        if (top_left_row + 3) > self.height || (top_left_col + 3) > self.width {
            return 0;
        }

        Crossword::MAS_X_CHECKS.iter().map(|check| {
            let chars: [char; 5] = [
                self.lines[top_left_row + check[0].0][top_left_col + check[0].1],
                self.lines[top_left_row + check[1].0][top_left_col + check[1].1],
                self.lines[top_left_row + check[2].0][top_left_col + check[2].1],
                self.lines[top_left_row + check[3].0][top_left_col + check[3].1],
                self.lines[top_left_row + check[4].0][top_left_col + check[4].1],
            ];

            match chars {
                ['A', 'M', 'S', 'M', 'S'] | ['A', 'S', 'M', 'S', 'M'] => 1usize,
                _ => 0usize,
            }
        }).sum()
    }

    fn count_mas_x_in_grid(&self) -> usize {
        let mut count: usize = 0;
        for row in 0..=(self.height - 3) {
            for col in 0..=(self.width - 3) {
                count += self.count_max_s_in_3_by_3(row, col);
            }
        }

        count
    }
}

fn part1(crossword: &Crossword) {
    let xmas_count = crossword.count_xmas_in_grid();
    println!("# of occurrences of XMAS in crossword: {:?}", xmas_count);
}

fn part2(crossword: &Crossword) {
    let mas_x_count = crossword.count_mas_x_in_grid();
    println!("# of occurrences of cross-MAS in crossword: {:?}", mas_x_count);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let lines = util::file::read_lines_raw(file_path)?;
    let crossword = Crossword::new(&lines);

    part1(&crossword);
    part2(&crossword);

    Ok(())
}