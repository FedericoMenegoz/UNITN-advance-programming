// For the implementation of the iterator see here: https://stackoverflow.com/questions/43833588/implement-intoiterator-for-binary-tree

use std::{cmp::Ordering, fmt::Debug};
#[derive(Debug, PartialEq)]
pub struct TreeNode<T: PartialOrd + Clone + Ord> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

pub struct RefNodeIterator<'a, T: PartialOrd + Clone + Ord + 'a> {
    stack: Vec<&'a TreeNode<T>>,
    next: Option<&'a T>,
}

impl<'a, T: PartialOrd + Clone + Ord + 'a> IntoIterator for &'a TreeNode<T> {
    type Item = &'a T;
    type IntoIter = RefNodeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut stack = Vec::new();

        let smallest = pop_smallest_ref(self, &mut stack);

        RefNodeIterator {
            stack,
            next: Some(smallest),
        }
    }
}

impl<'a, T: PartialOrd + Clone + Ord + 'a> Iterator for RefNodeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(next) = self.next.take() {
            return Some(next);
        }

        if let Some(node) = self.stack.pop() {
            if let Some(ref right) = node.right {
                self.stack.push(right);
            }
            return Some(&node.value);
        }

        None
    }
}

fn pop_smallest_ref<'a, T>(node: &'a TreeNode<T>, stack: &mut Vec<&'a TreeNode<T>>) -> &'a T
where
    T: PartialOrd + Clone + Ord + 'a,
{
    if let Some(ref left) = node.left {
        stack.push(node);
        return pop_smallest_ref(left, stack);
    }

    if let Some(ref right) = node.right {
        stack.push(right);
    }

    &node.value
}
impl<T: PartialOrd + Clone + Ord + std::fmt::Debug> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        match self.value.partial_cmp(&value).expect("Always comparable!") {
            Ordering::Greater | Ordering::Equal => match self.left {
                None => {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
                Some(_) => self.left.as_mut().unwrap().insert(value),
            },
            Ordering::Less => match self.right {
                None => {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
                Some(_) => self.right.as_mut().unwrap().insert(value),
            },
        }
    }

    fn from_vec(mut vector: Vec<T>) -> Option<Self> {
        vector.sort();
        TreeNode::insert_from_ordered_vec(&vector[..]).map(|tree| *tree)
    }

    fn insert_from_ordered_vec(vector: &[T]) -> Option<Box<Self>> {
        match vector.len() {
            0 => None,
            1 => Some(Box::new(TreeNode::new(vector[0].clone()))),
            _ => {
                let mid = (vector.len() as f32 / 2.0).ceil() as usize;
                let (first_half, second_half) = vector.split_at(mid);

                let mut root = TreeNode::new(
                    first_half
                        .last()
                        .expect("Should at least have one element!")
                        .clone(),
                );

                root.left = TreeNode::insert_from_ordered_vec(&first_half[..mid - 1]);
                root.right = TreeNode::insert_from_ordered_vec(second_half);

                Some(Box::new(root))
            }
        }
    }
}
pub fn print_tree<T: PartialOrd + Clone + Ord + std::fmt::Debug>(root: &TreeNode<T>, width: usize) {
    if let Some(leaf) = root.right.as_ref() {
        print_tree(leaf, width + 3)
    }

    println!("{:width$?}", &root.value);

    if let Some(leaf) = root.left.as_ref() {
        print_tree(leaf, width + 3)
    }
}

pub fn es1() {
    let mut root = TreeNode::from_vec(vec![10, 4, 3, 1, 6, 7]).unwrap();
    root.insert(50);
    root.insert(-3);
    print_tree(&root, 0);
    println!("{}",root.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}
#[cfg(test)]
mod tree_node_tests {
    use super::TreeNode;

    #[test]
    fn new_with_i32_and_no_leafs() {
        let root = TreeNode::new(5);
        assert_eq!(root.value, 5);
        assert_eq!(root.left, None);
        assert_eq!(root.right, None);
    }

    #[test]
    fn insert_from_vec_not_in_order() {
        let tree = TreeNode::from_vec(vec![56, 23, 76, 1]).unwrap();
        let ordered_vector = vec![1, 23, 56, 76];
        tree.into_iter()
            .zip(ordered_vector)
            .for_each(|(a, b)| assert_eq!(*a, b));
    }

    #[test]
    fn insert_123_from_vec() {
        let three_numbers = vec![1, 2, 3];
        let root = TreeNode::from_vec(three_numbers);
        assert_eq!(root.as_ref().unwrap().value, 2);
        assert_eq!(root.as_ref().unwrap().left.as_ref().unwrap().value, 1);
        assert_eq!(root.as_ref().unwrap().right.as_ref().unwrap().value, 3);
    }
}
