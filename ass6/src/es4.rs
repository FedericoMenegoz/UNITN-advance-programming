use std::fmt::Display;

use rand::Rng;
#[derive(Debug, PartialEq)]
struct PublicStreetlight {
    id: String,
    on: bool,
    burn_out: bool,
}
#[derive(PartialEq, Debug)]
struct PublicIllumination {
    lights: Vec<PublicStreetlight>,
}

struct PublicIlluminationIterBurnOut<'a> {
    current: usize,
    lights: &'a mut Vec<PublicStreetlight>,
}

impl<'a> Iterator for PublicIlluminationIterBurnOut<'a> {
    type Item = PublicStreetlight;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(_broken_bulb) = self.lights.iter().enumerate().find(|(index, light)| {
            self.current = *index;
            light.burn_out
        }) {
            Some(self.lights.remove(self.current))
        } else {
            None
        }
    }
}

impl Display for PublicStreetlight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("The bulb {} ", self.id);
        match self.burn_out {
            false => msg += "is working fine.",
            true => msg += "needs repairing.",
        }
        write!(f, "{msg}")
    }
}

impl Display for PublicIllumination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = "PUBBLIC ILLUMINATION STATUS\n".to_string();
        self.lights
            .iter()
            .for_each(|light| msg = format!("{msg}\n{light}"));
        writeln!(f, "{msg}")
    }
}
impl<'a> PublicIllumination {
    fn new(lights: Vec<PublicStreetlight>) -> PublicIllumination {
        PublicIllumination { lights }
    }

    fn iter_burn_out(&'a mut self) -> PublicIlluminationIterBurnOut<'a> {
        PublicIlluminationIterBurnOut {
            current: 0,
            lights: &mut self.lights,
        }
    }

    fn switch_all(&mut self, on: bool) {
        let mut rng = rand::thread_rng();
        self.lights.iter_mut().for_each(|light| {
            light.burn_out |= rng.gen_bool(0.2);
            light.on = on && !light.burn_out;
        })
    }

    fn print_all(&self) {
        self.lights.iter().for_each(|light| println!("{light:?}"));
        println!();
    }
}
impl Default for PublicStreetlight {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            on: false,
            burn_out: false,
        }
    }
}
impl PublicStreetlight {
    fn new(id: String) -> Self {
        PublicStreetlight {
            id,
            ..Default::default()
        }
    }
}

pub fn es4() {
    let mut lights = Vec::new();

    (0..5).for_each(|i| lights.push(PublicStreetlight::new(i.to_string())));

    let mut illumination = PublicIllumination::new(lights);

    illumination.print_all();

    illumination.switch_all(true);
    println!("{illumination}");
    illumination.switch_all(false);
    println!("{illumination}");

    let iter = illumination.iter_burn_out();
    for broken_light in iter {
        println!("Removing {broken_light}");
    }

    illumination.switch_all(true);
    println!("\n{illumination}");

    illumination.switch_all(false);
    println!("{illumination}");

    let iter = illumination.iter_burn_out();
    for broken_light in iter {
        println!("Removing {broken_light}");
    }
    println!("\n{illumination}");
}

#[cfg(test)]
mod tests {
    use crate::es4::PublicIllumination;

    use super::PublicStreetlight;

    #[test]
    fn basics4() {
        let light_0 = PublicStreetlight::default();

        assert_eq!(light_0.id, "".to_owned());
        assert_eq!(light_0.on, false);
        assert_eq!(light_0.burn_out, false);

        let light_1 = PublicStreetlight::new("1".to_string());
        assert_eq!(light_1.id, "1".to_owned());
        assert_eq!(light_1.on, false);
        assert_eq!(light_1.burn_out, false);

        let mut light_2 = PublicStreetlight::new("2".to_string());
        let mut light_3 = PublicStreetlight::new("3".to_string());

        light_2.burn_out = true;
        light_3.burn_out = true;

        let mut ill = PublicIllumination::new(vec![light_0, light_1, light_2, light_3]);
        let iter_broken = ill.iter_burn_out();

        for broken_bulb in iter_broken {
            assert_eq!(broken_bulb.burn_out, true)
        }

        assert_eq!(
            ill,
            PublicIllumination::new(vec![
                PublicStreetlight::default(),
                PublicStreetlight::new(1.to_string())
            ])
        );
    }
}
