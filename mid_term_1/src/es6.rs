use std::collections::HashMap;

use crate::es6::function::string_to_tuple;

use self::{hashmaps::Maps, unumber::Unumber};

pub fn es6() {
    let keys = (0..4).collect::<Vec<Unumber>>();

    let map: HashMap<Unumber, String> = keys
        .iter()
        .map(|key| (*key, (*key).to_string() + "_stringa"))
        .collect();
    let maps = Maps { map };

    println!("{:#?}", maps);

    let maps = string_to_tuple(maps);
    println!("{:#?}", maps);
}

mod hashmaps {
    use super::unumber::Unumber;
    use std::collections::HashMap;
    #[derive(Debug)]
    pub struct Maps {
        pub map: HashMap<Unumber, String>,
    }
}

mod unumber {
    pub type Unumber = usize;
}

mod function {
    use std::collections::HashMap;

    use super::{hashmaps::Maps, unumber::Unumber};

    pub fn string_to_tuple(maps: Maps) -> HashMap<Unumber, (Unumber, String)> {
        maps.map
            .iter()
            .map(|(key, string)| (*key, (string.len(), string.to_owned())))
            .collect()
    }
}
