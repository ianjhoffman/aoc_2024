use util::res::Result;
use util::file::GenericParseError;

struct CalibrationEquation {
    result: u64,
    operands: Vec<u64>,
}

#[derive(Clone)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

#[derive(Clone)]
struct CalibrationResult {
    operands: Vec<u64>,
    operators: Vec<Operation>,
    result: u64,
}

impl std::str::FromStr for CalibrationEquation {
    type Err = GenericParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.split_once(": ") {
            Some((result_raw, operands_raw)) => {
                Ok(CalibrationEquation{
                    result: result_raw.parse::<u64>()?,
                    operands: operands_raw.split_ascii_whitespace().map(|operand_raw| {
                        operand_raw.parse::<u64>().map_err(|e| e.into())
                    }).collect::<std::result::Result<Vec<u64>, Self::Err>>()?
                })
            },
            None => Err(GenericParseError::ValueError(format!("Invalid equation: {}", s).to_owned()))
        }
    }
}

impl CalibrationEquation {
    fn get_calibration_result(&self, allow_concat: bool) -> Option<CalibrationResult> {
        if self.operands.is_empty() { return None }

        let start_value = self.operands[0];
        let mut dfs_stack: Vec<CalibrationResult> = vec![CalibrationResult{
            operands: vec![start_value],
            operators: vec![],
            result: start_value,
        }];

        while !dfs_stack.is_empty() {
            let mut state = dfs_stack.pop().unwrap();
            let next_operand = self.operands[state.operands.len()];
            state.operands.push(next_operand);
            let incomplete = state.operands.len() < self.operands.len();

            // Set up two next possible states
            let mut add_state = state;
            let mut mul_state = add_state.clone();
            let mut concat_state = add_state.clone();

            // Modify the next possible states based on their operators
            add_state.operators.push(Operation::Add);
            add_state.result += next_operand;

            mul_state.operators.push(Operation::Multiply);
            mul_state.result *= next_operand;

            if allow_concat {
                concat_state.operators.push(Operation::Concat);
                concat_state.result = CalibrationEquation::concatenate_operands(concat_state.result, next_operand);
            }

            // Push if incomplete and not overshooting result
            // Return if complete and exactly at result
            if incomplete && add_state.result <= self.result {
                dfs_stack.push(add_state);
            } else if !incomplete && add_state.result == self.result {
                return Some(add_state)
            } // otherwise, drop this path

            if incomplete && mul_state.result <= self.result {
                dfs_stack.push(mul_state);
            } else if !incomplete && mul_state.result == self.result {
                return Some(mul_state)
            } // otherwise, drop this path

            if allow_concat {
                if incomplete && concat_state.result <= self.result {
                    dfs_stack.push(concat_state);
                } else if !incomplete && concat_state.result == self.result {
                    return Some(concat_state)
                } // otherwise, drop this path
            }
        }

        None
    }

    fn concatenate_operands(a: u64, b: u64) -> u64 {
        let mut result = a;
        let mut b_reduce = b;
        while b_reduce != 0 {
            result *= 10;
            b_reduce /= 10;
        }
        result + b
    }
}

fn part1(equations: &Vec<CalibrationEquation>) {
    let total_calibration_result: u64 = equations.iter().filter_map(|equation| {
            equation.get_calibration_result(false).map(|res| res.result)
    }).sum();

    println!("Total calibration result with + and *: {}", total_calibration_result);
}

fn part2(equations: &Vec<CalibrationEquation>) {
    let total_calibration_result: u64 = equations.iter().filter_map(|equation| {
        equation.get_calibration_result(true).map(|res| res.result)
    }).sum();

    println!("Total calibration result with +, *, and ||: {}", total_calibration_result);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let equations = util::file::read_lines_to_type::<CalibrationEquation>(file_path)?;

    part1(&equations);
    part2(&equations);

    Ok(())
}