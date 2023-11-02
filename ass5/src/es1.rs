fn print_mono<T: for<'a> Printable<'a>>(gen: &T) {
    gen.printlns();
}

fn print_dyn<'a>(gen: &'a dyn Printable<'a>) {
    gen.printlns();
}
trait Printable<'a> {
    fn printlns(&'a self);
}

impl<'a> Printable<'a> for i32 {
    fn printlns(&'a self) {
        println!("{}", self);
    }
}

impl<'a, T: Printable<'a>> Printable<'a> for Vec<T> {
    fn printlns(&'a self) {
        for c in self {
            c.printlns();
        }
    }
}

impl<'a> Printable<'a> for String {
    fn printlns(&'a self) {
        println!("{:?}", self)
    }
}

pub fn es1() {
    let int: &dyn Printable = &34;
    int.printlns();
    //print_mono(&int);
    print_dyn(int);
    let stringa: &dyn Printable = &String::from("Ciao 🐻!");
    stringa.printlns();
    //print_mono(&stringa);
    print_dyn(stringa);
    let vettore: &dyn Printable = &vec![1, 2, 3, 4];
    vettore.printlns();
    //print_mono(&vettore);
    print_dyn(vettore);
    let vettore: &dyn Printable = &vec!["ciao".to_string(), "come".to_string(), "va?".to_string()];
    vettore.printlns();
    //print_mono(&vettore);
    print_dyn(vettore);
}
