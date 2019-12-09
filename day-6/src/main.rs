use std::fs;

#[derive(Debug)]
struct Orbit {
    parent: String,
    child: String
}

impl From<String> for Orbit {
    fn from(input: String) -> Self {
        let planets: Vec<&str> = input.split(")").collect();
        Orbit { parent: String::from(planets[0]), child: String::from(planets[1]) }
    }
}

#[derive(Debug)]
struct Galaxy {
    orbits: Vec<Orbit>
}

impl Galaxy {
    fn new(input: String) -> Self {
        Galaxy { orbits: input.lines().map(|l| Orbit::from(String::from(l))).collect() }
    }
    
    fn total_orbits(&self, start: String) -> usize {
        let mut total_distance = 1;
        let mut current_orbits: Vec<Orbit> = self.orbits_by_parent(start);
        let mut distance = 1;
        
        while current_orbits.len() > 0 {
            println!("{:?}", current_orbits);
            let mut next_orbits: Vec<Orbit> = Vec::new();
            distance += 1;
            
            for orbit in current_orbits {
                let mut child_orbits = self.orbits_by_parent(orbit.child);  
                total_distance += child_orbits.len() * distance;
                next_orbits.append(&mut child_orbits);
            }
            
            current_orbits = next_orbits;
        } 
       
       total_distance
    }

    fn traverse(&self, from_planet: String, to_planet: String) -> usize {
        let mut visited: Vec<String> = Vec::new();
        self.traverse_with_list(from_planet, to_planet, 1, &mut visited).unwrap()
    }

    fn traverse_with_list(&self, from_planet: String, to_planet: String, distance: usize, visited: &mut Vec<String>) -> Option<usize> {
        visited.push(from_planet.clone());

        for connection in self.connections(from_planet) {
            if connection == to_planet {
                return Some(distance - 2);
            }

            if visited.contains(&connection) {
                continue;
            }

            if let Some(d) = self.traverse_with_list(connection, to_planet.clone(), distance + 1, visited) {
                return Some(d);
            }
        }

        None
    }
    
    fn orbits_by_parent(&self, root: String) -> Vec<Orbit> {
        (&self.orbits)
            .into_iter()
            .filter(|o| o.parent == root)
            .map(|o| Orbit { parent: o.parent.clone(), child: o.child.clone() })
            .collect()
    }

    fn connections(&self, root: String) -> Vec<String> {
        let mut connections: Vec<String> = Vec::new();

        for orbit in &self.orbits {
            if orbit.child == root {
                connections.push(orbit.parent.clone());
            }

            if orbit.parent == root {
                connections.push(orbit.child.clone());
            }
        }

        connections
    }
}

fn main() {
    let galaxy = Galaxy::new(fs::read_to_string("input.txt").unwrap());
    println!("Distance between YOU and SAN: {:?}", galaxy.traverse(String::from("YOU"), String::from("SAN")));
}

#[test]
fn test_case() {
    let input = fs::read_to_string("test.txt").unwrap();
    let galaxy = Galaxy::new(input);    
    assert_eq!(galaxy.total_orbits(String::from("COM")), 42);
}

#[test]
fn test_traverse() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let galaxy = Galaxy::new(input);
    assert_eq!(galaxy.traverse(String::from("YOU"), String::from("SAN")), 4);
}
