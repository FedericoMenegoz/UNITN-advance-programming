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
    println!("\n********** *********** **********");
    println!("           EXERCIZE 1           ");
    println!("********** *********** **********\n");
    es1::es1();
    println!("\n********** *********** **********");
    println!("           EXERCIZE 2           ");
    println!("********** *********** **********\n");
    es2::es2();
    println!("\n********** *********** **********");
    println!("           EXERCIZE 3           ");
    println!("********** *********** **********\n");
    es3::es3();
    println!("\n********** *********** **********");
    println!("           EXERCIZE 4           ");
    println!("********** *********** **********\n");
    es4::es4();
    println!("\n********** *********** **********");
    println!("           EXERCIZE 5           ");
    println!("********** *********** **********\n");
    es5::es5();
    println!("\n********** *********** **********");
    println!("           EXERCIZE 6           ");
    println!("********** *********** **********\n");
    es6::es6();
    println!("\n********** *********** **********");
    println!("           EXERCIZE 7           ");
    println!("********** *********** **********\n");
    es7::es7();
    println!("\n********** *********** **********");
    println!("           EXERCIZE 8           ");
    println!("********** *********** **********\n");
    es8::es8();
    println!("\n********** *********** **********");
    println!("           EXERCIZE 9           ");
    println!("********** *********** **********\n");
    es9::es9();
    println!("\n********** *********** **********");
    println!("           EXERCIZE 10           ");
    println!("********** *********** **********\n");
    es10::es10();
}

// test_module.rs

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::es1::*;
    use crate::es2::*;
    use crate::es3::*;
    use crate::es4::sub_slice;
    use crate::es5::insert_if_longer;
    use crate::es5::is_sorted;
    use crate::es5::max;
    use crate::es5::swap;
    use crate::es7::pancake_sort;
    use crate::es8::merge;

    #[test]
    fn test_modify_odd() {
        let mut array = vec![1, 2, 3, 4, 5];
        modify_odd(&mut array);
        assert_eq!(array, vec![0, 2, 0, 4, 0]);
    }

    #[test]
    fn test_count_character_empty_string() {
        let result = count_character(String::new());
        let expected: HashMap<char, u32> = HashMap::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_character_single_char() {
        let result = count_character(String::from("a"));
        let mut expected = HashMap::new();
        expected.insert('a', 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_character_multiple_chars() {
        let result = count_character(String::from("hello"));
        let mut expected = HashMap::new();
        expected.insert('h', 1);
        expected.insert('e', 1);
        expected.insert('l', 2);
        expected.insert('o', 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_character_mixed_chars() {
        let result = count_character(String::from("aabbcc"));
        let mut expected = HashMap::new();
        expected.insert('a', 2);
        expected.insert('b', 2);
        expected.insert('c', 2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_at_value() {
        let slice = &[1, 2, 3, 4, 5];
        assert_eq!(
            split_at_value(slice, 3),
            Some(([1, 2].as_slice(), [3, 4, 5].as_slice()))
        );
    }

    #[test]
    fn test_sub_slice() {
        let vec_1 = vec![1, 2, 3, 4, 5];
        let vec_2 = vec![3, 4];

        assert_eq!(sub_slice(&vec_1, &vec_2), Ok([3, 4].as_slice()));

        let vec_2 = vec![99];
        assert_eq!(sub_slice(&vec_1, &vec_2), Err("Not found".to_string()));
    }

    #[test]
    fn test_max() {
        assert_eq!(max(&vec![1, 2, 3, 4, 5]), Some(5));
        assert_eq!(max(&vec![]), None);
    }

    #[test]
    fn test_swap() {
        let mut vec = vec![1, 2, 3, 4, 5];
        swap(&mut vec);
        assert_eq!(vec, vec![5, 2, 3, 4, 1]);
    }

    #[test]
    fn test_is_sorted() {
        assert_eq!(is_sorted(&vec![1, 2, 3, 4, 5]), true);
        assert_eq!(is_sorted(&vec![]), true);
        assert_eq!(is_sorted(&vec![2, 4, 1, -90]), false);
    }

    #[test]
    fn test_insert_if_longer() {
        let mut parole = vec!["Ciao".to_string(), "sono".to_string()];
        insert_if_longer(&mut parole, "Fede".to_owned());
        assert_eq!(parole, vec!["Ciao".to_string(), "sono".to_string()]);
        insert_if_longer(&mut parole, "Federico Menegoz".to_owned());
        assert_eq!(
            parole,
            vec![
                "Ciao".to_string(),
                "sono".to_string(),
                "Federico Menegoz".to_string()
            ]
        );
    }

    #[test]
    fn test_pancake_sort() {
        let mut v = [23, 4, -1, 65, 30];
        pancake_sort(&mut v);

        assert_eq!(v, [65, 30, 23, 4, -1]);
    }

    #[test]
    fn test_merge() {
        let slice_1 = [3, 67, 990];
        let slice_2 = [-1, 1, 90];
        let result = [-1, 1, 3, 67, 90, 990];

        assert_eq!(merge(&slice_1, &slice_2), result);
        assert_eq!(merge(&result, &[]), result);
        assert_eq!(merge(&[], &[]), &[]);
    }
}
