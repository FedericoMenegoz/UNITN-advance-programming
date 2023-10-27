
fn skip_prefix<'a>(telephone_number: &'a str, prefix: &'a str) -> &'a str {
    return if telephone_number.starts_with(prefix){
        &telephone_number[prefix.len() ..]
    } else {
        telephone_number
    }
}
pub fn es7() {
    let n1 = "04341234567";
    let p1 = "0434";
    let n2 = "45030";
    dbg!(skip_prefix(n1, p1));
    dbg!(skip_prefix(n2, p1));
}