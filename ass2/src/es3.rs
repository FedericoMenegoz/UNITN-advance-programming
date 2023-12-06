pub fn es3() {
    let slice = [1, 2, 3, 4, 5];
    let sub = split_at_value(&slice, 3).unwrap();
    println!(
        "Given the slice [1,2,3,4,5] and the value 3, we get the followiong slices: {:?} and {:?}",
        sub.0, sub.1
    );
}

pub fn split_at_value(slice: &[i32], value: i32) -> Option<(&[i32], &[i32])> {
    let pos = slice.iter().position(|item| *item == value)?;
    Some(slice.split_at(pos))
}
