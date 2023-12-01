pub fn multiply(i: i32, f1: f32, f2: f64) -> f64 {
    i as f64 * f1 as f64 * f2
}

pub fn es3() {
    println!("23 * 0.5 * π = {}", multiply(23, 0.5, std::f64::consts::PI));
}
