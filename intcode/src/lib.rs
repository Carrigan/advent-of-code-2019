use std::convert::TryFrom;

#[derive(Debug)]
pub struct Instruction {
    code: InstructionCode,
    modes: ParameterExtension
}

impl From<i64> for Instruction {
    fn from(instruction: i64) -> Instruction {
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
    RelativeBaseAdjust,
    Finish
}

impl TryFrom<i64> for InstructionCode {
    type Error = String;

    fn try_from(param: i64) -> Result<Self, Self::Error> {
        match param {
            1 => Ok(InstructionCode::Addition),
            2 => Ok(InstructionCode::Multiplication),
            3 => Ok(InstructionCode::Input),
            4 => Ok(InstructionCode::Output),
            5 => Ok(InstructionCode::JumpIfTrue),
            6 => Ok(InstructionCode::JumpIfFalse),
            7 => Ok(InstructionCode::LessThan),
            8 => Ok(InstructionCode::Equals),
            9 => Ok(InstructionCode::RelativeBaseAdjust),
            99 => Ok(InstructionCode::Finish),
            _ => Err(String::from("Invalid op code"))
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParameterMode {
    Position,
    Immediate,
    Relative
}

#[derive(Debug)]
pub struct ParameterExtension {
    modes: Vec<ParameterMode>
}

impl From<i64> for ParameterExtension {
    fn from(params: i64) -> Self {
        let mut modes: Vec<ParameterMode> = Vec::new();
        let mut n = params;

        while n > 0 {
            let current_int = n % 10;
            let current_mode = match current_int {
                1 => ParameterMode::Immediate,
                2 => ParameterMode::Relative,
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
    memory: Vec<i64>,
    pc: usize,
    inputs: Vec<i64>,
    relative_base: i64
}

impl From<String> for Program {
    fn from(program_string: String) -> Self {
        let mut memory: Vec<i64> = program_string.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
        for _ in 0..2000 { memory.push(0); }

        Program {
            memory,
            pc: 0,
            inputs: Vec::new(),
            relative_base: 0
        }
    }
}

pub enum ProgramResult {
    Output(i64),
    Complete
}

impl Program {
    pub fn run(&mut self, inputs: &mut Vec<i64>) -> Vec<i64> {
        let mut codes: Vec<i64> = Vec::new();

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

    pub fn append_inputs(&mut self, inputs: &mut Vec<i64>) {
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
                    let destination = self.destination_for(self.pc, 3, &instruction.modes);


                    self.store_result(p1 + p2, destination);

                    self.pc + 4
                },

                InstructionCode::Multiplication => {
                    let p1 = self.parameter_for(self.pc, 1, &instruction.modes);
                    let p2 = self.parameter_for(self.pc, 2, &instruction.modes);
                    let destination = self.destination_for(self.pc, 3, &instruction.modes);

                    self.store_result(p1 * p2, destination);

                    self.pc + 4
                },

                InstructionCode::Input => {
                    let next_input = self.inputs.remove(0);
                    let destination = self.destination_for(self.pc, 1, &instruction.modes);

                    self.store_result(next_input, destination);

                    self.pc + 2
                },

                InstructionCode::Output => {
                    let output = self.parameter_for(self.pc, 1, &instruction.modes);
                    result = Some(ProgramResult::Output(output));

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
                    let destination = self.destination_for(self.pc, 3, &instruction.modes);
                    
                    let result = if p1 < p2 { 1 } else { 0 }; 
                    self.store_result(result, destination);

                    self.pc + 4
                }

                InstructionCode::Equals => {
                    let p1 = self.parameter_for(self.pc, 1, &instruction.modes);
                    let p2 = self.parameter_for(self.pc, 2, &instruction.modes);
                    let destination = self.destination_for(self.pc, 3, &instruction.modes);
                    
                    let result = if p1 == p2 { 1 } else { 0 }; 
                    self.store_result(result, destination);

                    self.pc + 4
                }

                InstructionCode::RelativeBaseAdjust => {
                    let p1 = self.parameter_for(self.pc, 1, &instruction.modes);
                    self.relative_base = self.relative_base + p1;

                    self.pc + 2
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

    fn parameter_for(&self, pc: usize, index: usize, ext: &ParameterExtension) -> i64 {
        let mode = ext.at_position(index - 1);
        let value = self.memory[pc + index];

        match mode {
            ParameterMode::Position => self.memory[value as usize],
            ParameterMode::Relative => self.memory[(value + self.relative_base) as usize],
            _ => value
        }
    }

    fn destination_for(&self, pc: usize, index: usize, ext: &ParameterExtension) -> i64 {
        let mode = ext.at_position(index - 1);
        let value = self.memory[pc + index];

        match mode {
            ParameterMode::Relative => value + self.relative_base,
            _ => value
        }
    }

    fn store_result(&mut self, result: i64, position: i64) {
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
