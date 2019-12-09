use std::fs;
use intcode::Program;

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
