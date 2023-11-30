use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
    rc::Rc,
};
type NodeRef<T> = Rc<Node<T>>;
type Adj<T> = Vec<Rc<Node<T>>>;
struct Node<T> {
    val: T,
    adj: Adj<T>,
}

impl<T: Hash + PartialEq + Eq> Node<T> {
    fn new(val: T, adj: Adj<T>) -> Self {
        Node { val, adj }
    }

    fn get_value(&self) -> &T {
        &self.val
    }
}

impl<T: Hash + PartialEq + Eq + Display + Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let adj_values = self
            .adj
            .iter()
            .map(|val| val.get_value())
            .collect::<Vec<&T>>();
        write!(
            f,
            "[value: {}, adjacents: \"{:?}\"]",
            self.get_value(),
            adj_values
        )
    }
}
struct Graph<T> {
    nodes: Adj<T>,
}

impl<T: Hash + PartialEq + Eq + Display + Debug + Clone> Graph<T> {
    fn new(nodes: Adj<T>) -> Self {
        Graph { nodes }
    }

    fn dfs(&self, start_node: NodeRef<T>) -> Adj<T> {
        let mut visited = HashSet::new();
        let mut ret_vec = Vec::new();
        ret_vec.push(start_node.clone());
        self.dfs_helper(start_node.clone(), &mut visited, &mut ret_vec);
        ret_vec
    }

    fn dfs_helper(&self, start_node: NodeRef<T>, visited: &mut HashSet<T>, result: &mut Adj<T>) {
        start_node.adj.iter().for_each(|node| {
            if !visited.contains(node.get_value()) {
                visited.insert(node.get_value().clone());
                result.push(node.clone());
                self.dfs_helper(node.clone(), visited, result);
            }
        });
    }
}

pub fn es7() {
    let n1 = Rc::new(Node::new(1, vec![]));
    let n2 = Rc::new(Node::new(2, vec![n1.clone()]));
    let n3 = Rc::new(Node::new(3, vec![]));
    let n4 = Rc::new(Node::new(4, vec![n1.clone(), n3.clone()]));
    let n5 = Rc::new(Node::new(5, vec![n2.clone(), n4.clone()]));
    let n6 = Rc::new(Node::new(6, vec![n5.clone(), n4.clone()]));
    let n7 = Rc::new(Node::new(7, vec![n2.clone(), n4.clone()]));

    let graph = Graph::new(vec![
        n1.clone(),
        n2.clone(),
        n3.clone(),
        n4.clone(),
        n5.clone(),
        n6.clone(),
        n7.clone(),
    ]);

    let mut paths: Vec<Vec<NodeRef<i32>>> = vec![];
    for n in graph.nodes.iter() {
        paths.push(graph.dfs(n.clone()))
    }

    paths.iter().for_each(|path| {
        println!("{:?}", path);
    });
}
