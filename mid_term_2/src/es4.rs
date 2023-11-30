mod odd_module {
    pub const CONSTANT: i32 = 123;
}

mod even_module {
    pub const CONSTANT: i32 = 246;
}

mod getter_function {
    use super::{even_module, odd_module};

    pub fn get_constant(value: u32) -> i32 {
        match value % 2 == 0 {
            false => odd_module::CONSTANT,
            true => even_module::CONSTANT,
        }
    }
}
pub fn es4() {
    println!("Test 1:");
    test_1();
    println!();
    println!("Test 2:");
    test_2();
    println!();
}
fn test_1() {
    println!("odd constant: {}", odd_module::CONSTANT);
    println!("even constant: {}", even_module::CONSTANT);
    print!("test function: {}", getter_function::get_constant(100));
}
fn test_2() {
    println!("odd constant: {}", odd_module::CONSTANT);
    println!("even constant: {}", even_module::CONSTANT);
    println!("test function: {}", getter_function::get_constant(105));
}
