use std::convert::TryFrom;

#[derive(Debug)]
pub struct Instruction {
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
pub enum InstructionCode {
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
pub enum ParameterMode {
    Position,
    Immediate
}

#[derive(Debug)]
pub struct ParameterExtension {
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

pub struct Program {
    memory: Vec<i32>,
    pc: usize,
    inputs: Vec<i32>
}

impl From<String> for Program {
    fn from(program_string: String) -> Self {
        Program {
            memory: program_string.split(",").map(|s| s.parse::<i32>().unwrap()).collect(),
            pc: 0,
            inputs: Vec::new()
        }
    }
}

pub enum ProgramResult {
    Output(i32),
    Complete
}

impl Program {
    pub fn run(&mut self, inputs: &mut Vec<i32>) -> Vec<i32> {
        let mut codes: Vec<i32> = Vec::new();

        self.append_inputs(inputs);

        loop {
            let code = match self.run_until_event() {
                ProgramResult::Output(result) => result,
                ProgramResult::Complete => break
            };

            codes.push(code);
        }

        codes
    }

    pub fn append_inputs(&mut self, inputs: &mut Vec<i32>) {
        self.inputs.append(inputs);
    }

    pub fn run_until_event(&mut self) -> ProgramResult {
        loop {
            let instruction = Instruction::from(*self.memory.get(self.pc).unwrap());
            let mut result: Option<ProgramResult> = None;

            self.pc = match instruction.code {
                InstructionCode::Addition => {
                    let p1 = self.parameter_for(self.pc, 1, &instruction.modes);
                    let p2 = self.parameter_for(self.pc, 2, &instruction.modes);

                    self.store_result(p1 + p2, self.memory[self.pc + 3]);

                    self.pc + 4
                },

                InstructionCode::Multiplication => {
                    let p1 = self.parameter_for(self.pc, 1, &instruction.modes);
                    let p2 = self.parameter_for(self.pc, 2, &instruction.modes);
                    let p3 = self.memory[self.pc + 3];

                    self.store_result(p1 * p2, p3);

                    self.pc + 4
                },

                InstructionCode::Input => {
                    let next_input = self.inputs.remove(0);
                    self.store_result(next_input, self.memory[self.pc + 1]);

                    self.pc + 2
                },

                InstructionCode::Output => {
                    result = Some(ProgramResult::Output(self.parameter_for(self.pc, 1, &instruction.modes)));

                    self.pc + 2
                },

                InstructionCode::JumpIfTrue => {
                    let p1 = self.parameter_for(self.pc, 1, &instruction.modes);
                    let p2 = self.parameter_for(self.pc, 2, &instruction.modes);

                    match p1 {
                        0 => self.pc + 3,
                        _ => p2 as usize
                    }
                }

                InstructionCode::JumpIfFalse => {
                    let p1 = self.parameter_for(self.pc, 1, &instruction.modes);
                    let p2 = self.parameter_for(self.pc, 2, &instruction.modes);

                    match p1 {
                        0 => p2 as usize,
                        _ => self.pc + 3
                    }
                }

                InstructionCode::LessThan => {
                    let p1 = self.parameter_for(self.pc, 1, &instruction.modes);
                    let p2 = self.parameter_for(self.pc, 2, &instruction.modes);
                    let p3 = self.memory[self.pc + 3];
                    
                    let result = if p1 < p2 { 1 } else { 0 }; 
                    self.store_result(result, p3);

                    self.pc + 4
                }

                InstructionCode::Equals => {
                    let p1 = self.parameter_for(self.pc, 1, &instruction.modes);
                    let p2 = self.parameter_for(self.pc, 2, &instruction.modes);
                    let p3 = self.memory[self.pc + 3];
                    
                    let result = if p1 == p2 { 1 } else { 0 }; 
                    self.store_result(result, p3);

                    self.pc + 4
                }

                InstructionCode::Finish => {
                    result = Some(ProgramResult::Complete);

                    self.pc
                }
            };

            if let Some(event) = result {
                return event;
            }
        }
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

#[test]
fn test_program() {
    use std::fs;

    let input = fs::read_to_string("test.txt").unwrap();
    let mut program = Program::from(input);
    let diagnostic_codes = program.run(&mut vec!(8));

    assert_eq!(*diagnostic_codes.last().unwrap(), 1000);
}
