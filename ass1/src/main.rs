pub mod es1;
pub mod es10;
pub mod es2;
pub mod es3;
pub mod es4;
pub mod es5;
pub mod es6;
pub mod es7;
pub mod es8;
pub mod es9;

fn main() {
    println!("********** *********** **********");
    println!("********** ESERCIZIO 1 **********");
    println!("********** *********** **********\n");
    es1::es1();
    println!("\n********** *********** **********");
    println!("********** ESERCIZIO 2 **********");
    println!("********** *********** **********\n");
    es2::es2();
    println!("\n********** *********** **********");
    println!("********** ESERCIZIO 3 **********");
    println!("********** *********** **********\n");
    es3::es3();
    println!("\n********** *********** **********");
    println!("********** ESERCIZIO 4 **********");
    println!("********** *********** **********\n");
    es4::es4();
    println!("\n********** *********** **********");
    println!("********** ESERCIZIO 5 **********");
    println!("********** *********** **********\n");
    es5::es5();
    println!("\n********** *********** **********");
    println!("********** ESERCIZIO 6 **********");
    println!("********** *********** **********\n");
    es6::es6();
    println!("\n********** *********** **********");
    println!("********** ESERCIZIO 7 **********");
    println!("********** *********** **********\n");
    es7::es7();
    println!("\n********** *********** **********");
    println!("********** ESERCIZIO 8 **********");
    println!("********** *********** **********\n");
    es8::es8();
    println!("\n********** *********** **********");
    println!("********** ESERCIZIO 9 **********");
    println!("********** *********** **********\n");
    es9::es9();
    println!("\n********** *********** **********");
    println!("********** ESERCIZIO 10 *********");
    println!("********** *********** **********\n");
    es10::es10();
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use physical_constants::SPEED_OF_LIGHT_IN_VACUUM;

    use crate::es1::string_reverse;
    use crate::es10::transpose;
    use crate::es2::bigger;
    use crate::es3::multiply;
    use crate::es4::e_equals_mc_squared;
    use crate::es5::max_min;
    use crate::es6::lord_farquaad;
    use crate::es7::get_furniture_price;
    use crate::es8::append;
    use crate::es9::is_amstrong;

    #[test]
    fn test_es1() {
        assert_eq!(
            "ociredeF onos oi oaiC",
            string_reverse("Ciao io sono Federico")
        );
        assert_eq!("🦀🫀👌🏿", string_reverse("👌🏿🫀🦀"));
        assert_eq!("तेस्मन", string_reverse("नमस्ते"));
    }

    #[test]
    fn test_es2() {
        assert_eq!(bigger(5, 65), 65);
        assert_eq!(bigger(5, 5), 5);
        assert_eq!(bigger(-23, 5), 5);
    }

    #[test]
    fn test_es3() {
        assert_eq!(
            multiply(23, 0.5, std::f64::consts::PI),
            23.0 * 0.5 * std::f64::consts::PI
        )
    }

    #[test]
    fn test_es4() {
        assert_eq!(e_equals_mc_squared(0.0), 0.0);
        assert_eq!(e_equals_mc_squared(1.0), SPEED_OF_LIGHT_IN_VACUUM.powf(2.0));
        assert_eq!(
            e_equals_mc_squared(2.0),
            2.0 * SPEED_OF_LIGHT_IN_VACUUM.powf(2.0)
        );
    }

    #[test]
    fn test_es5() {
        assert_eq!((Some(6), Some(1)), max_min(vec![1, 2, 3, 4, 5, 6]));
        assert_eq!((None, None), max_min(vec![]));
        assert_eq!(
            (Some(590), Some(-234)),
            max_min(vec![3, -234, 590, 10, -100])
        );
    }

    #[test]
    fn test_es6() {
        assert_eq!(
            lord_farquaad("This text will become more explosive".to_string()),
            "This t💥xt will b💥com💥 mor💥 💥xplosiv💥"
        );
    }

    #[test]
    fn test_es7() {
        let mut furniture: HashMap<String, f32> = HashMap::new();
        furniture.insert(String::from("chair"), 50.0);
        furniture.insert(String::from("table"), 100.0);
        furniture.insert(String::from("bed"), 200.0);

        let furniture_name = String::from("chair");
        let price = get_furniture_price(&furniture, furniture_name);
        assert_eq!(price, 50.0);
        let furniture_name = String::from("sofa");
        let price = get_furniture_price(&furniture, furniture_name);
        assert_eq!(price, -1.0);
    }

    #[test]
    fn test_es8() {
        let input = String::from("Hello");
        let expected_output = String::from("Hellofoobar");
        assert_eq!(append(input), expected_output);
    }

    #[test]
    fn test_es9() {
        assert_eq!(is_amstrong(153), true);
        assert_eq!(is_amstrong(370), true);
        assert_eq!(is_amstrong(371), true);
        assert_eq!(is_amstrong(407), true);
        assert_eq!(is_amstrong(1634), true);
        assert_eq!(is_amstrong(8208), true);
        assert_eq!(is_amstrong(9474), true);

        assert_eq!(is_amstrong(123), false);
        assert_eq!(is_amstrong(456), false);
        assert_eq!(is_amstrong(789), false);
        assert_eq!(is_amstrong(999), false);
        assert_eq!(is_amstrong(1000), false);
        assert_eq!(is_amstrong(1234), false);
        assert_eq!(is_amstrong(5678), false);
    }

    #[test]
    fn test_es10() {
        let matrix = ((1, 2), (3, 4));
        let transposed = transpose(matrix);
        assert_eq!(transposed, ((1, 3), (2, 4)));
    }
}
