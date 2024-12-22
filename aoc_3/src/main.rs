use util::res::Result;

#[derive(Debug)]
enum ScanState {
    Garbage,

    // Multiply states
    AwaitingU,
    AwaitingL,
    AwaitingMulLeftParens,
    AwaitingNum1Digit1,
    AwaitingNum1Digit2OrComma,
    AwaitingNum1Digit3OrComma,
    AwaitingComma,
    AwaitingNum2Digit1,
    AwaitingNum2Digit2OrRightParens,
    AwaitingNum2Digit3OrRightParens,
    AwaitingMulRightParens,

    // Do/Don't states
    AwaitingO,
    AwaitingNOrLeftParens,
    AwaitingApostrophe,
    AwaitingT,
    AwaitingDoDontLeftParens,
    AwaitingDoDontRightParens,
}

#[derive(Debug)]
struct Multiply {
    a: u32,
    b: u32,
    enabled: bool,
}

fn extract_multiplies(memory: &String) -> Vec<Multiply> {
    let mut ret: Vec<Multiply> = vec![];
    let mut state = ScanState::Garbage;
    let mut pending_enabled: bool = true;
    let mut enabled: bool = true;
    let mut num1_str: String = "".to_owned();
    let mut num2_str: String = "".to_owned();

    // In this house we love a cute little state machine
    for c in memory.chars() {
        match (&state, c) {
            (ScanState::AwaitingU, 'u') => state = ScanState::AwaitingL,
            (ScanState::AwaitingL, 'l') => state = ScanState::AwaitingMulLeftParens,
            (ScanState::AwaitingMulLeftParens, '(') => state = ScanState::AwaitingNum1Digit1,
            (ScanState::AwaitingNum1Digit1, '0'..='9') => {
                num1_str = c.to_string();
                state = ScanState::AwaitingNum1Digit2OrComma;
            },
            (ScanState::AwaitingNum1Digit2OrComma, '0'..='9') => {
                num1_str.push(c);
                state = ScanState::AwaitingNum1Digit3OrComma;
            },
            (ScanState::AwaitingNum1Digit3OrComma, '0'..='9') => {
                num1_str.push(c);
                state = ScanState::AwaitingComma;
            },
            (ScanState::AwaitingNum1Digit2OrComma | ScanState::AwaitingNum1Digit3OrComma | ScanState::AwaitingComma, ',') => {
                state = ScanState::AwaitingNum2Digit1;
            },
            (ScanState::AwaitingNum2Digit1, '0'..='9') => {
                num2_str = c.to_string();
                state = ScanState::AwaitingNum2Digit2OrRightParens;
            },
            (ScanState::AwaitingNum2Digit2OrRightParens, '0'..='9') => {
                num2_str.push(c);
                state = ScanState::AwaitingNum2Digit3OrRightParens;
            },
            (ScanState::AwaitingNum2Digit3OrRightParens, '0'..='9') => {
                num2_str.push(c);
                state = ScanState::AwaitingMulRightParens;
            },
            (ScanState::AwaitingNum2Digit2OrRightParens | ScanState::AwaitingNum2Digit3OrRightParens | ScanState::AwaitingMulRightParens, ')') => {
                // We found a full multiply instruction!
                ret.push(Multiply{
                    a: num1_str.parse::<u32>().unwrap(),
                    b: num2_str.parse::<u32>().unwrap(),
                    enabled: enabled,
                });
                state = ScanState::Garbage;
            },
            (ScanState::AwaitingO, 'o') => state = ScanState::AwaitingNOrLeftParens,
            (ScanState::AwaitingNOrLeftParens, '(') => {
                // Past the point of no return towards a `do()` instruction
                pending_enabled = true;
                state = ScanState::AwaitingDoDontRightParens;
            },
            (ScanState::AwaitingNOrLeftParens, 'n') => {
                // Past the point of no return towards a `don't()` instruction
                pending_enabled = false;
                state = ScanState::AwaitingApostrophe;
            },
            (ScanState::AwaitingApostrophe, '\'') => state = ScanState::AwaitingT,
            (ScanState::AwaitingT, 't') => state = ScanState::AwaitingDoDontLeftParens,
            (ScanState::AwaitingDoDontLeftParens, '(') => state = ScanState::AwaitingDoDontRightParens,
            (ScanState::AwaitingDoDontRightParens, ')') => {
                // We found a full `do()`/`don't()`` instruction!
                enabled = pending_enabled;
                state = ScanState::Garbage;
            },
            (_, 'm') => state = ScanState::AwaitingU, // Started `mul()`
            (_, 'd') => state = ScanState::AwaitingO, // Started `do()`` or `don't()`
            _ => state = ScanState::Garbage
        }
    }

    ret
}

fn part1(multiplies: &Vec<Multiply>) {
    let total: u64 = multiplies.iter().map(|multiply| u64::from(multiply.a * multiply.b)).sum();
    
    println!("Found {:?} multiplies, total of products = {:?}", multiplies.len(), total);
}

fn part2(multiplies: &Vec<Multiply>) {
    let enabled_products = multiplies.iter().filter_map(|multiply| {
        if multiply.enabled { Some(u64::from(multiply.a * multiply.b)) } else { None }
    }).collect::<Vec<u64>>();
    let total: u64 = enabled_products.iter().sum();

    println!("Found {:?} enabled multiplies, total of products = {:?}", enabled_products.len(), total);
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let lines = util::file::read_lines_raw(file_path)?;
    let multiplies =  extract_multiplies(&lines.join(""));

    part1(&multiplies);
    part2(&multiplies);

    Ok(())
}