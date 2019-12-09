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
        Amplifier {
            program: Program::from(program_code),
            phase_setting
        }
    }

    fn run(&mut self, input: i32) -> i32 {
        let inputs: Vec<i32> = vec!(self.phase_setting, input);
        let outputs = self.program.run(inputs);
        
        *outputs.last().unwrap()
    }
}

struct SweepGenerator {
    index: usize
}

impl SweepGenerator {
    fn new() -> Self {
        SweepGenerator { index: 0 } 
    }
}

impl Iterator for SweepGenerator {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 5 * 4 * 3 * 2 { return None; }

        let mut possible: Vec<i32> = vec!(0,1,2,3,4);
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
    fn compute(code: String) -> Self {
        let mut value = -1000;
        let mut phase_settings: Vec<i32> = Vec::new();
        let mut generator = SweepGenerator::new();
    
        for settings in generator {
            let result = run_amplifier_array(code.clone(), &settings);
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

fn main() {
    let code = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", SweepResult::compute(code));
}

#[test]
fn test_amp_1() {
    let phase_settings: Vec<i32> = vec!(4,3,2,1,0);
    let program = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    let results = SweepResult::compute(program);
    assert_eq!(results.value, 43210);
    assert_eq!(results.phase_settings, phase_settings);
}

#[test]
fn test_amp_2() {
    let phase_settings: Vec<i32> = vec!(0,1,2,3,4);
    let program = String::from("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
    let results = SweepResult::compute(program);
    assert_eq!(results.value, 54321);
    assert_eq!(results.phase_settings, phase_settings);
}

#[test]
fn test_amp_3() {
    let phase_settings: Vec<i32> = vec!(1,0,4,3,2);
    let program = String::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
    let results = SweepResult::compute(program);
    assert_eq!(results.value, 65210);
    assert_eq!(results.phase_settings, phase_settings);
}
