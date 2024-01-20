pub fn es10() {
    let vec = vec!["How".to_owned(), "Are".to_owned(), "You".to_owned()];
    print!("From {:?},", vec);
    println!(" to {:?}", order(vec));
}

fn order(vec: Vec<String>) -> Vec<String> {
    vec.iter()
        .enumerate()
        .map(|(index, s)| format!("{index} - {s}"))
        .collect()
}

#[cfg(test)]
mod test {
    use super::order;

    #[test]
    fn test_es10() {
        let vec = vec!["How".to_owned(), "Are".to_owned(), "You".to_owned()];
        let vec_ord = order(vec.clone());
        let right_vec = vec![
            "0 - How".to_owned(),
            "1 - Are".to_owned(),
            "2 - You".to_owned(),
        ];

        assert_eq!(vec_ord, right_vec);
    }
}
