
struct Password {
    code: [u32; 6]
}

impl Password { 
    fn from_int(n: u32) -> Password {
        Password {
            code: [
                get_digit(n, 5),
                get_digit(n, 4),
                get_digit(n, 3),
                get_digit(n, 2),
                get_digit(n, 1),
                get_digit(n, 0)
            ]
        }
    }
    
    fn is_valid(&self) -> bool {
        let mut same_flag = false;
        let mut last: Option<u32> = None;
        
        for i in 0..5 {      
            let current = self.code[i];
            let next = self.code[i + 1];
                  
            if next < current {
                return false;
            }

            if next == current {
                same_flag = same_flag | match i {
                    0 => self.code[i + 2] != current,
                    4 => last.unwrap() != current,
                    _ => self.code[i + 2] != current && last.unwrap() != current
                };
            }
            
            last = Some(current);
        }
        
        return same_flag;
    }
}

fn get_digit(n: u32, i: usize) -> u32 {
    let mut x = n;
    for _ in 0..i { x /= 10; }
    x % 10
}

fn main() {
    let mut total_count = 0;
    
    for x in 138241..674034 {
        if Password::from_int(x).is_valid() { total_count += 1 }
    }
    
    println!("{:?}", total_count);
}

#[test]
fn test_all() {
    assert_eq!(Password::from_int(112233).is_valid(), true);
    assert_eq!(Password::from_int(123444).is_valid(), false);
    assert_eq!(Password::from_int(111122).is_valid(), true);
}
