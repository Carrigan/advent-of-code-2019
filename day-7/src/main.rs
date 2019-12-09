use std::fs;
use intcode::Program;

struct Amplifier {
    program: Program,
    phase_setting: i32
}

impl std::fmt::Debug for Amplifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Amplifier <ps: {}>", self.phase_setting)
    }
}

impl Amplifier {
    fn new(program_code: String, phase_setting: i32) -> Self {
        let mut program = Program::from(program_code);
        program.append_inputs(&mut vec!(phase_setting));
        Amplifier { program, phase_setting }
    }

    fn run(&mut self, input: i32) -> i32 {
        let mut inputs: Vec<i32> = vec!(input);
        let outputs = self.program.run(&mut inputs);
        
        *outputs.last().unwrap()
    }

    fn run_until_event(&mut self, input: i32) -> intcode::ProgramResult {
        let mut inputs: Vec<i32> = vec!(input);
        self.program.append_inputs(&mut inputs);

        self.program.run_until_event()     
    }
}

struct SweepGenerator {
    possibilities: Vec<i32>,
    index: usize
}

impl SweepGenerator {
    fn new(possibilities: Vec<i32>) -> Self {
        SweepGenerator { index: 0 , possibilities } 
    }
}

impl Iterator for SweepGenerator {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 5 * 4 * 3 * 2 { return None; }

        let mut possible: Vec<i32> = self.possibilities.clone();
        let mut current: Vec<i32> = Vec::new();
        
        // Save our index and increment it
        let mut index = self.index;
        self.index += 1;

        // Iterate through
        for n in 0..5 {
            let current_divisor = 5 - n;
            let current_index = index % current_divisor;
            current.push(possible.remove(current_index));
            index = index / current_divisor;
        }

        Some(current)
    }
}

#[derive(Debug)]
struct SweepResult {
    phase_settings: Vec<i32>,
    value: i32
}

impl SweepResult {
    fn run_array(code: String, possiblities: Vec<i32>) -> Self {
        let mut value = -1000;
        let mut phase_settings: Vec<i32> = Vec::new();
        let generator = SweepGenerator::new(possiblities);
    
        for settings in generator {
            let result = run_amplifier_array(code.clone(), &settings);
            if result > value {
                value = result;
                phase_settings = settings;
            }
        }
    
        SweepResult { phase_settings, value }
    }

    fn run_feedback_array(code: String, possiblities: Vec<i32>) -> Self {
        let mut value = -1000;
        let mut phase_settings: Vec<i32> = Vec::new();
        let generator = SweepGenerator::new(possiblities);
    
        for settings in generator {
            let result = run_with_feedback(code.clone(), &settings);
            if result > value {
                value = result;
                phase_settings = settings;
            }
        }
    
        SweepResult { phase_settings, value }
    }
}

fn run_amplifier_array(code: String, phase_settings: &Vec<i32>) -> i32 {
    let mut value = 0;
    let mut amplifiers: Vec<Amplifier> = phase_settings
        .into_iter()
        .map(|ps| Amplifier::new(code.clone(), *ps))
        .collect();

    for amplifier in &mut amplifiers {
        value = amplifier.run(value);
    }

    value
}

fn run_with_feedback(code: String, phase_settings: &Vec<i32>) -> i32 {
    let mut value = 0;
    let mut amplifiers: Vec<Amplifier> = phase_settings
        .into_iter()
        .map(|ps| Amplifier::new(code.clone(), *ps))
        .collect();

    loop {
        for amplifier in &mut amplifiers {
            value = match amplifier.run_until_event(value) {
                intcode::ProgramResult::Output(n) => n,
                intcode::ProgramResult::Complete => return value
            };
        }
    }
}

fn main() {
    let code = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", SweepResult::run_feedback_array(code, vec!(5,6,7,8,9)));
}

#[test]
fn test_amp_1() {
    let phase_settings: Vec<i32> = vec!(4,3,2,1,0);
    let program = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    let results = SweepResult::run_array(program, vec!(0,1,2,3,4));
    assert_eq!(results.value, 43210);
    assert_eq!(results.phase_settings, phase_settings);
}

#[test]
fn test_amp_2() {
    let phase_settings: Vec<i32> = vec!(0,1,2,3,4);
    let program = String::from("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
    let results = SweepResult::run_array(program, vec!(0,1,2,3,4));
    assert_eq!(results.value, 54321);
    assert_eq!(results.phase_settings, phase_settings);
}

#[test]
fn test_amp_3() {
    let phase_settings: Vec<i32> = vec!(1,0,4,3,2);
    let program = String::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
    let results = SweepResult::run_array(program, vec!(0,1,2,3,4));
    assert_eq!(results.value, 65210);
    assert_eq!(results.phase_settings, phase_settings);
}

#[test]
fn test_amp_feedback_1() {
    let phase_settings: Vec<i32> = vec!(9,8,7,6,5);
    let program = String::from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
    let results = SweepResult::run_feedback_array(program, vec!(5,6,7,8,9));
    assert_eq!(results.value, 139629729);
    assert_eq!(results.phase_settings, phase_settings);
}

#[test]
fn test_amp_feedback_2() {
    let phase_settings: Vec<i32> = vec!(9,7,8,5,6);
    let program = String::from("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
    let results = SweepResult::run_feedback_array(program, vec!(5,6,7,8,9));
    assert_eq!(results.value, 18216);
    assert_eq!(results.phase_settings, phase_settings);
}