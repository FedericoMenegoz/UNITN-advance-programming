pub fn es1() {
    let s1 = "aaaaAAAA";
    let s2 = "Ciao!";
    let s3 = "😿🤣";

    println!("{:?} => {:?}", s1, prev_str(s1));
    println!("{:?} => {:?}", s2, prev_str(s2));
    println!("{:?} => {:?}", s3, prev_str(s3));
}

fn prev_str(str: &str) -> String {
    str.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                match c {
                    'A' => 'A',
                    'a' => 'a',
                    c => (c as u8 - 1) as char,
                }
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prev_str() {
        assert_eq!(prev_str("aaaaAAAA"), "aaaaAAAA");
        assert_eq!(prev_str("Ciao!"), "Bhan!");
        assert_eq!(prev_str("😿🤣"), "😿🤣");
    }
}
