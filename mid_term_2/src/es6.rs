use std::fmt::Debug;

trait Unknown {
    fn serialize(&self) -> String;
}

impl Unknown for i32 {
    fn serialize(&self) -> String {
        format!("{self}")
    }
}

impl Unknown for String {
    fn serialize(&self) -> String {
        format!("{self}")
    }
}

impl Unknown for &str {
    fn serialize(&self) -> String {
        format!("{self}")
    }
}

impl<T: Unknown + Debug> Unknown for Vec<T> {
    fn serialize(&self) -> String {
        format!("{:?}", self)
    }
}

fn get_vec() -> Vec<Box<dyn Unknown>> {
    vec![]
}

fn print_vec(v: &Vec<Box<dyn Unknown>>) {
    v.iter().for_each(|val| println!("{}", val.serialize()));
}
pub fn es6() {
    println!("Test 1:");
    test_1();
    println!();
    println!("Test 2:");
    test_2();
    println!();
}

fn test_1() {
    let mut v = get_vec();
    v.push(Box::new("hiii!".to_string()));
    v.push(Box::new(-587));
    v.push(Box::new("xyz".to_string()));
    v.push(Box::new(vec![4, 5, 6]));
    print_vec(&v);
}

fn test_2() {
    let mut v = get_vec();
    v.push(Box::new("invalid data".to_string()));
    v.push(Box::new(58));
    v.push(Box::new(987));
    v.push(Box::new(vec![vec!["hi1", "hi2"], vec!["hi3"]]));
    print_vec(&v);
}
