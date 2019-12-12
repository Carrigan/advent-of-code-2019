#[derive(Debug)]
struct Point {
    color: Color,
    x: i64,
    y: i64
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Up, 
    Down,
    Left,
    Right
}

impl Direction {
    fn turn(&self, other: Direction) -> Direction {
        match self {
            Direction::Up => if other == Direction::Left { Direction::Left } else { Direction::Right },
            Direction::Down => if other == Direction::Left { Direction::Right } else { Direction::Left },
            Direction::Left => if other == Direction::Left { Direction::Down } else { Direction::Up },
            Direction::Right => if other == Direction::Left { Direction::Up } else { Direction::Down }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Color {
    White,
    Black
}

struct Space {
    visited: Vec<Point>
}

impl Space {
    fn new() -> Self {
        Space { visited: vec!(Point { x: 0, y: 0, color: Color::White }) }
    }

    fn update(&mut self, x: i64, y: i64, color: Color) {
        for point in &mut self.visited {
            if point.x == x && point.y == y {
                point.color = color;
                return;
            }
        }        

        self.visited.push(Point { x, y, color });
    }

    fn has_visited(&self, x: i64, y: i64) -> bool {
        for point in &self.visited {
            if point.x == x && point.y == y {
                return true;
            }
        }
        
        false
    }

    fn color_of(&self, x: i64, y: i64) -> Color {
        for point in &self.visited {
            if point.x == x && point.y == y {
                return point.color;
            }
        }        

        Color::Black
    }
}

struct Robot {
    brain: intcode::Program,
    heading: Direction,
    x: i64,
    y: i64,
}

impl Robot {
    fn new(code: String) -> Self {
        Robot { brain: intcode::Program::from(code), x: 0, y: 0, heading: Direction::Up }
    }

    fn iterate(&mut self, current_color: Color) -> Option<(Color, Direction)> {
        // Run the program
        let input: i64 = if current_color == Color::White { 1 } else { 0 };
        self.brain.append_inputs(&mut vec!(input));

        let color = match self.brain.run_until_event() {
            intcode::ProgramResult::Output(c) => if c == 1 { Color::White } else { Color::Black },
            intcode::ProgramResult::Complete => return None
        };

        let direction = match self.brain.run_until_event() {
            intcode::ProgramResult::Output(c) => if c == 1 { Direction::Right } else { Direction::Left },
            intcode::ProgramResult::Complete => return None
        };

        // Update our positional state
        self.heading = self.heading.turn(direction);
        
        self.x = match self.heading {
            Direction::Left => self.x - 1,
            Direction::Right => self.x + 1,
            _ => self.x
        };

        self.y = match self.heading {
            Direction::Up => self.y + 1,
            Direction::Down => self.y - 1,
            _ => self.y
        };

        println!("Program output: {:?}, {:?}", color, direction);

        Some((color, direction))
    }
}

struct RobotProgram<'a> {
    robot: &'a mut Robot,
    space: &'a mut Space
}

impl <'a> Iterator for RobotProgram<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let current_x = self.robot.x;
        let current_y = self.robot.y;
        let current_color = self.space.color_of(current_x, current_y);

        // This will return the next color
        if let Some((color, _)) = self.robot.iterate(current_color) {
            self.space.update(current_x, current_y, color);
            return Some(Point { x: current_x, y: current_y, color: color });
        }

        None
    }
}


fn main() {
    let code = std::fs::read_to_string("input.txt").unwrap();
    let mut robot = Robot::new(code);
    let mut space = Space::new();
    let program = RobotProgram { robot: &mut robot, space: &mut space };

    for visitation in program {
        println!("Visited: {:?} {:?} -> {:?}", visitation.x, visitation.y, visitation.color);
    }

    for y in 0..6 {
        for x in 0..41 {
            print!("{}", if space.color_of(x, 0 - y) == Color::White { '#' } else { ' ' });
        }

        println!("");
    }
}
