use std::fs;

fn calculate_fuel(mass: u32) -> u32 {
    if mass < 9 { return 0; }

    let fuel = mass / 3 - 2;
    fuel + calculate_fuel(fuel)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut fuel_total = 0;

    for entry in input.lines() {
        let num = entry.parse::<u32>().unwrap();
        fuel_total += calculate_fuel(num);
    }

    println!("Total needed: {:?}", fuel_total)
}

#[test]
fn test_mass_12() {
    assert_eq!(calculate_fuel(12), 2);
}

#[test]
fn test_mass_14() {
    assert_eq!(calculate_fuel(14), 2);
}

#[test]
fn test_mass_1969() {
    assert_eq!(calculate_fuel(1969), 966);
}

#[test]
fn test_mass_100756() {
    assert_eq!(calculate_fuel(100756), 50346);
}
