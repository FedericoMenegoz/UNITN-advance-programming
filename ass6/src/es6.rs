use std::{rc::Rc, cell::RefCell};
#[derive(PartialEq, Clone, Copy)]
enum State {
    One,
    Zero,
}
type Node = Option<Rc<RefCell<EntangledBit>>>;
struct EntangledBit {
    val: State,
    next: Node,
    prev: Node
}

impl Default for EntangledBit {
    
    fn default() -> Self {
        EntangledBit { val: State::Zero, next: None, prev: None }
    }
}

impl EntangledBit {

    fn set(&mut self) {
       if self.get() == State::Zero { self.set_state(State::One); }
    }

    fn reset(&mut self) {
        if self.get() == State::One { self.set_state(State::Zero); }

     }

    fn set_state(&mut self, state:State) {
        self.val = state;
        self.next.as_ref()
            .map(|next| {next
                .borrow_mut().set()});
    }

    fn get(&self) -> State {
        self.val
    }



    fn entagled_with(&mut self, other: &mut EntangledBit) {
        match &self.next {
            Some(node) => {
                node.borrow_mut().entagled_with(other)
            },
            None => {

                match other.prev.take() {
                    Some(node) => node.borrow_mut().next = std::mem::replace(&mut other.next, None),
                    None => {}
                }

                self.next = Some(Rc::clone());

            }
        }

    }
        

        
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic6() {
        let bit_1 = EntangledBit::default();
        let bit_2 = EntangledBit::default();
        let bit_3 = EntangledBit::default();
        let bit_4 = EntangledBit::default();

        bit_
    }
}