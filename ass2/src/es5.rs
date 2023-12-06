pub fn es5() {
    let mut not_sorted = vec![45, 3, 56, 12, -2];
    let mut sorted = not_sorted.clone();
    sorted.sort();
    println!(
        "The max value of {not_sorted:?} is {}",
        max(&not_sorted).unwrap()
    );
    swap(&mut not_sorted);
    println!("Swapping the same vector we get {not_sorted:?}");

    println!(
        "The vector {not_sorted:?} is sorted? {:?}",
        is_sorted(&not_sorted)
    );
    println!("The vector {sorted:?} is sorted? {:?}", is_sorted(&sorted));
    let mut parole = ["ciao", "sono"]
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    print!("Adding \"shorter\" and \"longer than 10\" to {parole:?} ");
    insert_if_longer(&mut parole, "shorter".to_string());
    insert_if_longer(&mut parole, "longer than 10".to_string());
    println!("and we get {parole:?}")
}

pub fn max(vec: &[i32]) -> Option<i32> {
    vec.iter().max().copied()
}

pub fn swap(vec: &mut [i32]) {
    let len = vec.len();
    if len > 1 {
        let (first_half, second_half) = vec.split_at_mut(1);
        std::mem::swap(&mut first_half[0], &mut second_half[len - 2])
    }
}

pub fn is_sorted(vec: &[i32]) -> bool {
    !vec.iter()
        .enumerate()
        .skip(1)
        .any(|(index, val)| *val < vec[index - 1])
}

pub fn insert_if_longer(vec: &mut Vec<String>, string: String) {
    if string.len() > 10 {
        vec.push(string);
    }
}
