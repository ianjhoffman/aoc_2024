use util::res::Result;

#[derive(Clone)]
struct SystemState {
    registers: [u64; 3],
    instruction_pointer: usize,
    output: Vec<u8>,
}

impl SystemState {
    fn new(registers: [u64; 3]) -> SystemState {
        SystemState{registers, instruction_pointer: 0, output: vec![]}
    }

    fn apply_instruction(&mut self, instruction: u8, operand: u8, allow_jump: bool) {
        match instruction {
            0 => { // adv
                self.registers[0] = self.evaluate_division(operand);
            },
            1 => { // bxl
                self.registers[1] = self.registers[1] ^ u64::from(operand);
            },
            2 => { // bst
                self.registers[1] = self.evaluate_combo_operand(operand) & 0b111;
            },
            3 => { // jnz
                if self.registers[0] != 0 && allow_jump {
                    self.instruction_pointer = usize::from(operand);
                    return;
                }
            },
            4 => { // bxc
                self.registers[1] = self.registers[1] ^ self.registers[2];
            },
            5 => { // out
                self.output.push((self.evaluate_combo_operand(operand) & 0b111) as u8);
            },
            6 => { // bdv
                self.registers[1] = self.evaluate_division(operand);
            },
            7 => { // cdv
                self.registers[2] = self.evaluate_division(operand);
            },
            _ => {}, // invalid, do nothing
        }

        self.instruction_pointer += 2;
    }

    fn evaluate_combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => u64::from(operand),
            4..=6 => self.registers[(operand - 4) as usize],
            _ => 0 // invalid
        }
    }

    fn evaluate_division(&self, operand: u8) -> u64 {
        self.registers[0] >> self.evaluate_combo_operand(operand)
    }
}

struct Computer {
    state: SystemState,
    instructions: Vec<u8>,
}

impl Computer {
    fn from_lines(lines: &Vec<String>) -> Result<Computer> {
        if lines.len() != 5 {
            return Err(From::from("Invalid # of lines"));
        }

        let system_state = SystemState::new([
            Computer::parse_register_line(&lines[0], "Register A: ")?,
            Computer::parse_register_line(&lines[1], "Register B: ")?,
            Computer::parse_register_line(&lines[2], "Register C: ")?,
        ]);

        Ok(Computer{
            state: system_state,
            instructions: Computer::parse_program_line(&lines[4])?,
        })
    }

    fn parse_register_line(line: &String, prefix: &str) -> Result<u64> {
        if let Some(raw_val) = line.strip_prefix(prefix) {
            Ok(raw_val.parse::<u64>()?)
        } else {
            Err(From::from(format!("Invalid register line: {}", line)))
        }
    }

    fn parse_program_line(line: &String) -> Result<Vec<u8>> {
        if let Some(raw_program) = line.strip_prefix("Program: ") {
            raw_program.split(',').map(|raw_val| {
                raw_val.parse::<u8>().map(|v| v & 0b111).map_err(|e| e.into())
            }).collect()
        } else {
            Err(From::from(format!("Invalid program line: {}", line)))
        }
    }

    fn run_program_with_state(&self, init_state: &SystemState) -> SystemState {
        let mut state = init_state.clone();
        while state.instruction_pointer < self.instructions.len() - 1 {
            let ip = state.instruction_pointer;
            state.apply_instruction(self.instructions[ip], self.instructions[ip + 1], true);
        }

        state
    }

    fn run_program(&self) -> SystemState {
        self.run_program_with_state(&self.state)
    }

    fn run_instructions_once_with_state(&self, init_state: &SystemState) -> SystemState {
        let mut state = init_state.clone();
        while state.instruction_pointer < self.instructions.len() - 1 {
            let ip = state.instruction_pointer;
            state.apply_instruction(self.instructions[ip], self.instructions[ip + 1], false);
        }

        state
    }
}

fn part1(computer: &Computer) {
    let finished_state = computer.run_program();
    let output = finished_state.output.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",");

    println!("Output of program: {}", output);
}

fn part2(computer: &Computer) {
    //
    // This solution relies on a couple observations and assumptions
    // that may or may not only apply to my input. Specifically:
    //
    //    1. Each iteration of the full sequence of instructions only
    //       depends on the value register A.
    //    2. Each iteration of the full sequence of instructions prints
    //       one digit and returns to the first instruction.
    //    3. Each iteration of the full sequence shifts off the 3
    //       least significant bits of register A.
    //

    let mut init_state = computer.state.clone();
    let mut possible_values: Vec<u64> = vec![0];
    for digit_to_find in computer.instructions.iter().rev() {
        let mut new_possible_values: Vec<u64> = vec![];
        for possible_value in possible_values.iter() {
            for next_3_bits in 0b000..=0b111 {
                // Initialize register A to expanded value (with 3 new least significant bits)
                let new_possible_value = (possible_value << 3) | next_3_bits;
                init_state.registers[0] = new_possible_value;

                // Get what we printed from running the program for one loop with this state
                let out_state = computer.run_instructions_once_with_state(&init_state);
                if out_state.output.last() == Some(digit_to_find) {
                    new_possible_values.push(new_possible_value);
                }
            }
        }
        possible_values = new_possible_values;
    }

    let min_possible_value = possible_values.iter().fold(u64::MAX, |acc, e| acc.min(*e));
    println!(
        "Out of {} possible values for A, the lowest to produce a quine is {}",
        possible_values.len(), min_possible_value
    );
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let lines = util::file::read_lines_raw(file_path)?;
    let computer = Computer::from_lines(&lines)?;

    part1(&computer);
    part2(&computer);

    Ok(())
}