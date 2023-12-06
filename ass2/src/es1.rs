pub fn es1() {
    println!("{:?}", create_vec(100));
}

pub fn modify_odd(array: &mut [u32]) {
    array.iter_mut().for_each(|n| {
        if *n % 2 == 1 {
            *n = 0;
        }
    })
}

pub fn create_vec(n: u32) -> Vec<u32> {
    let mut vettore = (0..=n).collect::<Vec<u32>>();
    modify_odd(&mut vettore);
    vettore
}
