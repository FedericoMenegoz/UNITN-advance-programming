pub fn bigger(n1: i32, n2: i32) -> i32 {
    match n1 > n2 {
        true => n1,
        false => n2,
    }
}

pub fn es2() {
    println!("From 5 and 65 the bigger is {}", bigger(5, 65));
}
