use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T: Display> Node<T> {
    fn print_list_helper(&self) {
        println!("{} ", self.element);
        self.next
            .as_ref()
            .map(|next_node| next_node.borrow().print_list_helper());
    }
}

struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

struct ListIter<T> {
    list: List<T>,
}

impl<T: Display> Iterator for ListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}
impl<T: Display> IntoIterator for List<T> {
    type Item = T;

    type IntoIter = ListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter { list: self }
    }
}

impl<T: Display> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn push(&mut self, element: T) {
        // create the node to put in front of the list
        let new_node = Rc::new(RefCell::new(Node {
            element,
            next: None,
            prev: None,
        }));

        // if list is not empty
        if let Some(prev_head) = self.head.take() {
            // attach the head's list to the new node
            prev_head.borrow_mut().prev = Some(Rc::clone(&new_node));
            new_node.borrow_mut().next = Some(prev_head);

            //change the head to the new node
            self.head = Some(new_node)
        } else {
            // if list is empty head and tail should point to the new node
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        }

        // increase the size of 1
        self.size += 1;
    }

    // same as push just invert tail with head
    fn push_back(&mut self, element: T) {
        let new_node = Rc::new(RefCell::new(Node {
            element,
            next: None,
            prev: None,
        }));

        if let Some(prev_tail) = self.tail.take() {
            prev_tail.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(prev_tail);

            self.tail = Some(new_node)
        } else {
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        }
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        // map on a option will call the function only if Some() is found
        // it will return None otherwise
        // with self.head.take we are leaving self.head = None
        self.head.take().map(|prev_head| {
            // decrease the size
            self.size -= 1;

            // we need to check if there is more than one node
            match prev_head.borrow_mut().next.take() {
                // if there is another node we need to remove his prev
                // and make the head's list to point to it
                Some(node) => {
                    node.borrow_mut().prev = None;
                    self.head = Some(node);
                }
                // if no other node is in the list then we need to set to
                // None the tail as well
                None => {
                    self.tail.take();
                }
            }
            // to take the actual value
            // try unwrap will return a Ok with the inner value if
            // there is only one reference to it otherwise an Err
            Rc::try_unwrap(prev_head)
                // ok converts the Result in an Option trashing the Err
                .ok()
                .unwrap()
                // consume the RefCell returning the wrapped value
                .into_inner()
                .element
        })
    }

    // same as pop with tail <=> head
    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|prev_tail| {
            self.size -= 1;
            match prev_tail.borrow_mut().prev.take() {
                Some(node) => {
                    node.borrow_mut().next = None;
                    self.tail = Some(node);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(prev_tail).ok().unwrap().into_inner().element
        })
    }

    fn print_list(&self) {
        self.head.as_ref().map(|node| {
            node.borrow().print_list_helper();
        });
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element && self.next == other.next && self.prev == other.prev
    }
}

impl<T: PartialEq + Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.element)
    }
}
impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail == other.tail && self.size == other.size
    }
}

impl<T: Debug + Display> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("List")
            .field("head", &self.head)
            .field("tail", &self.tail)
            .field("size", &self.size)
            .finish()
    }
}

impl<T: Debug + Display> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.element)
    }
}

pub fn es2() {
    println!("Test 1:");
    es2_test1();
    println!("Test 2:");
    es2_test2();
    println!("Test 3:");
    es2_test3();
    println!("Test 4:");
    es2_test4();
    println!("Test 5:");
    es2_test5();
}
pub fn es2_test1() {
    let mut list: List<i32> = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.print_list();
}
pub fn es2_test2() {
    let mut list: List<i32> = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.print_list();
    debug_assert_eq!(list.size, 3);
    debug_assert_eq!(list.head.clone().unwrap().borrow().element, 3);
    debug_assert_eq!(list.tail.clone().unwrap().borrow().element, 1);
}

pub fn es2_test3() {
    let mut list: List<i32> = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    debug_assert_eq!(list.pop(), Some(3));
    list.print_list();
    debug_assert_eq!(list.pop(), Some(2));
    list.print_list();
    debug_assert_eq!(list.pop(), Some(1));
    list.print_list();
    debug_assert_eq!(list.pop(), None);
    list.print_list();
    debug_assert_eq!(list.size, 0);
    debug_assert_eq!(list.head, None);
    debug_assert_eq!(list.tail, None);
}

pub fn es2_test4() {
    let mut list: List<i32> = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    debug_assert_eq!(list.size, 3);
    println!("{}", list.size);
    list.print_list();
    debug_assert_eq!(list.head.clone().unwrap().borrow().element, 1);
    println!("{}", list.head.clone().unwrap().borrow().element);
    debug_assert_eq!(list.tail.clone().unwrap().borrow().element, 3);
    println!("{}", list.tail.clone().unwrap().borrow().element);
}

pub fn es2_test5() {
    let mut list: List<i32> = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.print_list();
    debug_assert_eq!(list.pop_back(), Some(3));
    list.print_list();
    debug_assert_eq!(list.pop_back(), Some(2));
    list.print_list();
    debug_assert_eq!(list.pop_back(), Some(1));
    list.print_list();
    debug_assert_eq!(list.pop_back(), None);
    debug_assert_eq!(list.size, 0);
    debug_assert_eq!(list.head, None);
    println!("{:?}", list.head);
    debug_assert_eq!(list.tail, None);
    println!("{:?}", list.tail);
}
