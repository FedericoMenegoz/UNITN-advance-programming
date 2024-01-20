# Part 1: multiple question choice on Rust semantic

## A: Will the following code compile? Why?

```Rust
fn main() {
 let x = String::from("hello");
 let y = x; // this is line 3
 println!("{}, world!", y);
 println!("{}, world!", x);
 let x = 10;
 let y = x;
 println!("y={}", y);
 println!("x={}", x);
}
```
1. yes;
2. no, x is moved;
3. no, x and y are defined twice;
4. no, x does not implement the clone trait;
5. no, x is borrowed twice;

This code snipet won't compile as the in line 3 we move the value of x into y, then in line 5 we try to print x. The compiler:
```
error[E0382]: borrow of moved value: `x`
  --> src/main.rs:65:28
   |
62 |     let x = String::from("hello");
   |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
63 |     let y = x; // this is line 3
   |             - value moved here
64 |     println!("{}, world!", y);
65 |     println!("{}, world!", x);
   |                            ^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
   |
63 |     let y = x.clone(); // this is line 3
   |              ++++++++

For more information about this error, try `rustc --explain E0382`.
```
