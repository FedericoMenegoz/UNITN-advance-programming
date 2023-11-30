use std::vec::IntoIter;

struct BinIter {
    valori: Vec<bool>,
}

impl BinIter {
    fn new(number: i128, length: usize) -> Self {
        let mut valori = vec![false; length];
        format!("{:b}", number)
            .chars()
            .rev()
            .enumerate()
            .for_each(|(index, bit)| {
                if index < length {
                    if bit == '0' {
                        valori[index] = false;
                    } else {
                        valori[index] = true;
                    }
                }
            });

        BinIter { valori }
    }
}
impl IntoIterator for BinIter {
    type Item = bool;

    type IntoIter = IntoIter<bool>;

    fn into_iter(self) -> Self::IntoIter {
        self.valori.into_iter()
    }
}

pub fn es1() {
    println!("Test 1:");
    test_1();
    println!("Test 2:");
    test_2();
}

fn test_1() {
    for n in BinIter::new(4641312598359562305508665788689819792, 128) {
        print!("{}", if n { 1 } else { 0 })
    }
    println!();
    for n in BinIter::new(18956403638425120546, 64) {
        print!("{}", if n { 1 } else { 0 })
    }
    println!();
    for n in BinIter::new(15, 4) {
        print!("{}", if n { 1 } else { 0 })
    }
    println!();
}

fn test_2() {
    for n in BinIter::new(1568948940, 16) {
        print!("{}", if n { "*" } else { "_" })
    }
    println!();
    for n in BinIter::new(8978540, 16) {
        print!("{}", if n { "*" } else { "_" })
    }
    println!();
    for n in BinIter::new(375, 9) {
        print!("{}", if n { "*" } else { "_" })
    }
    println!();
}
// 000010010001010000100101001110110111000101000101010101100011110101001101101001001000100100110111000001100100011110111110110000000
// 00001001000101000010010100111011011100010100010101010110001111010100110110100100100010010011011100000110010001111011111011000000
