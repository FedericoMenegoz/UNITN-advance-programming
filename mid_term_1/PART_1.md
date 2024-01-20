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
- [ ] yes;
- [ ] no, x is moved;
- [ ] no, x and y are defined twice;
- [ ] no, x does not implement the clone trait;
- [ ] no, x is borrowed twice;