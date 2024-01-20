struct X {
    s: Option<String>,
    _i: i32,
}

impl X {
    fn new(s: &str, _i: i32) -> Self {
        X {
            s: Some(s.to_owned()),
            _i,
        }
    }

    fn take_str(&mut self) -> Option<String> {
        self.s.take()
    }
}
pub fn es2() {
    let mut x = X::new("ciao", 3);

    println!(
        "The first time using take_str produces {:?} and the second time it produces {:?}",
        x.take_str(),
        x.take_str()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_take_str() {
        let mut x = X::new("ciao", 3);
        assert_eq!(x.s, Some("ciao".to_owned()));
        assert_eq!(x._i, 3);

        assert_eq!(x.take_str(), Some("ciao".to_owned()));
        assert_eq!(x.take_str(), None);
    }
}
