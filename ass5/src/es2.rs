
use rand::prelude::*;
const TITLE:[&str;4] =
[
    "Libro",
    "Jack Spusson and the Giannate Indegne.",
    "100 ways to say 'Bubusettete'",
    "This isn't really a Random generated string, but it was the only way to keep title (attribute of book) with an &str"
    ];
    #[derive(Debug, Default)]
    enum Category{
        #[default]
        Horror,
        Comic,
        Essay,
        Romance
    }
    
#[allow(dead_code)]
#[derive(Debug)]
struct Book <'a>{
    title: &'a str,
    cat: Category
}
#[derive(Debug, Default)]
struct Library <'a> {
    bookcase: [Vec<Book<'a>>; 10]
}

impl <'a> Default for Book<'a> {
    fn default() -> Self {
        let title =
            TITLE.get(thread_rng().gen_range(0..=3))
            .expect("There are 4 possible books.").clone();
        let cat = match thread_rng().gen_range(0..=3) {
            0 => Category::Horror,
            1 => Category::Comic,
            2 => Category::Essay,
            3 => Category::Romance,
            n => panic!("Should be a number from 0 to 3, but got {}!", n)
        };
        Book { title, cat}
    }
}

impl<'a> Book<'a> {
    fn default_with_cat (cat: Category) -> Book<'a>{
        Book {
            cat,
            ..Book::default()
        }
    }
}

trait Populatable {
    fn populate(&mut self);
}
 impl <'a> Populatable for Library <'a> {
     fn populate(&mut self) {
         for floor in self.bookcase.iter_mut() {
             (0..=3)
                 .into_iter()
                 .for_each(|_| {
                     floor.push(Book::default_with_cat(Category::default()));
                 })
         }
     }
 }
pub fn es2() {

    println!("Hello, world!");
    let mut lib = Library::default();
    lib.populate();
    println!("{:?}", lib);
}
