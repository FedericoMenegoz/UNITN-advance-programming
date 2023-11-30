use std::ops::Add;

trait CloneAndDouble {
    fn clone_and_double(&self) -> Self;
}

impl<T: Clone + Add<Output = T>> CloneAndDouble for T {
    fn clone_and_double(&self) -> Self {
        self.clone() + self.clone()
    }
}

pub fn es5() {
    println!("Test 1:");
    test_1();
    println!();
    println!("Test 2:");
    test_2();
    println!();
}
fn test_1() {
    println!("{}", 1u8.clone_and_double());
    println!("{}", 1i8.clone_and_double());
    println!("{}", 2u16.clone_and_double());
    println!("{}", 2i16.clone_and_double());
    println!("{}", 3u32.clone_and_double());
    println!("{}", 3i32.clone_and_double());
    println!("{}", 4u64.clone_and_double());
    println!("{}", 4i64.clone_and_double());
}
fn test_2() {
    println!("{}", 2u8.clone_and_double());
    println!("{}", 2i8.clone_and_double());
    println!("{}", 3u16.clone_and_double());
    println!("{}", 3i16.clone_and_double());
    println!("{}", 4u32.clone_and_double());
    println!("{}", 4i32.clone_and_double());
    println!("{}", 5u64.clone_and_double());
    println!("{}", 5i64.clone_and_double());
}
