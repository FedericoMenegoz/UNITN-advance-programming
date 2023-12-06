use std::slice::Iter;

pub fn es6() {
    let array = [1, 2, 3, 4, 5];
    let iter = array.iter();
    println!("From iter we get: {:?}", build_vector(iter));
}

pub fn build_vector(iter: Iter<i32>) -> Vec<&i32> {
    iter.collect()
}
