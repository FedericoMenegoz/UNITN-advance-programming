use std::fmt::{Debug, Display};

pub fn restricted<T: PartialOrd + Debug, U: Display>(t1: T, t2: T, u: U) -> T {
    let ret_val = match t1 > t2 {
        false => t1,
        true => t2,
    };

    print!("{0:<8}", "minor:");
    println!("{:?}", ret_val);

    print!("{0:<8}", "u:");
    println!("{}", u);

    ret_val
}

pub fn es3() {
    let (n1, n2, d1) = (5, 6, "displayable");

    restricted(n1, n2, d1);
}
