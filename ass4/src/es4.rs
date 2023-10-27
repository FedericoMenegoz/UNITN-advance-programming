
#[derive(Debug)]
struct DoubleRef<'a, 'b:'a, T: ?Sized> {
    r: &'b T,
    s: &'a T
}


pub fn es4() {
    let live_longer = "I live longer.".to_string();
    let dr;
    {
        let live_less = "About to die.".to_string();
        dr = DoubleRef {r: &live_less, s:&live_longer};
        let ret = longest(&dr.r, &dr.s);
        println!("{}", ret);
        let ret = longest(&dr.s, &dr.r);
        longest(&live_less, &live_longer);
        longest(&live_longer, &live_less);
        println!("{}", ret);
        print!("{:?}", dr);
    }
    
    todo!("

    \"Understand when this is useful. 
    Find a case when you need to specify that one lifetime outlives the other.
    Everything works here.\"
    
    ");
}

fn longest<'b, 'a: 'b>(x:&'a str, y:&'b str) -> &'b str {
    if x.len() > y.len() { x } else { y}
} // this funcion doesn't work