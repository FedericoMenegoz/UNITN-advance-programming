#![allow(dead_code)]
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Car {
    model: String,
    year: u32,
    price: u32,
    rent: bool,
}

pub struct CarDealer(Vec<Rc<RefCell<Car>>>);

pub struct User(Option<Rc<RefCell<Car>>>);

impl CarDealer {
    fn new(cars: Vec<Car>) -> Self {
        let cars = cars
            .into_iter()
            .map(|car| Rc::new(RefCell::new(car)))
            .collect::<Vec<Rc<RefCell<Car>>>>();
        CarDealer(cars)
    }

    fn add_car(&mut self, car: Car) {
        self.0.push(Rc::new(RefCell::new(car)));
    }

    fn print_cars(&self) {
        self.0.iter().for_each(|car| println!("{:?}", car))
    }

    fn rent_user(&self, user: &mut User, model: String) -> Result<(), &str> {
        let _ = self.end_rental(user);
        return if let Some(car_to_rent) = self.0.iter().find(|&car| car.borrow().model == model) {
            let mut car = car_to_rent.borrow_mut();
            match car.rent {
                true => Err("This car is already rented."),
                false => {
                    car.rent = true;
                    user.0 = Some(Rc::clone(car_to_rent));
                    Ok(())
                }
            }
        } else {
            Err("Car not found")
        };
    }

    fn end_rental(&self, user: &mut User) -> Result<(), &str> {
        if let Some(rented_car) = &user.0 {
            rented_car.borrow_mut().rent = false;
            user.0 = None;
            Ok(())
        } else {
            Err("User has no car")
        }
    }
}
pub fn create_a_vec_of_car() -> Vec<Car> {
    let cars = vec![
        Car {
            model: "Astra".to_string(),
            year: 2005,
            price: 100,
            rent: false,
        },
        Car {
            model: "BMW".to_string(),
            year: 2005,
            price: 200,
            rent: false,
        },
        Car {
            model: "Yaris".to_string(),
            year: 2005,
            price: 200,
            rent: false,
        },
    ];
    cars
}

pub fn es2() {
    let mut car_dealer = CarDealer::new(create_a_vec_of_car());

    car_dealer.add_car(Car {
        model: "C3".to_string(),
        year: 2005,
        price: 20,
        rent: false,
    });

    let mut user = User(None);
    car_dealer.print_cars();
    match car_dealer.rent_user(&mut user, "C3".to_string()) {
        Ok(()) => println!("Rent successful!"),
        Err(msg) => println!("{msg}"),
    }
    match car_dealer.end_rental(&mut user) {
        Ok(()) => println!("Rent returned!"),
        Err(msg) => println!("{msg}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_car() {
        let mut car_dealer = CarDealer::new(vec![]);
        car_dealer.add_car(Car {
            model: "prova".to_string(),
            year: 2000,
            price: 100,
            rent: false,
        });
        assert_eq!(
            car_dealer
                .0
                .iter()
                .find(|&c| c.borrow().model == "prova".to_string())
                .map(|_| true),
            Some(true)
        );
    }

    #[test]
    fn test_rent_operation() {
        let car_dealer = CarDealer::new(create_a_vec_of_car());
        let mut user = User(None);

        // rent a car present in the car_dealer and check rent status of the car
        assert_eq!(car_dealer.rent_user(&mut user, "Astra".to_string()), Ok(()));
        assert_eq!(
            car_dealer
                .0
                .iter()
                .find(|&car| car.borrow().model == "Astra".to_string())
                .unwrap()
                .borrow()
                .rent,
            true
        );
        // end the rental and check rent status of the car
        assert_eq!(car_dealer.end_rental(&mut user), Ok(()));
        assert_eq!(
            car_dealer
                .0
                .iter()
                .find(|&car| car.borrow().model == "Astra".to_string())
                .unwrap()
                .borrow()
                .rent,
            false
        );

        // try to end a rental for a user without a car rented
        assert_eq!(car_dealer.end_rental(&mut user), Err("User has no car"));

        // try to rent a model which is not present
        assert_eq!(
            car_dealer.rent_user(&mut user, "model not present".to_string()),
            Err("Car not found")
        );

        // try to rent mutliple car will end previous rent
        assert_eq!(car_dealer.rent_user(&mut user, "Astra".to_string()), Ok(()));
        assert_eq!(car_dealer.rent_user(&mut user, "BMW".to_string()), Ok(()));
        assert_eq!(
            car_dealer
                .0
                .iter()
                .find(|&car| car.borrow().model == "Astra".to_string())
                .unwrap()
                .borrow()
                .rent,
            false
        );
    }
}
