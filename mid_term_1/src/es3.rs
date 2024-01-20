use std::mem::swap;

#[derive(Debug)]
struct NameSurname {
    _name: String,
    surname: String,
}

pub fn es3() {
    let mut gigi = NameSurname {
        _name: "Gigi".to_owned(),
        surname: "Broccolo".to_owned(),
    };
    println!(
        "Gigi want a new surname! His new choice is 'Verza'!\nOld struct:{:#?}",
        gigi,
    );
    let old_surname = replace_surname(&mut gigi, "Verza".to_owned());
    println!("The old surname was {:?}", old_surname);
    println!("The new struct is {:?}", gigi);
}

fn replace_surname(name_surname: &mut NameSurname, mut s: String) -> String {
    swap(&mut name_surname.surname, &mut s);
    s
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_surname() {
        let mut name_surname = NameSurname {
            _name: "John".to_owned(),
            surname: "Doe".to_owned(),
        };
        let old_surname = replace_surname(&mut name_surname, "Smith".to_owned());
        assert_eq!(old_surname, "Doe");
        assert_eq!(name_surname.surname, "Smith");
    }
}
