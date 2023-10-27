use CarrotState::{Raw, Cooked, Burnt, Fried};

const ERR_MSG_CHANGES_CARROTSTATE: &str = "
Did you add a variant to the enum CarrotState?
Well better check if need implementation here!";
trait Heatable {
    fn cook(&mut self);
}

trait Friable {
    fn cook(&mut self);
}

trait Heater {
    fn heat(&self, food: &mut dyn Heatable);
}
trait Frier {
    fn fry(&self, food: &mut dyn Friable);   
}

struct Oven;

struct Pan;

impl Heater for Oven {
    fn heat(&self, food: &mut dyn Heatable) {
        food.cook();
    }
}
impl Heater for Pan {
    fn heat(&self, food: &mut dyn Heatable) {
        food.cook();
    }
}
impl Frier for Pan {
    fn fry(&self, food: &mut dyn Friable) {
        food.cook();
    }
}

struct Pie {
    ready: bool
}

struct Carrot {
    state: CarrotState
}

enum CarrotState {
    Raw,
    Cooked,
    Fried,
    Burnt,
}

trait Edible {
    fn eat(&self);
}

impl Heatable for Pie {
    fn cook(&mut self) {
        match self.ready {
            true => println!("You burned the pie!"),
            false => self.ready = true
        }
    }
}
#[allow(unreachable_patterns)]
impl Heatable for Carrot {
    fn cook(&mut self) {
        match self.state {
            Raw => self.state = Cooked,
            Burnt | Cooked | Fried => self.state = Burnt,
            _ => panic!("{ERR_MSG_CHANGES_CARROTSTATE}")
        }
    }
}

#[allow(unreachable_patterns)]
impl Friable for Carrot {
    fn cook(&mut self) {
        match self.state {
            Fried => self.state = Burnt,
            Cooked | Raw | Burnt => self.state = Fried,
            _ => panic!("{ERR_MSG_CHANGES_CARROTSTATE}")

        }
    }
}

impl Edible for Pie {
    fn eat(&self) {
        match self.ready {
            true => println!("Yummy!"),
            false => println!("You go stomach ache!")
        }
    }
}

#[allow(unreachable_patterns)]
impl Edible for Carrot {
    fn eat(&self) {
        match self.state {
            Raw => println!("mmh, crunchy"),
            Cooked => println!("mmh, yummy"),
            Fried => println!("mmh, crispy"),
            Burnt => println!("mmh, burnt"),
            _ => panic!("{ERR_MSG_CHANGES_CARROTSTATE}")
        }
    }
}

pub fn es7() {
    let mut carrot = Carrot { state: Raw };
    carrot.eat();
    Heatable::cook(&mut carrot);
    carrot.eat();
    Heatable::cook(&mut carrot);
    carrot.eat();
    Friable::cook(&mut carrot);
    carrot.eat();
    
    let mut pie = Pie { ready: false };
    let pan = Pan;
    pan.heat(&mut pie);
    pan.heat(&mut pie);
    
    let mut carrot =  Carrot { state: Raw }; 
    pan.fry(&mut carrot);
    
    pan.fry(&mut carrot);
    
    carrot.eat();

    let heater = Oven;
    heater.heat(&mut pie);

}