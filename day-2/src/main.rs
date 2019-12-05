use std::fs;

struct ProgramState {
    status: ProgramStatus,
    index: usize,
    variable_1: u32,
    variable_2: u32
}

#[derive(PartialEq)]
enum ProgramStatus {
    Idle,
    AdditionVariableOne,
    AdditionVariableTwo,
    AdditionResultLocation,
    MultiplicationVariableOne,
    MultiplicationVariableTwo,
    MultiplicationResultLocation,
    Finished
}

fn string_to_program(program_string: String) -> Vec<u32> {
    return program_string.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
}

fn run_program(program: &mut Vec<u32>) {
    let mut state = ProgramState { status: ProgramStatus::Idle, index: 0, variable_1: 0, variable_2: 0 };

    while state.status != ProgramStatus::Finished {
        let operation = program[state.index];

        state = match state.status {
            // Program state operations
            ProgramStatus::Idle => {
                match operation {
                    1 => ProgramState { status: ProgramStatus::AdditionVariableOne, ..state },
                    2 => ProgramState { status: ProgramStatus::MultiplicationVariableOne, ..state },
                    _ => ProgramState { status: ProgramStatus:: Finished, ..state}
                }
            },

            // Addition operations
            ProgramStatus::AdditionVariableOne => {
                ProgramState {
                    status: ProgramStatus::AdditionVariableTwo,
                    variable_1: program[operation as usize],
                    ..state
                }
            },
            ProgramStatus::AdditionVariableTwo => {
                ProgramState {
                    status: ProgramStatus::AdditionResultLocation,
                    variable_2: program[operation as usize],
                    ..state
                }
            }
            ProgramStatus::AdditionResultLocation => {
                program[operation as usize] = state.variable_1 + state.variable_2;
                ProgramState { status: ProgramStatus::Idle , ..state }
            }

            // Multiplication operations
            ProgramStatus::MultiplicationVariableOne => {
                ProgramState {
                    status: ProgramStatus::MultiplicationVariableTwo,
                    variable_1: program[operation as usize],
                    ..state
                }
            },
            ProgramStatus::MultiplicationVariableTwo => {
                ProgramState {
                    status: ProgramStatus::MultiplicationResultLocation,
                    variable_2: program[operation as usize],
                    ..state
                }
            }
            ProgramStatus::MultiplicationResultLocation => {
                program[operation as usize] = state.variable_1 * state.variable_2;
                ProgramState { status: ProgramStatus::Idle , ..state }
            }

            // Finished
            ProgramStatus::Finished => {
                panic!("Something has gone terribly wrong...")
            }
        };

        state.index += 1;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut program = string_to_program(input);

    // 1202 alarm!
    program[1] = 12;
    program[2] = 2;

    run_program(&mut program);
    println!("{:?}", program);
}

fn compare_outputs(program: &mut Vec<u32>, expected: &Vec<u32>) {
    run_program(program);
    for i in 0..program.len() {
        assert_eq!(program[i], expected[i]);
    }
}

#[test]
fn test_parser() {
    let mut program = string_to_program(String::from("1,2,3,4,5"));
    let expected = vec!(1,2,3,4,5);

    for i in 0..program.len() {
        assert_eq!(program[i], expected[i]);
    }
}

#[test]
fn test_case_1() {
    let mut program = vec!(1,0,0,0,99);
    let expected = vec!(2,0,0,0,99);
    compare_outputs(&mut program, &expected);
}

#[test]
fn test_case_2() {
    let mut program = vec!(2,3,0,3,99);
    let expected = vec!(2,3,0,6,99);
    compare_outputs(&mut program, &expected);
}

#[test]
fn test_case_3() {
    let mut program = vec!(2,4,4,5,99,0);
    let expected = vec!(2,4,4,5,99,9801);
    compare_outputs(&mut program, &expected);
}

#[test]
fn test_case_4() {
    let mut program = vec!(1,1,1,4,99,5,6,0,99);
    let expected = vec!(30,1,1,4,2,5,6,0,99);
    compare_outputs(&mut program, &expected);
}
