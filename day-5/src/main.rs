use std::fs;
use std::convert::TryFrom;

#[derive(Debug)]
struct Instruction {
    code: InstructionCode,
    modes: ParameterExtension
}

impl From<i32> for Instruction {
    fn from(instruction: i32) -> Instruction {
        let code = instruction % 100;
        let extensions = instruction / 100;

        Instruction {
            code: InstructionCode::try_from(code).unwrap(),
            modes: ParameterExtension::from(extensions)
        }
    }
}

#[derive(PartialEq, Debug)]
enum InstructionCode {
    Addition,
    Multiplication,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Finish
}

impl TryFrom<i32> for InstructionCode {
    type Error = String;

    fn try_from(param: i32) -> Result<Self, Self::Error> {
        match param {
            1 => Ok(InstructionCode::Addition),
            2 => Ok(InstructionCode::Multiplication),
            3 => Ok(InstructionCode::Input),
            4 => Ok(InstructionCode::Output),
            5 => Ok(InstructionCode::JumpIfTrue),
            6 => Ok(InstructionCode::JumpIfFalse),
            7 => Ok(InstructionCode::LessThan),
            8 => Ok(InstructionCode::Equals),
            99 => Ok(InstructionCode::Finish),
            _ => Err(String::from("Invalid op code"))
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ParameterMode {
    Position,
    Immediate
}

#[derive(Debug)]
struct ParameterExtension {
    modes: Vec<ParameterMode>
}

impl From<i32> for ParameterExtension {
    fn from(params: i32) -> Self {
        let mut modes: Vec<ParameterMode> = Vec::new();
        let mut n = params;

        while n > 0 {
            let current_int = n % 10;
            let current_mode = match current_int {
                1 => ParameterMode::Immediate,
                _ => ParameterMode::Position
            };

            modes.push(current_mode);

            n = n / 10;
        }

        ParameterExtension { modes }
    }
}

impl ParameterExtension {
    fn at_position(&self, position: usize) -> ParameterMode {
        match self.modes.get(position) {
            Some(mode) => *mode,
            None => ParameterMode::Position
        }
    }
}

struct Program {
    memory: Vec<i32>
}

impl From<String> for Program {
    fn from(program_string: String) -> Self {
        Program {
            memory: program_string.split(",").map(|s| s.parse::<i32>().unwrap()).collect()
        }
    }
}

impl Program {
    fn run(&mut self, inputs: Vec<i32>) -> Vec<i32> {
        println!("Executing program with length {:?}", self.memory.len());

        let mut pc = 0;
        let mut input_index = 0;
        let mut codes: Vec<i32> = Vec::new();

        'program: loop {
            let instruction = Instruction::from(*self.memory.get(pc).unwrap());
            println!("{:?} -> {:?}", pc, instruction);

            pc = match instruction.code {
                InstructionCode::Addition => {
                    let p1 = self.parameter_for(pc, 1, &instruction.modes);
                    let p2 = self.parameter_for(pc, 2, &instruction.modes);

                    self.store_result(p1 + p2, self.memory[pc + 3]);

                    pc + 4
                },

                InstructionCode::Multiplication => {
                    let p1 = self.parameter_for(pc, 1, &instruction.modes);
                    let p2 = self.parameter_for(pc, 2, &instruction.modes);
                    let p3 = self.memory[pc + 3];

                    self.store_result(p1 * p2, p3);

                    pc + 4
                },

                InstructionCode::Input => {
                    self.store_result(inputs[input_index], self.memory[pc + 3]);
                    input_index += 1;

                    pc + 2
                },

                InstructionCode::Output => {
                    codes.push(self.parameter_for(pc, 1, &instruction.modes));

                    pc + 2
                },

                InstructionCode::JumpIfTrue => {
                    let p1 = self.parameter_for(pc, 1, &instruction.modes);
                    let p2 = self.parameter_for(pc, 2, &instruction.modes);

                    match p1 {
                        0 => pc + 3,
                        _ => p2 as usize
                    }
                }

                InstructionCode::JumpIfFalse => {
                    let p1 = self.parameter_for(pc, 1, &instruction.modes);
                    let p2 = self.parameter_for(pc, 2, &instruction.modes);

                    match p1 {
                        0 => p2 as usize,
                        _ => pc + 3
                    }
                }

                InstructionCode::LessThan => {
                    let p1 = self.parameter_for(pc, 1, &instruction.modes);
                    let p2 = self.parameter_for(pc, 2, &instruction.modes);
                    let p3 = self.memory[pc + 3];
                    
                    let result = if p1 < p2 { 1 } else { 0 }; 
                    self.store_result(result, p3);

                    pc + 4
                }

                InstructionCode::Equals => {
                    let p1 = self.parameter_for(pc, 1, &instruction.modes);
                    let p2 = self.parameter_for(pc, 2, &instruction.modes);
                    let p3 = self.memory[pc + 3];
                    
                    let result = if p1 == p2 { 1 } else { 0 }; 
                    self.store_result(result, p3);

                    pc + 4
                }

                InstructionCode::Finish => {
                    break 'program
                }
            }
        }

        codes
    }

    fn parameter_for(&self, pc: usize, index: usize, ext: &ParameterExtension) -> i32 {
        let mode = ext.at_position(index - 1);
        let value = self.memory[pc + index];

        match mode {
            ParameterMode::Position => self.memory[value as usize],
            _ => value
        }
    }

    fn store_result(&mut self, result: i32, position: i32) {
        self.memory[position as usize] = result;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut program = Program::from(input);
    let diagnostic_codes = program.run(vec!(5));

    println!("{:?}", diagnostic_codes);
}

#[test]
fn test_program() {
    let input = fs::read_to_string("test.txt").unwrap();
    let mut program = Program::from(input);
    let diagnostic_codes = program.run(vec!(8));

    assert_eq!(*diagnostic_codes.last().unwrap(), 1000);
}
