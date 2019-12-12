use std::fmt;
use std::fs;

#[derive(PartialEq, Copy, Clone)]
enum ObjectType {
    Empty,
    Asteroid,
}

impl fmt::Debug for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if *self == ObjectType::Empty { '.' } else { '#' })
    }
}

impl From<char> for ObjectType {
    fn from(c: char) -> Self {
        match c {
            '.' => ObjectType::Empty,
            _ => ObjectType::Asteroid,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Object {
    point_type: ObjectType,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Space {
    objects: Vec<Object>,
}

impl From<String> for Space {
    fn from(s: String) -> Self {
        let mut objects: Vec<Object> = Vec::new();

        Space {
            objects: s
                .lines()
                .enumerate()
                .map(|(row_i, row)| {
                    let objects: Vec<Object> = row.chars()
                        .enumerate()
                        .map(|(char_i, lc)| Object {
                            x: char_i,
                            y: row_i,
                            point_type: ObjectType::from(lc),
                        })
                        .collect();
                    
                    objects
                })
                .flatten()
                .collect()
        }
    }
}

impl Space {
    fn find_best_station(&self) -> BaseStation {
        let mut best_station: Option<BaseStation> = None;

        for object in &self.objects {
            // We can only build on asteroids
            if object.point_type == ObjectType::Empty {
                continue;
            }

            let mut this_station = BaseStation {
                x: object.x,
                y: object.y,
                count: 0
            };

            for comparison in &self.objects {
                // We are only looking for asteroids that are not this asteroid
                if comparison.point_type == ObjectType::Empty || object == comparison { 
                    continue; 
                }

                if self.can_see(object, comparison) {
                    println!("- {:?} can see {:?}", object, comparison);
                    this_station.count += 1;
                } else {
                    println!("- {:?} cannot see {:?}", object, comparison);
                }
            }

            println!("First station established: {:?}", this_station);
            
            best_station = match best_station {
                None => { 
                    Some(this_station)
                },
                Some(station) => 
                    if this_station.count > station.count { 
                        Some(this_station) 
                    } else { 
                        Some(station) 
                    }
            }
        }

        best_station.unwrap()
    }

    fn can_see(&self, obj: &Object, other: &Object) -> bool {
        for point in SpaceIterator::new(obj, other) {
            let object_type = self.object_at(point.x, point.y);

            if obj.x == point.x && obj.y == point.y {
                continue;
            }

            if other.x == point.x && other.y == point.y {
                break;
            }
            
            if object_type == ObjectType::Asteroid {
                return false;
            }
        }

        true
    }

    fn object_at(&self, x: usize, y: usize) -> ObjectType {
        for object in &self.objects {
            if object.x == x && object.y == y {
                return object.point_type;
            }
        }

        ObjectType::Empty
    }
}

// http://playtechs.blogspot.com/2007/03/raytracing-on-grid.html
struct SpaceIterator {
    dx: i32,
    dy: i32,
    x: i32,
    y: i32,
    n: i32,
    x_inc: i32,
    y_inc: i32,
    error: i32
}

impl SpaceIterator {
    fn new(obj: &Object, other: &Object) -> Self {
        let dx = if obj.x > other.x { obj.x - other.x } else { other.x - obj.x } as i32;
        let dy = if obj.y > other.y { obj.y - other.y } else { other.y - obj.y } as i32;
        
        SpaceIterator {
            dx: dx * 2, 
            dy: dy * 2,
            x: obj.x as i32,
            y: obj.y as i32,
            n: 1 + dx + dy,
            x_inc: if other.x > obj.x { 1 } else { -1 },
            y_inc: if other.y > obj.y { 1 } else { -1 },
            error: dx - dy
        }
    }
}

impl Iterator for SpaceIterator {
    type Item = SpaceResult;

    fn next(&mut self) -> Option<Self::Item> {
        self.n -= 1;
        if self.n == 0 {
            return None;
        }

        let result = SpaceResult { x: self.x as usize, y: self.y as usize };

        if self.error > 0 {
            self.x += self.x_inc;
            self.error -= self.dy;
        } else {
            self.y += self.y_inc;
            self.error += self.dx;
        }

        Some(result)
    }
}

struct SpaceResult {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct BaseStation {
    x: usize,
    y: usize,
    count: usize
}

fn main() {
    let space_diagram = fs::read_to_string("test1.txt").unwrap();
    let space = Space::from(space_diagram);
    println!("{:?}", space);
}

#[test]
fn simple_test() {
    let space = Space::from(String::from("#.#\n##."));
    
    let obj1 = &space.objects[0];
    let obj2 = &space.objects[2];
    let obj3 = &space.objects[3];

    assert_eq!(space.can_see(obj3, obj1), true);
    assert_eq!(space.can_see(obj2, obj1), true);
    assert_eq!(space.can_see(obj3, obj2), false);
}

#[test]
fn test_1() {
    let space_diagram = fs::read_to_string("test1.txt").unwrap();
    let space = Space::from(space_diagram);
    let station = space.find_best_station();
    assert_eq!(station.x, 3);
    assert_eq!(station.y, 4);
    assert_eq!(station.count, 8);
}

#[test]
fn test_2() {
    let space_diagram = fs::read_to_string("test2.txt").unwrap();
    let space = Space::from(space_diagram);
    let station = space.find_best_station();
    assert_eq!(station.x, 5);
    assert_eq!(station.y, 8);
    assert_eq!(station.count, 33);
}
