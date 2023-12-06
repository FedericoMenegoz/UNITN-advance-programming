pub fn es4() {
    let v1 = vec![1, 45, 2, 3, 1, 2, 3, 67];
    let v2 = vec![1, 2, 3];
    println!(
        "Given v1: {:?} and v2: {:?} we get {:?}",
        v1,
        v2,
        sub_slice(&v1, &v2).unwrap()
    );
    let v2 = vec![1, 2, 99];
    println!(
        "Given v1: {:?} and v2: {:?} we get {:?}",
        v1,
        v2,
        sub_slice(&v1, &v2).unwrap_err()
    );
}

pub fn sub_slice<'a>(vec_1: &'a [i32], vec_2: &'a [i32]) -> Result<&'a [i32], String> {
    if let Some(index) = vec_1
        .windows(vec_2.len())
        .position(|window| window == vec_2)
    {
        Ok(&vec_1[index..(index + vec_2.len())])
    } else {
        Err("Not found".to_string())
    }
}
