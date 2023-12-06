pub fn es7() {
    let mut v = vec![1, 56, 2, 46, 111, 12, -1];
    print!("pancake_sort({:?}) = ", v);
    pancake_sort(&mut v);
    println!("{:?}", v);
}

pub fn pancake_sort(a: &mut [i32]) {
    // if a has one element skip
    if a.len() > 1 {
        // find the index of the max value
        let (index, _) = a
            .iter()
            .enumerate()
            .max_by(|(_, val1), (_, val2)| val1.cmp(val2))
            .expect("Lenght > 1");
        // split the array in two
        let (_, second_half) = a.split_at_mut(index);
        // reverse the array on top
        second_half.reverse();
        // reverse all the array
        a.reverse();
        // the largest element is positioned on the bottom
        // now call the function recursively on the rest of the
        // array
        pancake_sort(&mut a[1..]);
    }
}
