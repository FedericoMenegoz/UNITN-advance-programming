#[derive(Debug)]
struct AirFleet(Vec<Airplane>);

#[derive(Debug)]
struct Airplane {
    company: AirplaneCompany,
    model: String,
}

#[derive(PartialEq, Clone, Debug)]
enum AirplaneCompany {
    Airbus,
    Boeing,
}

impl AirFleet {
    fn remove_boeing(&mut self) {
        self.0
            .retain(|plane| plane.company != AirplaneCompany::Boeing);
    }

    fn add_airplane(&mut self, plane: Airplane) {
        self.0.push(plane);
    }

    fn search_airplane(&self, model: &str) -> Result<AirplaneCompany, String> {
        match self.0.iter().find(|&plane| plane.model == model) {
            Some(plane) => Ok(plane.company.clone()),
            None => Err("Model not found!".to_owned()),
        }
    }
}
pub fn es5() {
    let mut fleet = AirFleet(Vec::new());

    let airplane1 = Airplane {
        company: AirplaneCompany::Airbus,
        model: String::from("A320"),
    };

    let airplane2 = Airplane {
        company: AirplaneCompany::Boeing,
        model: String::from("747"),
    };

    fleet.add_airplane(airplane1);
    fleet.add_airplane(airplane2);

    println!("Your fleet: {:#?}", fleet);

    fleet.remove_boeing();

    match fleet.search_airplane("A320") {
        Ok(company) => println!("Found airplane of company: {:?}", company),
        Err(error) => println!("Error: {}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_boeing() {
        let mut fleet = AirFleet(Vec::new());

        let airplane1 = Airplane {
            company: AirplaneCompany::Airbus,
            model: String::from("A320"),
        };

        let airplane2 = Airplane {
            company: AirplaneCompany::Boeing,
            model: String::from("747"),
        };

        fleet.add_airplane(airplane1);
        fleet.add_airplane(airplane2);

        fleet.remove_boeing();

        assert_eq!(fleet.0.len(), 1);
        assert_eq!(fleet.0[0].company, AirplaneCompany::Airbus);

        fleet.remove_boeing();
        assert_eq!(fleet.0[0].company, AirplaneCompany::Airbus);
    }

    #[test]
    fn test_search_airplane_found() {
        let mut fleet = AirFleet(Vec::new());

        let airplane1 = Airplane {
            company: AirplaneCompany::Airbus,
            model: String::from("A320"),
        };

        let airplane2 = Airplane {
            company: AirplaneCompany::Boeing,
            model: String::from("747"),
        };

        fleet.add_airplane(airplane1);
        fleet.add_airplane(airplane2);
        match fleet.search_airplane("A320") {
            Ok(company) => assert_eq!(company, AirplaneCompany::Airbus),
            Err(_) => panic!("Expected airplane not found!"),
        }
    }

    #[test]
    fn test_search_airplane_not_found() {
        let mut fleet = AirFleet(Vec::new());

        let airplane1 = Airplane {
            company: AirplaneCompany::Airbus,
            model: String::from("A320"),
        };

        let airplane2 = Airplane {
            company: AirplaneCompany::Boeing,
            model: String::from("747"),
        };

        fleet.add_airplane(airplane1);
        fleet.add_airplane(airplane2);

        match fleet.search_airplane("A380") {
            Ok(_) => panic!("Unexpected airplane found!"),
            Err(error) => assert_eq!(error, "Model not found!"),
        }
    }
}
