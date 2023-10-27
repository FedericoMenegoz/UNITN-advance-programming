use std::ops::{Add, Sub, Mul};
#[derive(Debug)]
struct Pair (i32, String);

impl Add<i32> for Pair {
    type Output = Pair;
    fn add(self, rhs: i32) -> Self::Output {
        Pair (self.0 + rhs, self.1)
    }
}

impl Sub<i32> for Pair {
    type Output = Pair;
    fn sub(self, rhs: i32) -> Self::Output {
        Pair(self.0 - rhs, self.1)
    }
}

impl Add<&str> for Pair {
    type Output = Pair;
    fn add(self, rhs: &str) -> Self::Output {
        Pair (self.0, self.1 + rhs)
    }
}

impl Sub<&str> for Pair {
    type Output = Pair;
    fn sub(self, rhs: &str) -> Self::Output {
        Pair(self.0, self.1.replace(rhs, ""))
    }
}

impl Add for Pair {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.0 + rhs.1.as_str()
    }
}

impl Sub for Pair {
    type Output = Pair;

    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.0 - rhs.1.as_str()
    }
}

impl Mul<i32> for Pair {
    type Output = Pair;

    fn mul(self, rhs: i32) -> Self::Output {
        let new_str = (0..rhs)
        .into_iter()
        .fold("".to_string(), |acc, _| acc + self.1.as_str());

        Pair (self.0.pow(rhs as u32), new_str)
    }
} 

pub fn es5() {
    let (p1, p2) = 
    (
        Pair(2, "Hello Pair".to_string()), 
        Pair(6, "Goobye Pair".to_string())
    );
    
    println!("{:?}", p1 + p2);
    
    
    let (p1, p2) = 
    (
        Pair(2, "Hello Pair".to_string()), 
        Pair(6, "Pair".to_string())
    );
    println!("{:?}", p1 - p2);
    
    let p1 = Pair(2, "Hello Pair".to_string());
    println!("{:?}", p1 * 2);
}