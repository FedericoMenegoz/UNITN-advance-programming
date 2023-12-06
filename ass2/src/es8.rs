pub fn es8() {
    let slice_1 = [3, 67, 990];
    let slice_2 = [-1, 1, 90];

    println!(
        "Merging {slice_1:?} and {slice_2:?} => {:?}",
        merge(&slice_1, &slice_2)
    );
}

pub fn merge(slice_1: &[i32], slice_2: &[i32]) -> Vec<i32> {
    let mut it1 = slice_1.iter();
    let mut it2 = slice_2.iter();
    let mut val1 = it1.next();
    let mut val2 = it2.next();

    let mut merged = vec![];

    while val1.is_some() || val2.is_some() {
        match (val1, val2) {
            (Some(v1), Some(v2)) => {
                if v1 <= v2 {
                    merged.push(*v1);
                    val1 = it1.next();
                } else {
                    merged.push(*v2);
                    val2 = it2.next();
                }
            }
            (None, Some(v2)) => {
                merged.push(*v2);
                val2 = it2.next();
            }
            (Some(v1), None) => {
                merged.push(*v1);
                val1 = it1.next();
            }
            _ => {}
        }
    }
    merged
}
