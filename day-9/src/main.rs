use std::fs;
use intcode::Program;

fn main() {
    let code = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", Program::from(code).run(&mut vec!(1)));
}

#[test]
fn test_p1_1() {
    let program = String::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    let output = Program::from(program).run(&mut vec!());
    assert_eq!(output, vec!(109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99));
}

#[test]
fn test_p1_2() {
    let program = String::from("1102,34915192,34915192,7,4,7,99,0");
    let output = Program::from(program).run(&mut vec!());
    assert_eq!(*output.last().unwrap() > 999_9999_9999_9999, true);
}

#[test]
fn test_p1_3() {
    let program = String::from("104,1125899906842624,99");
    let output = Program::from(program).run(&mut vec!());
    assert_eq!(*output.last().unwrap(), 1125899906842624);
}