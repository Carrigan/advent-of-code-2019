
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
        
        for i in 0..5 {            
            if self.code[i + 1] < self.code[i] {
                return false;
            }

            if self.code[i + 1] == self.code[i] {
                same_flag = true;
            }
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
    assert_eq!(Password::from_int(111_112).is_valid(), true);
    assert_eq!(Password::from_int(223_450).is_valid(), false);
    assert_eq!(Password::from_int(123_789).is_valid(), false);
}
