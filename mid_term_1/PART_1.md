# Part 1: multiple question choice on Rust semantic

## Question 1
### A - Will the following code compile? Why?

```Rust
fn main() {
    let x = String::from("hello");
    let y = x; // this is line 3
    println!("{}, world!", y);
    println!("{}, world!", x); // this line 5
    let x = 10;
    let y = x;
    println!("y={}", y);
    println!("x={}", x);
}
```
1. yes;
- [X] **no, x is moved**;
3. no, x and y are defined twice;
4. no, x does not implement the clone trait;
5. no, x is borrowed twice;

### Explanation:
1. This code snipet won't compile as in line 3 we move the value of `x` into `y`, then in line 5 we try to borrow `x` for printing. The compiler:
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
3. in rust varable shadowing is allowed; 
4. x implement the clone trait, the trait that would allow this code to compile is the Copy trait as written in the compiler error message.;
5. x is immutably borrowed twice, which is fine, if it was a mutable borrow then we would get a compile error;

### B - If A does not compile how could it be fixed?
- [X] replace line 3 with let y = x.clone();
2. replace line 3 with let y: &String = x;
3. add y.drop(); between the first and the second println!
4. rename x and y in the second half of the code to x_1 and y_1
5. delete println!("{}, world!", y);

### Explanation:
1. x.clone() create a copy of the variable without moving, this would allow to use both variable after
2. x is of type String, to make it compile we need to add the "&" before x => let y = &x;
3. drop would only free the variable y, it won't move back the content to x
4. this has nothing to do with the error in line 5
5. we should delete line 5 not line 4

## Question 2
### A - Will the following code compile? Why?

```Rust
let x = &[1,2,3,4];
let y = x; // this is line 2
println!("{:?}", y);
println!("{:?}", x);
```

- [X] yeah;
2. no, x is moved;
3. no, [i32, 4] does not implement the copy trait;
4. no, x does not implement Debug;

### Explanation
This code compile fine because line 2 is not moving the value of x, it is immutably borrowing it.

### B - If not, how to fix it?
No need to fix it.

## Question 3
### A - The owneer of vec's original data at HERE is?
```Rust
fn foo(vec: Vec<i32>) { // this is line 1
    let x = vec.clone();
    let y = vec;
    let z = &y;
    let w = y;
    // HERE
}
```
1. x
2. y
3. z
- [X] w
5. the code doesn't compile

### B - When the value of `x` is dropped?
1. after line 2
2. after line 3
3. after line 4
- [X] after line 5 (because it is automatically dropped when it goes out of scope)

## Question 4
### A - Will the following code compile? Why?

```Rust
fn main() {
    let s1 = String::from("hello ");
    let s2 = String::from("world!");
    let s3 = concat(s1,s2);
    println!("{} + {} = {}",s1,s2,s3); // this is line 5
}
fn concat(s1:String, s2: String) -> String {
    s1 + s2.as_str()
}
```

1. yes
- [X] no, s1 and s2 are moved inside the function
3. no, is not possible to concatenate s1 to s2 since s1 is not mutable ()
4. no, the String type dose not support the + operator

### Explanation
1. No it does not compile as the function concat will take ownership of s1 and s2, so we will get the error in line 5 of "borrow of moved value":
```
error[E0382]: borrow of moved value: `s1`
  --> src/main.rs:66:29
   |
63 |     let s1 = String::from("hello ");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
64 |     let s2 = String::from("world!");
65 |     let s3 = concat(s1,s2);
   |                     -- value moved here
66 |     println!("{} + {} = {}",s1,s2,s3); // this is line 5
   |                             ^^ value borrowed here after move
   |
note: consider changing this parameter type in function `concat` to borrow instead if owning the value isn't necessary
  --> src/main.rs:68:14
   |
68 | fn concat(s1:String, s2: String) -> String {
   |    ------    ^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
```
2. correct
3. s1 is not mutable, but the concat function is not modifying s1, it is returning the s1 + s2.as_string as a new String.
4. yes it does

### B - If not, how to fix it?

- [X] replace line 4 with let s3 = concat(s1.clone(),s2.clone())
2. the function should contain return (s1 + s2.as_str()).clone();
3. replace line 5 with println!("{} + {} = {}",&s1,&s2,&s3);

## Question 5
### Does the following code compile?
```Rust
{
    let x: &u8;
    let v = 10;
    match v {
        0..=10 => x = &v,
        _ => {}
    }
    println!("{x}");
}
```

1. Yes,
2. No, the x reference must be initialized at the first line
3. No, the x reference is not mutable and can't be overwritten with &v
- [X] No, x is possibly uninitialized

### Explanation
```
error[E0381]: used binding `x` is possibly-uninitialized
  --> src/main.rs:70:19
   |
64 |         let x: &u8;
   |             - binding declared here but left uninitialized
...
67 |             0..=10 => x = &v,
   |                       ------ binding initialized here in some conditions
...
70 |         println!("{x}");
   |                   ^^^ `x` used here but it is possibly-uninitialized
```
## Question 6
### Does the following code compile?

```Rust
let mut value: u8 = 5;
{
    let second = &mut value;
    {
        let third = 12;
        *second = third; //this is line 6
    }
}
println!("{}", value);
```
1. No, third has type i32, and can't be assigned to a u8 type.
2. No, third's life time ends after line 6, and can't be borrowed at line 9.
3. Yes, it displays 5
- [X] Yes, it displays 12

### Explanation
1. writing explicity the type in the first line, will make the compiler automatically infer `third`'s type as u8
2. It is true that `third`'s life time will end after line 6, but it is not borrowed at line 9 we are borrowing `value`
3. Nope it does not display 5, read next point;
4. The program does like this:
    - *first line*: define the variable `value` of type u8;
    - *third line*: define the variable `second` of type mutable reference of u8
    - *fifth line*: define the variable `third` (type inference u8) 
    - *sixth line*: as third is of type u8 the content of `third` it is copied inside the variable referenced by `second` (this program would compile even if `value`, `second` and `third` where of type String, the value would be then moved from `third` though) so it is coping 12 into value
    - *seventh line*: `third` is dropped
    - *eightth line*: `second` is dropped
    - *nineth line*: `value` is immutably borrowed and printed

    ## Question 7
    ### Given the following code, what can I add at line 2 without braking the code?

```Rust
let s = [1,2,3,4];
// This is line 2
let x = &s;
println!( "{:?}", x);
```

1. `s[0] = 0;`
2. `let b = &mut s;`
- [X] `let slice = &s[0..2];`
4. `s.push(5)`
5. None of the above

### Explanation
1. s is not mutable so I can't mutate its content
2. s is not mutable so I can't mutably borrow it
3. this will work as I can immutably borrow a variable many times
4. same as 1 and 2
5. .
