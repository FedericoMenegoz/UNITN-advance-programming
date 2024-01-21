mod modx {
    #[derive(Debug)]
    pub enum X {
        S(char),
        C(String),
    }
}
mod mody {
    #[derive(Debug)]
    pub enum X {
        F(f64, usize),
    }
}

mod modsum {
    use super::modx;
    use super::mody;

    pub fn sum(xx: &modx::X, xy: &mody::X) -> u8 {
        let op1 = match xx {
            modx::X::C(s) => s.len() as u8,
            modx::X::S(c) => *c as u8,
        };
        let op2 = match xy {
            mody::X::F(f, u) => f * *u as f64,
        };
        op1 + op2 as u8
    }
}

pub fn es11() {
    let xx_1 = modx::X::C("Ciao".to_owned());
    let xx_2 = modx::X::S('a');

    let xy = mody::X::F(2.5, 10);

    println!(
        "modx::X = {:?}\nmody::X = {:?}\nThe sum is {}",
        xx_1,
        xy,
        modsum::sum(&xx_1, &xy)
    );
    println!(
        "modx::X = {:?}\nmody::X = {:?}\nThe sum is {}",
        xx_2,
        xy,
        modsum::sum(&xx_2, &xy)
    );
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let xx_1 = modx::X::C("Ciao".to_owned());
        let xx_2 = modx::X::S('a');
        let xy = mody::X::F(2.5, 10);

        // ciao.len() + 2.5 * 10.0 = 25
        assert_eq!(modsum::sum(&xx_1, &xy), 29);

        // 'a' + 2.5 * 10.0 = 97 + 2.5 * 10.0
        assert_eq!('a' as u8, 97);
        assert_eq!(modsum::sum(&xx_2, &xy), 122);
    }
}
