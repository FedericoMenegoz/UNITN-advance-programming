use std::marker::PhantomData;

trait CompileTimeNode {
    type LeftType:CompileTimeNode;
    type RightType:CompileTimeNode;
    fn is_none() -> bool;
}

struct NullNode {}

struct Node<L:CompileTimeNode, R:CompileTimeNode> {
    left: PhantomData<L>,
    right: PhantomData<R>,
}

impl CompileTimeNode for NullNode {
    type LeftType = NullNode;

    type RightType = NullNode;

    fn is_none() -> bool {
        true
    }
}

impl<L:CompileTimeNode, R:CompileTimeNode> CompileTimeNode for Node<L, R> {
    type LeftType = L;

    type RightType = R;

    fn is_none() -> bool {
        false
    }
}

fn count_nodes<T:CompileTimeNode>() -> usize {
    if T::is_none() {
        0
    } else {
        1 + count_nodes::<T::LeftType>() + count_nodes::<T::RightType>()
    }
}

pub fn es5() {
    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic5() {
        assert_eq!(count_nodes::<Node::<NullNode, NullNode>>(), 1);
        assert_eq!(count_nodes::<Node::<Node::<Node<NullNode, NullNode>, Node<NullNode, NullNode>>, NullNode>>(), 4);
    }
}