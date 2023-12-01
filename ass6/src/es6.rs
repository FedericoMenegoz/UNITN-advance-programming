use std::{cell::RefCell, rc::Rc};

struct EntagledBit {
    state: Rc<RefCell<bool>>
}
impl Default for EntagledBit {
    fn default() -> Self {
        EntagledBit { state: Rc::new(RefCell::new(false)) }
    }
}
impl EntagledBit {
    fn set(&self) {
        *self.state.borrow_mut() = true;
    }

    fn reset(&self) {
        *self.state.borrow_mut() = false;
    }

    fn get(&self) -> bool{
        *self.state.borrow()
    }

    fn entangle_with(&self, other: &mut Self) {
        other.state = Rc::clone(&self.state);
    }
}

pub fn es6() {
    let mut b1 = EntagledBit::default();
    let mut b2 = EntagledBit::default();
    let mut b3 = EntagledBit::default();

    // b1 = 0
    // b2 = 0
    // b3 = 0
    
    b1.set();
    b1.entangle_with(&mut b2);
    assert_eq!(b1.get(), b2.get());
    
    // b1 = 1
    // b2 = 1
    // b3 = 0
    
    b2.reset();
    assert_eq!(b1.get(), b2.get());
    
    // b1 = 0
    // b2 = 0
    // b3 = 0
    
    b3.set();
    b3.entangle_with(&mut b1);
    assert_eq!(b1.get(), b3.get());
    
    // b1 = 1
    // b2 = 0
    // b3 = 1

    b2.entangle_with(&mut b3);
    assert_ne!(b1.get(), b3.get());
    assert_ne!(b1.get(), b2.get());
    
    // b1 = 1
    // b2 = 0
    // b3 = 0
}