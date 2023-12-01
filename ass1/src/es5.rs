pub fn max_min(v: Vec<i32>) -> (Option<i32>, Option<i32>) {
    (v.iter().max().copied(), v.iter().min().copied())
}

pub fn es5() {
    let vector = vec![1, 2, 3, 4, 5, 6];
    println!("The max and min of [1,2,3,4,5,6] are {:?}", max_min(vector))
}
