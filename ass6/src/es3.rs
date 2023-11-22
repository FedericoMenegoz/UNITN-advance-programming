use std::fmt::{Debug, Display};
trait Sound {
    fn make_sound(&self) -> String;
}

#[derive(Debug)]
struct Cat();
#[derive(Debug)]
struct Dog();
#[derive(Debug)]
struct Cow();

impl Sound for Cat {
    fn make_sound(&self) -> String {
        "Miaaaoooo!".to_string()
    }
}

impl Sound for Dog {
    fn make_sound(&self) -> String {
        "Baaaaauuu!".to_string()
    }
}

impl Sound for Cow {
    fn make_sound(&self) -> String {
        "Muuuuuuuu!".to_string()
    }
}

impl<'a> Debug for dyn Sound + 'a {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.make_sound())
    }
}

impl<'a> PartialEq for dyn Sound + 'a {
    fn eq(&self, other: &dyn Sound) -> bool {
        self.make_sound() == other.make_sound()
    }
}

impl<'a> Display for FarmCell<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.element.make_sound())
    }
}

#[derive(Debug, PartialEq)]
struct FarmCell<'a> {
    element: &'a dyn Sound,
    next: Option<Box<FarmCell<'a>>>,
}

struct FarmCellIterator<'a> {
    current: Option<&'a FarmCell<'a>>,
}

impl<'a> Iterator for FarmCellIterator<'a> {
    type Item = &'a FarmCell<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(animal) = self.current {
            self.current = animal.next.as_deref();
            Some(animal)
        } else {
            None
        }
    }
}

impl<'a> FarmCell<'a> {
    fn new(animal: &'a dyn Sound) -> Self {
        FarmCell {
            element: animal,
            next: None,
        }
    }

    fn insert(self, animal: &'a dyn Sound) -> Self {
        let mut to_insert = FarmCell::new(animal);
        to_insert.next = Some(Box::new(self));
        to_insert
    }

    fn iter(&'a self) -> FarmCellIterator<'a> {
        FarmCellIterator {
            current: Some(self),
        }
    }
}

impl<'a> Sound for FarmCell<'a> {
    fn make_sound(&self) -> String {
        let result = self
            .iter()
            .fold("".to_string(), |acc, cell| acc + &cell.element.make_sound());
        result
    }
}

pub fn es3() {
    let cat = Cat();
    let dog = Dog();
    let cow = Cow();

    let mut farm = FarmCell::new(&cat);
    farm = farm.insert(&dog);
    farm = farm.insert(&cow);

    for animal in farm.iter() {
        println!("{animal}")
    }

    println!("{}", farm.make_sound());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics3() {
        let cat = Cat();
        let dog = Dog();
        let cow = Cow();

        let mut farm = FarmCell::new(&cat);
        farm = farm.insert(&dog);
        farm = farm.insert(&cow);
        let mut farm_iter = farm.iter();
        assert_eq!(
            farm_iter.next().unwrap().element.make_sound(),
            cow.make_sound()
        );
        assert_eq!(
            farm_iter.next().unwrap().element.make_sound(),
            dog.make_sound()
        );
        assert_eq!(
            farm_iter.next().unwrap().element.make_sound(),
            cat.make_sound()
        );
        assert_eq!(farm_iter.next(), None);

        assert_eq!(
            farm.make_sound(),
            "Muuuuuuuu!Baaaaauuu!Miaaaoooo!".to_string()
        );
    }
}
