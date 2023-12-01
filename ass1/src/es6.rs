pub fn lord_farquaad(s: String) -> String {
    s.chars()
        .map(|c| match c {
            'e' => '💥',
            a => a,
        })
        .collect()
}

pub fn es6() {
    println!(
        "This text will become more explosive => {}",
        lord_farquaad("This text will become more explosive".to_string())
    )
}
