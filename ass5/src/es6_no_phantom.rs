use std::{thread, time::Duration};

use rand::Rng;

struct Open;
struct Closed;
#[derive(Debug)]
struct Stopped {
    reason: String,
}
struct Gate<S> {
    state_maker: S,
}
impl Open {
    fn close(self) -> Result<Gate<Closed>, Gate<Stopped>> {
        move_gate(Closed)
    }
}
impl Closed {
    fn open(self) -> Result<Gate<Open>, Gate<Stopped>> {
        move_gate(Open)
    }
}

fn move_gate<S>(s: S) -> Result<Gate<S>, Gate<Stopped>> {
    let mut rng = rand::thread_rng();
    let random_outcome_number = rng.gen_range(0..=10);

    match random_outcome_number {
        0 => Err(Gate {
            state_maker: Stopped {
                reason: "Photocell was triggered.".to_string(),
            },
        }),
        1 => Err(Gate {
            state_maker: Stopped {
                reason: "Found resistance.".to_string(),
            },
        }),
        2 => Err(Gate {
            state_maker: Stopped {
                reason: "Power out. Did you pay the bill?!".to_string(),
            },
        }),
        3 => Err(Gate {
            state_maker: Stopped {
                reason: "Gate's on strike!".to_string(),
            },
        }),
        4..=10 => Ok(Gate { state_maker: s }),
        _ => panic!("Random number should be from 0 to 10."),
    }
}

pub fn es6() {
    let gate = Gate {
        state_maker: Closed,
    };
    println!("Opening the gate...");
    thread::sleep(Duration::new(1, 0));

    match gate.state_maker.open() {
        Ok(gate) => {
            println!("Gate is open!");
            thread::sleep(Duration::new(1, 0));
            println!("Closing the gate...");
            thread::sleep(Duration::new(1, 0));

            match gate.state_maker.close() {
                Ok(_) => {
                    println!("Gate is closed!");
                }
                Err(t) => println!("Gate stopped: {:?}", t.state_maker.reason),
            }
        }
        Err(t) => println!("Gate stopped: {:?}", t.state_maker.reason),
    }
}
