

use std::collections::LinkedList;

trait Split <'a, T:'a> {
    fn splitt(&'a self) -> (T, T);
}

 impl <'a> Split <'a ,&'a str> for String {
     fn splitt(&self) -> (&str, &str) {
         let middle = self.len() / 2;
         self.split_at(middle)
     }
 }

 impl <'a> Split <'a, &'a[i32]> for &[i32] {
     fn splitt(&self) -> (&[i32], &[i32]) {
         let middle = self.len()/2;
         self.split_at(middle)
     }
}

impl <'a> Split <'a, LinkedList<f64>> for LinkedList <f64> {
    fn splitt(&self) -> (LinkedList<f64>, LinkedList<f64>) {
        let front = self.len()/2;
        let end = self.len() - front;
        let vet = self.iter().copied().collect::<Vec<f64>>();
        let (mut first_half, mut second_half) = (LinkedList::new(), LinkedList::new());
        (0..front).into_iter()
            .for_each(|x| {
                first_half.push_back(vet.iter().nth(x).copied().unwrap());
        });
        (0..end).into_iter()
            .for_each(|x| {
                second_half.push_back(vet.iter().rev().nth(x).copied().unwrap())
            });
        (first_half, second_half)
    }
}

pub fn es5() {
    let ll = LinkedList::from([2.3,4.5,4.4,1.1,0.3]);
    let s1 = "ciao".to_owned();
    let a1 = &[1,2,3,4,5];
    dbg!(s1.splitt());
    dbg!((&a1[..]).splitt());
    dbg!(ll.splitt());

}
