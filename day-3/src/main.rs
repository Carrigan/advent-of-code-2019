use std::str::FromStr;
use std::fs;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn is_parallel_to(&self, other: &Direction) -> bool {
        match other {
            Direction::Up | Direction::Down => 
                *self == Direction::Up || *self == Direction::Down,
            Direction::Left | Direction::Right => 
                *self == Direction::Left || *self == Direction::Right,
        }
    }
}

struct Wire {
    travels: Vec<Travel>
}

#[derive(Debug)]
struct WireError;

impl FromStr for Wire {
    type Err = WireError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { travels: s.split(",").map(|s| Travel::from_str(s).unwrap()).collect() })
    }
}

#[derive(Clone, Debug)]
struct Travel {
    direction: Direction,
    distance: i32
}

#[derive(Debug, PartialEq)]
enum TravelError {
    ParseError,
    EnumError
}

impl FromStr for Travel {
    type Err = TravelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s.bytes().nth(0).unwrap() as char {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => return Err(TravelError::EnumError)
        };

        let distance = match (s[1..]).parse::<i32>() {
            Ok(n) => n,
            Err(_) => return Err(TravelError::ParseError)
        };

        Ok(Travel { direction, distance })
    }
}

#[derive(Debug)]
struct Intersection {
    point: Point,
    distance_1: Option<u32>,
    distance_2: Option<u32>
}

impl Intersection {
    fn total_distance(&self) -> u32 {
        self.distance_1.unwrap() + self.distance_2.unwrap()
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn travel(&self, travel: &Travel) -> Point {
        let x = match &travel.direction {
            Direction::Up => self.x,
            Direction::Down => self.x,
            Direction::Left => self.x - travel.distance,
            Direction::Right => self.x + travel.distance
        };

        let y = match &travel.direction {
            Direction::Up => self.y + travel.distance,
            Direction::Down => self.y - travel.distance,
            Direction::Left => self.y,
            Direction::Right => self.y
        };

        Point { x, y }
    }
    
    fn mv(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Up => Point { x: self.x, y: self.y + 1 },
            Direction::Down => Point { x: self.x, y: self.y - 1 },
            Direction::Left => Point { x: self.x - 1, y: self.y },
            Direction::Right => Point { x: self.x + 1, y: self.y },
        }
    }
}

struct Line {
    start: Point,
    travel: Travel
}

impl Line {
    fn contains(&self, point: &Point) -> bool {
        let mut current_point = self.start.clone();
        
        // Optimization: If the start point does not share an X or a Y, they cannot cross
        if self.start.x != point.x && self.start.y != point.y {
            return false;
        }
        
        for _ in 0..self.travel.distance {
            if &current_point == point {
                return true;
            }
            
            current_point = current_point.mv(&self.travel.direction);
        }
        
        return false;
    }
}

impl Wire {
   fn closest_intersection_with(&self, wire: &Wire) -> Option<u32> {
        let mut intersections: Vec<Intersection> = Vec::new();

        // Convert wire 1 to lines
        let mut last_point = Point { x: 0, y: 0};
        let mut lines: Vec<Line> = Vec::new();
        for travel in &self.travels {
            let next_point = last_point.travel(travel);

            lines.push(Line { start: last_point, travel: travel.clone() });
            last_point = next_point;
        }

        // Follow wire 2 as it is constructed and mark all intersects
        let mut current_location = Point { x: 0, y: 0 };
        let mut total_distance = 1;
        
        for travel in &wire.travels {
            println!("Checking for intersects along travel {:?}", travel);
            
            for _ in 0..travel.distance {
                current_location = current_location.mv(&travel.direction);
                
                // Check if any lines contain that point
                for line in &lines {                    
                    if line.contains(&current_location) {
                        println!("Intersect found! {:?} {:?}", current_location, total_distance);
                        intersections.push(Intersection { 
                            point: current_location.clone(),
                            distance_1: Some(total_distance),
                            distance_2: None
                        });
                    }
                }
                
               total_distance += 1;
            }
        }
        
        // Reiterate through the first wire and mark the distances at the intersects
        current_location = Point { x: 0, y: 0 };
        total_distance = 1;
        
        for travel in &self.travels {
            for _ in 0..travel.distance {
                current_location = current_location.mv(&travel.direction);
                
                for intersect in &mut intersections {
                    if intersect.point == current_location {
                        if intersect.distance_2 == None {
                            intersect.distance_2 = Some(total_distance);
                        }
                    }
                }
                
                total_distance += 1;
            }
        }
        
        println!("{:?}", intersections);
        
        
        // Go through all points and find the closest one
        let mut closest_intersect: Option<u32> = None;

        for intersect in intersections {
            let intersect_distance = intersect.total_distance();
            
            closest_intersect = match closest_intersect {
                Some(distance) => 
                    if distance < intersect_distance { 
                        Some(distance)
                    } else { 
                        Some(intersect_distance) 
                    },
                None => Some(intersect_distance)
            }
        }

        closest_intersect
    } 
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let wires: Vec<Wire> = input.lines().map(|l| Wire::from_str(l).unwrap()).collect();
    let intersect = wires.get(0).unwrap().closest_intersection_with(wires.get(1).unwrap()).unwrap();
    
    println!("{:?}", intersect);
}


#[test]
fn test_case_1() {
    let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    let wires: Vec<Wire> = input.lines().map(|l| Wire::from_str(l).unwrap()).collect();
    let intersect = wires.get(0).unwrap().closest_intersection_with(wires.get(1).unwrap()).unwrap();

    assert_eq!(intersect, 610);
}

#[test]
fn test_case_2() {
    let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    let wires: Vec<Wire> = input.lines().map(|l| Wire::from_str(l).unwrap()).collect();
    let intersect = wires.get(0).unwrap().closest_intersection_with(wires.get(1).unwrap()).unwrap();

    assert_eq!(intersect, 410);
}

#[test]
fn test_case_3() {
    let input = "U7,R6,D4,L4\nR8,U5,L5,D3";
    let wires: Vec<Wire> = input.lines().map(|l| Wire::from_str(l).unwrap()).collect();
    let intersect = wires.get(0).unwrap().closest_intersection_with(wires.get(1).unwrap()).unwrap();

    assert_eq!(intersect, 30);
}