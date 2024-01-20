fn res1(n: i32) -> Result<i32, String> {
    match n % 10 {
        0 => Ok(n),
        _ => Err("Error".to_owned()),
    }
}

fn res2(res: Result<i32, String>) -> Result<i32, String> {
    if let Ok(n) = res {
        match n % 5 {
            0 => Ok(n),
            _ => Err("Error".to_owned()),
        }
    } else {
        Err("Error".to_owned())
    }
}

fn wrapper(n: i32) -> Result<i32, String> {
    res2(res1(n))
}

pub fn es9() {
    println!("Is 16 divisible by 10 and 5?{:?}", wrapper(16));
    println!("Is 20 divisible by 10 and 5?{:?}", wrapper(20));
    println!("Is 15 divisible by 10 and 5?{:?}", wrapper(15));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_res1() {
        assert_eq!(res1(10), Ok(10));
        assert_eq!(res1(15), Err("Error".to_owned()));
    }

    #[test]
    fn test_res2() {
        assert_eq!(res2(Ok(10)), Ok(10));
        assert_eq!(res2(Ok(16)), Err("Error".to_owned()));
        assert_eq!(res2(Err("Some error".to_owned())), Err("Error".to_owned()));
    }

    #[test]
    fn test_wrapper() {
        assert_eq!(wrapper(10), Ok(10));
        assert_eq!(wrapper(15), Err("Error".to_owned()));
    }
}
