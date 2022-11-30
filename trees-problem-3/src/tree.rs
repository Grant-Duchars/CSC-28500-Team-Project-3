struct Node {
    value: String,
    left_children: Vec<Box<Node>>,
    right_children: Vec<Box<Node>>,
}

impl Node {
    fn new(value: String) -> Self {
        Node {
            value,
            left_children: Vec::new(),
            right_children: Vec::new(),
        }
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

pub struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    /// Construct a new empty tree
    pub fn new() -> Self {
        Tree { root: None }
    }

    /// Insert an edge into the tree. Returns false if nothing was inserted
    pub fn insert(&mut self, edge: (String, String, bool)) -> bool {
        match &mut self.root {
            Some(node) => Tree::insert_recursive(node, &edge),
            None => {
                let mut new_node = Node::new(edge.0);
                if edge.2 {
                    new_node.left_children = vec![Node::new(edge.1).into()];
                } else {
                    new_node.right_children = vec![Node::new(edge.1).into()];
                }
                self.root = new_node.into();
                true
            }
        }
    }

    fn insert_recursive(node: &mut Box<Node>, edge: &(String, String, bool)) -> bool {
        if node.value == edge.0 {
            if edge.2 {
                // If child is left of parent
                node.left_children.push(Node::new(edge.1.to_owned()).into());
            } else {
                // child is right of parent
                node.right_children
                    .push(Node::new(edge.1.to_owned()).into());
            }
            return true;
        } else {
            let mut inserted = false;
            for i in 0..node.left_children.len() {
                if !inserted {
                    inserted = Self::insert_recursive(&mut node.left_children[i], &edge);
                }
            }
            for i in 0..node.right_children.len() {
                if !inserted {
                    inserted = Self::insert_recursive(&mut node.right_children[i], &edge);
                }
            }
            inserted
        }
    }

    /// Print the tree using preorder traversal
    pub fn print_preorder(&self) {
        match &self.root {
            Some(node) => {
                Self::preorder_recursive(node);
                println!();
            }
            None => println!("Empty tree"),
        }
    }

    /// Print the tree using inorder traversal
    pub fn print_inorder(&self) {
        match &self.root {
            Some(node) => {
                Self::inorder_recursive(node);
                println!();
            }
            None => println!("Empty tree"),
        }
    }

    /// Print the tree using postorder traversal
    pub fn print_postorder(&self) {
        match &self.root {
            Some(node) => {
                Self::postorder_recursive(node);
                println!();
            }
            None => println!("Empty tree"),
        }
    }

    fn preorder_recursive(node: &Box<Node>) {
        print!("{} ", node.value);
        for child in node.left_children.iter() {
            Self::preorder_recursive(child);
        }
        for child in node.right_children.iter() {
            Self::preorder_recursive(child);
        }
    }

    fn inorder_recursive(node: &Box<Node>) {
        for child in node.left_children.iter() {
            Self::inorder_recursive(child);
        }
        print!("{} ", node.value);
        for child in node.right_children.iter() {
            Self::inorder_recursive(child);
        }
    }

    fn postorder_recursive(node: &Box<Node>) {
        for child in node.left_children.iter() {
            Self::postorder_recursive(child);
        }
        for child in node.right_children.iter() {
            Self::postorder_recursive(child);
        }
        print!("{} ", node.value);
    }
}
