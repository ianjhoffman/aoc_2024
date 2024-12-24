#[macro_use] extern crate lazy_static;
use util::res::Result;
use regex::Regex;

// Returns gcd, x, y such that ax + by = gcd
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
	if a == 0 { return (b, 0, 1); }
	let (gcd, x, y) = extended_gcd(b % a, a);
    (gcd, (y - (b/a) * x), x)
}

struct Rational {
    numerator: i64,
    denominator: i64,
}

impl Rational {
    fn new(numerator: i64, denominator: i64) -> Rational {
        Rational{numerator, denominator}
    }

    fn mul_add(&self, other: &Rational, factor: &Rational) -> Rational {
        let to_add_numerator = other.numerator * factor.numerator;
        let to_add_denominator = other.denominator * factor.denominator;

        let numerator = self.numerator * to_add_denominator + self.denominator * to_add_numerator;
        let denominator = self.denominator * to_add_denominator;
        let gcd = extended_gcd(numerator, denominator).0;

        Rational{numerator: numerator / gcd, denominator: denominator / gcd}
    }

    fn mul(&self, factor: &Rational) -> Rational {
        let numerator = self.numerator * factor.numerator;
        let denominator = self.denominator * factor.denominator;
        let gcd = extended_gcd(numerator, denominator).0;

        Rational{numerator: numerator / gcd, denominator: denominator / gcd}
    }

    fn negate(&self) -> Rational {
        Rational{numerator: -self.numerator, denominator: self.denominator}
    }

    fn recip(&self) -> Rational {
        Rational{numerator: self.denominator, denominator: self.numerator}
    }
}

#[derive(Clone)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl ClawMachine {
    fn new(button_a: (i64, i64), button_b: (i64, i64), prize: (i64, i64)) -> ClawMachine {
        ClawMachine{button_a, button_b, prize}
    }

    // returns # of A and B presses for the optimal solution, if one exists
    fn solve(&self) -> Option<(usize, usize)> {
        // Gaussian row reduction
        let mut row_1: [Rational; 3] = [
            Rational::new(self.button_a.0, 1),
            Rational::new(self.button_b.0, 1),
            Rational::new(self.prize.0, 1)
        ];
        let mut row_2: [Rational; 3] = [
            Rational::new(self.button_a.1, 1),
            Rational::new(self.button_b.1, 1),
            Rational::new(self.prize.1, 1)
        ];

        // Remove x from row 2
        let factor_1 = Rational::new(self.button_a.1, self.button_a.0);
        for i in 0..=2 {
            row_2[i] = row_2[i].mul_add(&row_1[i].negate(), &factor_1);
        }

        // Normalize row 2
        let factor_2 = row_2[1].recip();
        for i in 0..=2 {
            row_2[i] = row_2[i].mul(&factor_2);
        }

        // Eliminate y from row 1
        let factor_3 = row_1[1].negate();
        for i in 0..=2 {
            row_1[i] = row_1[i].mul_add(&row_2[i], &factor_3);
        }

        // Normalize row 1
        let factor_4 = row_1[0].recip();
        for i in 0..=2 {
            row_1[i] = row_1[i].mul(&factor_4);
        }

        // No integer solution
        if row_1[2].denominator > 1 || row_2[2].denominator > 1 {
            return None
        }

        // No solution with two positive values
        if row_1[2].numerator < 0 || row_2[2].numerator < 0 {
            return None
        }

        // We found 2 positive integer solutions!
        return Some((row_1[2].numerator as usize, row_2[2].numerator as usize))
    }
}

fn parse_claw_machines_from_lines(lines: &Vec<String>) -> Result<Vec<ClawMachine>> {
    lazy_static! {
        static ref BUTTON_A_REGEX: Regex = Regex::new(r"^Button A: X\+(\d+), Y\+(\d+)$").unwrap();
        static ref BUTTON_B_REGEX: Regex = Regex::new(r"^Button B: X\+(\d+), Y\+(\d+)$").unwrap();
        static ref PRIZE_REGEX: Regex = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();
    }

    let mut claw_machines: Vec<ClawMachine> = vec![];
    for chunk in lines.chunks(4) {
        if chunk.len() < 3 {
            return Err(From::from("Not enough lines".to_owned()));
        }

        let button_a = match BUTTON_A_REGEX.captures(chunk[0].as_str()) {
            Some(caps) => {
                let x_val = caps.get(1).unwrap().as_str().parse::<i64>()?;
                let y_val = caps.get(2).unwrap().as_str().parse::<i64>()?;
                (x_val, y_val)
            },
            None => return Err(From::from(format!("Invalid line: {}", chunk[0])))
        };

        let button_b = match BUTTON_B_REGEX.captures(chunk[1].as_str()) {
            Some(caps) => {
                let x_val = caps.get(1).unwrap().as_str().parse::<i64>()?;
                let y_val = caps.get(2).unwrap().as_str().parse::<i64>()?;
                (x_val, y_val)
            },
            None => return Err(From::from(format!("Invalid line: {}", chunk[1])))
        };

        let prize = match PRIZE_REGEX.captures(chunk[2].as_str()) {
            Some(caps) => {
                let x_val = caps.get(1).unwrap().as_str().parse::<i64>()?;
                let y_val = caps.get(2).unwrap().as_str().parse::<i64>()?;
                (x_val, y_val)
            },
            None => return Err(From::from(format!("Invalid line: {}", chunk[2])))
        };

        claw_machines.push(ClawMachine::new(button_a, button_b, prize));
    }

    Ok(claw_machines)
}

fn part1(claw_machines: &Vec<ClawMachine>) {
    let num_tokens: usize = claw_machines.iter().map(|claw_machine| {
        match claw_machine.solve() {
            Some((a_presses, b_presses)) => {
                (3 * a_presses) + b_presses
            },
            None => 0,
        }
    }).sum();

    println!("Fewest tokens to win all possible prizes on original machines: {}", num_tokens);
}

fn part2(claw_machines: &Vec<ClawMachine>) {
    let modified_claw_machines: Vec<ClawMachine> = claw_machines.iter().map(|claw_machine| {
        let mut modified_claw_machine = claw_machine.clone();
        modified_claw_machine.prize.0 += 10000000000000;
        modified_claw_machine.prize.1 += 10000000000000;

        modified_claw_machine
    }).collect();

    let num_tokens: usize = modified_claw_machines.iter().map(|claw_machine| {
        match claw_machine.solve() {
            Some((a_presses, b_presses)) => {
                (3 * a_presses) + b_presses
            },
            None => 0,
        }
    }).sum();

    println!("Fewest tokens to win all possible prizes on modified machines: {}", num_tokens);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let lines = util::file::read_lines_raw(file_path)?;
    let claw_machines = parse_claw_machines_from_lines(&lines)?;

    part1(&claw_machines);
    part2(&claw_machines);

    Ok(())
}