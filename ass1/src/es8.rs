pub fn append(s: String) -> String {
    s + "foobar"
}

pub fn es8() {
    let original_string = String::from("Hello, world!");
    let appended_string = append(original_string.clone());

    println!("Original String: {}", original_string);
    println!("Appended String: {}", appended_string);
}
