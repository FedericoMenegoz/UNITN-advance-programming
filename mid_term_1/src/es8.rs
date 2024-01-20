#[derive(Debug)]
struct MaybePoint {
    x: Option<i32>,
    y: Option<i32>,
}

impl MaybePoint {
    fn new(x: Option<i32>, y: Option<i32>) -> Self {
        MaybePoint { x, y }
    }

    fn is_some(&self) -> bool {
        self.x.is_some() && self.y.is_some()
    }

    fn maybe_len(&self) -> Option<f32> {
        if self.is_some() {
            Some(self.x.unwrap() as f32 + self.y.unwrap() as f32)
        } else {
            None
        }
    }
}

pub fn es8() {
    let point = MaybePoint::new(Some(3), Some(4));

    if point.is_some() {
        println!("Point has x and y values: {:?}", point);
    } else {
        println!("Point does not have x and y values");
    }

    if let Some(len) = point.maybe_len() {
        println!("Point length: {}", len);
    } else {
        println!("Point length is not available");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_some() {
        let point = MaybePoint::new(Some(3), Some(4));
        assert_eq!(point.is_some(), true);
    }

    #[test]
    fn test_maybe_len() {
        let point = MaybePoint::new(Some(3), Some(4));
        assert_eq!(point.maybe_len(), Some(7.0));
    }

    #[test]
    fn test_maybe_len_none() {
        let point = MaybePoint::new(None, Some(4));
        assert_eq!(point.maybe_len(), None);
    }
}
