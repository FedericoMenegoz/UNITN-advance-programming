#[derive(Debug)]
enum StringInteger {
    String(String),
    Integer(i32),
}
pub fn es9() {
    let integer = 33;
    let string = "trentatre".to_string();
    let vector = vec![
        StringInteger::String(string),
        StringInteger::Integer(integer),
    ];

    println!("The vector with an i32 and a String is {:?}", vector);
}
