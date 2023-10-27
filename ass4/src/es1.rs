

 use rand::distributions::{Alphanumeric, DistString};
 pub fn es1() {
     let mut trovato = false;
     while !trovato {
         match lucky_slice("ciao") {
             None => continue,
             Some(c) => {
                 println!("{c}");
                 trovato = true;
             }
         }
     }
 }
 
 fn find_equal <'a, 'b>(s1: &'a str, s2: &'b str) -> Option<(&'a str, &'b str)> {
     if s1.len() > 1 || s2.len() > 1 {
         for i in 0..s1.len()-1 {
             let right= &s1[i..i+2];
             match s2.find(right) {
                 Some(c) => {
                     let (_, slice_s2) = s2.split_at(c);
                     let(slice_s2, _) = slice_s2.split_at(2);
                     return Some((right, slice_s2));
                 }
                 None => continue
             }
         }
    }
     None
 }

 fn lucky_slice(input_str: &str)-> Option<&str> {
     let string = Alphanumeric.sample_string(&mut rand::thread_rng(), input_str.len());
     let res = find_equal(input_str, &string);
 
     match res {
         None => None,
         Some(c) => Some(c.0)
     }
 }