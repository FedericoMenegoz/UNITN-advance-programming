struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl <'a, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) ->&'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
pub fn es3() {
    let annotation = ImportantExcerpt {part: "This was the old announcement."};
    let ret = annotation.announce_and_return_part("Rust announcement!");
    println!("{ret}");
}