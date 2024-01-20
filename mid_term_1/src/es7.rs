use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct Size {
    width: u32,
    height: u32,
}

impl Size {
    fn new(width: u32, height: u32) -> Self {
        Size { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn compare(&self, other: &Size) -> Option<bool> {
        match self.area().cmp(&other.area()) {
            Ordering::Equal => None,
            Ordering::Greater => Some(true),
            Ordering::Less => Some(false),
        }
    }
}
pub fn es7() {
    let size_1 = Size::new(2, 3);
    let size_2 = Size::new(3, 2);
    let size_3 = Size::new(3, 5);
    println!("Same area = None");
    println!("Greater area = Some(true)");
    println!("Smaller area = Some(false)");
    println!(
        "{:?} has same area of {:?}? {:?}",
        size_1,
        size_2,
        size_1.compare(&size_2)
    );
    println!(
        "{:?} has same area of {:?}? {:?}",
        size_1,
        size_3,
        size_1.compare(&size_3)
    );
    println!(
        "{:?} has same area of {:?}? {:?}",
        size_3,
        size_1,
        size_3.compare(&size_1)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_same_area() {
        let size_1 = Size::new(2, 3);
        let size_2 = Size::new(3, 2);
        assert_eq!(size_1.compare(&size_2), None);
    }

    #[test]
    fn test_compare_greater_area() {
        let size_1 = Size::new(2, 3);
        let size_3 = Size::new(3, 5);
        assert_eq!(size_1.compare(&size_3), Some(false));
    }

    #[test]
    fn test_compare_smaller_area() {
        let size_1 = Size::new(2, 3);
        let size_3 = Size::new(3, 5);
        assert_eq!(size_3.compare(&size_1), Some(true));
    }
}
