pub fn es1() {
    println!("Is 4539 3195 0343 6467 a valid card number? {} ", is_it_luhn("4539 3195 0343 6467"));
}

pub fn is_it_luhn(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), val| {
            val.to_digit(10)
                .map(|num| if count % 2 == 1 { num * 2 } else { num })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| (num + sum, count + 1))
        }).map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
}