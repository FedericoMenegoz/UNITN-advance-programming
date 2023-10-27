use std::fmt::Formatter;

trait Object {
    fn build(&self) -> &str;
    fn get_quantity(&self)-> String;
}
struct Chair <'a> {
    color: &'a str,
    quantity: &'a usize
}

impl <'a> Object for Chair<'a> {
    fn build(&self) -> &'static str {
        "Chair has been built"
    }

    fn get_quantity(&self) -> String {
        format!("{}", self.quantity)
    }
}

struct Wardrobe <'a> {
    color: &'a str,
    quantity: &'a usize
}
impl <'a> Object for Wardrobe<'a> {
    fn build(&self) -> &'static str {
        "Wardrobe has been built."
    }

    fn get_quantity(&self) -> String {
        format!("{}", self.quantity)
    }
}

impl <'a>std::fmt::Display for Wardrobe<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output = match self.quantity {
            0 => format!("There are no wardrobe of color {}.", self.color),
            1 => format!("There is 1 wardrobe of color {}.", self.color),
            _ => format!("There are {} wardrobe of color {}.", self.quantity, self.color),
        };
        write!(f, "{}", output)
    }
}

impl <'a> std::fmt::Display for Chair<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output = match self.quantity {
            0 => format!("There are no chairs of color {}.", self.color),
            1 => format!("There is 1 chair of color {}.", self.color),
            _ => format!("There are {} chairs of color {}.", self.quantity, self.color),
        };
        write!(f, "{}", output)
    }
}
pub fn es8() {
    let red_chair = Chair{ color: "red", quantity: &2};
    let blue_chair = Chair{ color: "blue", quantity: &0};
    let yellow_chair = Chair{ color: "yellow", quantity: &1};
    let red_wardrobe = Wardrobe{ color: "red", quantity: &20};
    let blue_wardrobe = Wardrobe{ color: "blue", quantity: &0};
    let yellow_wardrobe = Wardrobe{ color: "yellow", quantity: &1};

    println!("{}", red_chair.build());
    println!("{}", red_wardrobe.build());

    println!("Inventory:\n{}\n{}\n{}\n{}\n{}\n{}", red_chair, red_wardrobe, blue_chair, blue_wardrobe, yellow_chair, yellow_wardrobe);
}
