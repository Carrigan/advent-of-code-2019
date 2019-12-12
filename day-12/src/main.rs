extern crate regex;
use regex::Regex;

#[derive(Debug, Clone)]
struct Body {
    x_pos: i64,
    y_pos: i64,
    z_pos: i64,
    x_vel: i64,
    y_vel: i64,
    z_vel: i64,
}

impl PartialEq for Body {
    fn eq(&self, other: &Self) -> bool {
        if other.x_vel != self.x_vel { return false; }
        if other.y_vel != self.y_vel { return false; }
        if other.z_vel != self.z_vel { return false; }
        if other.x_pos != self.x_pos { return false; }
        if other.y_pos != self.y_pos { return false; }
        if other.z_pos != self.z_pos { return false; }

        true
    }
}

impl From<&str> for Body {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
        let captures = re.captures(s).unwrap();

        Body {
            x_pos: captures[1].parse::<i64>().unwrap(),
            y_pos: captures[2].parse::<i64>().unwrap(),
            z_pos: captures[3].parse::<i64>().unwrap(),
            x_vel: 0,
            y_vel: 0,
            z_vel: 0
        }
    }
}

impl Body {
    fn energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn potential_energy(&self) -> i64 {
        let abs_x = if self.x_pos < 0 { -self.x_pos } else { self.x_pos };
        let abs_y = if self.y_pos < 0 { -self.y_pos } else { self.y_pos };
        let abs_z = if self.z_pos < 0 { -self.z_pos } else { self.z_pos };

        abs_x + abs_y + abs_z
    }

    fn kinetic_energy(&self) -> i64 {
        let abs_x = if self.x_vel < 0 { -self.x_vel } else { self.x_vel };
        let abs_y = if self.y_vel < 0 { -self.y_vel } else { self.y_vel };
        let abs_z = if self.z_vel < 0 { -self.z_vel } else { self.z_vel };

        abs_x + abs_y + abs_z
    }

    fn apply_gravity(&mut self, other: &Body) {
        self.x_vel = if self.x_pos < other.x_pos {
            self.x_vel + 1
        } else if self.x_pos > other.x_pos {
            self.x_vel - 1
        } else {
            self.x_vel
        };

        self.y_vel = if self.y_pos < other.y_pos {
            self.y_vel + 1
        } else if self.y_pos > other.y_pos {
            self.y_vel - 1
        } else {
            self.y_vel
        };

        self.z_vel = if self.z_pos < other.z_pos {
            self.z_vel + 1
        } else if self.z_pos > other.z_pos {
            self.z_vel - 1
        } else {
            self.z_vel
        };
    }

    fn apply_velocity(&mut self) {
        self.x_pos += self.x_vel;
        self.y_pos += self.y_vel;
        self.z_pos += self.z_vel;
    }
}

#[derive(Debug, PartialEq, Clone)]
struct State {
    bodies: Vec<Body>
}

impl From<String> for State {
    fn from(s: String) -> Self {
        State {
            bodies: s.lines().map(|l| Body::from(l)).collect()
        }
    }
}

impl State {
    fn step(&mut self) -> i64 {
        // Apply gravities
        for index in 1..self.bodies.len() {
            let (left, right) = self.bodies.split_at_mut(index);
            let body_1 = left.last_mut().unwrap();

            for body_2 in right {
                body_1.apply_gravity(body_2);
                body_2.apply_gravity(body_1);
            }
        }

        // Apply velocities
        for body in &mut self.bodies { body.apply_velocity() };

        // Return the total energy
        self.total_energy()
    }

    fn total_energy(&self) -> i64 {
        let mut total_energy = 0;

        for body in &self.bodies {
            total_energy += body.energy();
        }

        total_energy
    }
}

fn run_until_lcms(state: &mut State) -> (usize, usize, usize) {
    let mut iterations = 0;

    let mut x_lcm: Option<usize> = None;
    let mut y_lcm: Option<usize> = None;
    let mut z_lcm: Option<usize> = None;
    
    loop {
        // Print our debug
        if iterations % 10_000 == 0 { println!("Iterations run: {}", iterations); }
        
        // Proceed
        state.step();
        iterations += 1;

        // Check if each dimension is halted
        if x_lcm == None && state.bodies.iter().all(|b| b.x_vel == 0) { x_lcm = Some(iterations); }
        if y_lcm == None && state.bodies.iter().all(|b| b.y_vel == 0) { y_lcm = Some(iterations); }
        if z_lcm == None && state.bodies.iter().all(|b| b.z_vel == 0) { z_lcm = Some(iterations); }

        // If it is the same as any others, break
        match (x_lcm, y_lcm, z_lcm) {
            (Some(x), Some(y), Some(z)) => return (x, y, z),
            _ => continue
        }
    }
}

fn main() {
    let mut state = State::from(std::fs::read_to_string("input.txt").unwrap());
    let (x, y ,z) = run_until_lcms(&mut state);
    println!("Done - {} {} {}", x, y, z);
}

#[test]
fn test() {
    let mut state = State::from(std::fs::read_to_string("test1.txt").unwrap());
    let (x, y, z) = run_until_lcms(&mut state);

    assert_eq!(x, 9);
    assert_eq!(y, 14);
    assert_eq!(z, 22);
}

#[test]
fn test_2() {
    let mut state = State::from(std::fs::read_to_string("test2.txt").unwrap());
    let (x, y, z) = run_until_lcms(&mut state);

    println!("Done - {} {} {}", x, y, z);
}
