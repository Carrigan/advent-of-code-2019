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
    
    fn orbits_by_parent(&self, root: String) -> Vec<Orbit> {
        (&self.orbits)
            .into_iter()
            .filter(|o| o.parent == root)
            .map(|o| Orbit { parent: o.parent.clone(), child: o.child.clone() })
            .collect()
    }
}

fn main() {
    let galaxy = Galaxy::new(fs::read_to_string("input.txt").unwrap());
    println!("Total orbits: {:?}", galaxy.total_orbits(String::from("COM")));
}

#[test]
fn test_case() {
    let input = fs::read_to_string("test.txt").unwrap();
    let galaxy = Galaxy::new(input);    
    assert_eq!(galaxy.total_orbits(String::from("COM")), 42);
}
