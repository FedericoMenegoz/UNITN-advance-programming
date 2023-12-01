use std::collections::HashMap;

pub fn es7() {
    let mut furniture: HashMap<String, f32> = HashMap::new();
    furniture.insert(String::from("chair"), 50.0);
    furniture.insert(String::from("table"), 100.0);
    furniture.insert(String::from("bed"), 200.0);

    let furniture_name = String::from("chair");
    let price = get_furniture_price(&furniture, furniture_name);
    println!("The price of chair is {}", price);
}

pub fn get_furniture_price(furniture: &HashMap<String, f32>, furniture_name: String) -> f32 {
    match furniture.get(&furniture_name) {
        Some(&price) => price,
        None => -1.0,
    }
}
