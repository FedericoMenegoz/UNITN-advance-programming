use std::collections::HashMap;

pub fn es2() {
    println!(
        "The letter count for \"Sopra la panca la capra campa, sotto la panca la capra crepa!\" are: {:#?}",
        count_character("Sopra la panca la capra campa, sotto la panca la capra crepa!".to_string())
    );
}

pub fn count_character(string: String) -> HashMap<char, u32> {
    let mut return_map = HashMap::new();
    string.chars().for_each(|c| {
        let c = c.to_ascii_lowercase();
        *return_map.entry(c).or_insert(0) += 1;
    });
    return_map
}
