
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
struct Person <'a>{
    name: String,
    father: Option<&'a Person<'a>>,
    mother: Option<&'a Person<'a>>
}
impl <'a> Display for Person<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let m_name = match self.mother {
            Some(t) => t.name.clone(),
            _ => "not present".to_string()
        };
        let f_name = match self.father {
            Some(t) => t.name.clone(),
            _ => "not present".to_string()
        };

        write!(f, "Person {{Name: {} [Father: {} Mother: {}]}}", self.name, f_name, m_name)
    }
}

impl <'a> Debug for Person<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl <'a> Person <'a> {
    fn new(name: &str, father: Option<&'a Person>, mother: Option<&'a Person>) -> Self{
        Person { name: name.to_string(), father, mother}
    }
    fn find_relatives (&self, generations: u32) -> Vec<&Person> {
        let mut parents = Vec::new();
        parents.push(self);
        if generations > 0 {
            let (mut from_mother, mut from_father) = (Vec::new(), Vec::new());
            match self.mother {
                Some(p) => {
                    from_mother = p.find_relatives(generations - 1);
                },
                None => ()
            };
            match self.father {
                Some(p) => {
                    from_father = p.find_relatives(generations - 1);
                },
                None => ()
            }
            from_mother.append(&mut from_father);
            parents.append(&mut from_mother);
        };
        parents
    }
    fn find_roots(&self) -> Vec<&Person> {
        let mut roots  = Vec::new();
        match (self.mother, self.father) {
            (_, None) | (None , _) => roots.append(&mut vec!(self)),
            _ => {
                roots = self.father.unwrap().find_roots();
                roots.append(&mut self.mother.unwrap().find_roots())
            }
        }
        roots
    }
}

pub fn es2() {
    let nonno_mamma = Person::new("Severino", None, None);
    let nonna_mamma = Person::new("Augusta", None, None);
    let nonno_papà = Person::new("Marco", None, None);
    let nonna_papà = Person::new("Maddalena", None, None);
    let papà = Person::new("Massimo", Some(&nonna_papà), Some(&nonno_papà));
    let mamma = Person::new("Bruna", Some(&nonna_mamma), Some(&nonno_mamma));
    let io = Person::new("Federico", Some(&papà), Some(&mamma));
    println!("{:?}",(nonno_mamma.find_relatives(3)));
    println!("{:?}",(io.find_relatives(3)));
    println!("{:?}",(io.find_roots()));
}
