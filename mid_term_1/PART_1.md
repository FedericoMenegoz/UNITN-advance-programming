# Part 1: multiple question choice on Rust semantic

## A: will the following code compile? Why?

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
