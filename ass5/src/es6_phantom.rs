use rand::Rng;
use std::{marker::PhantomData, thread, time::Duration};

struct Open;
struct Closed;
struct Stopped(String);

struct Gate<S> {
    _state_maker: PhantomData<S>,
}

impl Gate<Closed> {
    fn open(self) -> Result<Gate<Open>, Gate<Stopped>> {
        move_gate(Open)
    }
}

impl Gate<Open> {
    fn close(self) -> Result<Gate<Closed>, Gate<Stopped>> {
        move_gate(Closed)
    }
}

fn move_gate<S>(_: S) -> Result<Gate<S>, Gate<Stopped>> {
    let mut rng = rand::thread_rng();
    let random_outcome_number = rng.gen_range(0..=10);

    match random_outcome_number {
        0..=3 => Err(Gate {
            _state_maker: PhantomData,
        }),
        4..=10 => Ok(Gate {
            _state_maker: PhantomData,
        }),
        _ => panic!("Random number should be from 0 to 10."),
    }
}

pub fn es6() {
    let error_string = "\
The gate stopped...
👻👻👻
The struct 'Stopped' has a String field, which probably was suppose to explaing why it stopped. 
But I don't know how to get it using PhantomData. PhantomData does not store any data.
👻👻👻\
"
    .to_string();
    let gate: Gate<Closed> = Gate {
        _state_maker: PhantomData,
    };

    println!("Opening the gate...");
    thread::sleep(Duration::new(1, 0));

    match gate.open() {
        Ok(gate) => {
            println!("Gate is open!");
            thread::sleep(Duration::new(1, 0));
            println!("Closing the gate...");
            thread::sleep(Duration::new(1, 0));

            match gate.close() {
                Ok(_) => {
                    println!("Gate is closed!");
                }
                Err(_) => println!("{}", error_string),
            }
        }
        Err(_) => println!("{}", error_string),
    }
}
