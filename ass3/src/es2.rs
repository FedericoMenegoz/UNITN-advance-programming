use std::fmt;
use std::fmt::Formatter;

enum Fuel {
    Diesel,
    Gasoline,
    LPG,
    Methane,
    Electric
}

enum IPAdd {
    IPv4((u8,u8,u8)),
    IPv6((AddressGroup,AddressGroup,AddressGroup,AddressGroup))
}

struct AddressGroup {
    c1: u8,
    c2: u8,
    c3: u8,
    c4: u8,
}

impl AddressGroup {
    fn new (c1 :u8, c2 :u8, c3:u8, c4:u8) -> Self {
        if c1 > 15 || c2 > 15 || c3 > 15 || c4 > 15 { panic!("Values from 0 a 15!")}
        AddressGroup {
            c1,
            c2,
            c3,
            c4
        }
    }
}
impl fmt::Display for AddressGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}{:x}{:x}{:x}", self.c1, self.c2, self.c3, self.c4)
    }
}

impl fmt::Display for IPAdd {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        match self {
            IPAdd::IPv4(c) => write!(f, "{}.{}.{}", c.0, c.1, c.2),
            IPAdd::IPv6(c) =>  write!(f,"{}:{}:{}:{}", c.0, c.1, c.2, c.3)
        }

    }
}

pub fn es2() {
    let ip = IPAdd::IPv4((12,12,12));
    let ip2 = IPAdd::IPv6((
        AddressGroup::new(13, 3, 12, 4),
        AddressGroup::new(11, 3, 12, 4),
        AddressGroup::new(0, 3, 12, 4),
        AddressGroup::new(6, 3, 12, 4),
    ));
    println!("{}, {}", ip, ip2);
}