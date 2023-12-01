pub fn is_amstrong(number: u32) -> bool {
    let s = number.to_string();
    let len = s.len();
    s.chars()
        .map(|c| c.to_digit(10).expect("It should be a valid digit."))
        .fold(0_u32, |acc: u32, x: u32| acc + x.pow(len as u32))
        == number
}

pub fn es9() {
    println!("9 is an amstrong number? {}", is_amstrong(9));
    println!("10 is an amstrong number? {}", is_amstrong(10));
    println!("153 is an amstrong number? {}", is_amstrong(153));
    println!("154 is an amstrong number? {}", is_amstrong(154));
}
