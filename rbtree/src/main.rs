use std::cmp::Ordering;

//type Pointer<T> = Option<Rc<RefCell<T>>>;

/* BST node without parent pointer.
   If we introduce a parent pointer, we would need Rc<RefCell<T>> due to shared pointers
*/ 
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    key: i32,
}

impl Node {
    fn new(key: i32) -> Node {
        Node {
            left: None,
            right: None,
            key,
        }
    }

    fn insert(&mut self, key: i32) {
        todo!()
    }
}


struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    fn new() -> BST {
        BST {
            root: None
        }
    }

    fn contains(&self, key: i32) -> bool {
        let mut current = &self.root;
        
        while let Some(node) = current {
            match key.cmp(&node.key) {
                Ordering::Greater => current = &node.right,
                Ordering::Less => current = &node.left,
                Ordering::Equal => return true,
            }
        }
        false
    }

    fn insert(&mut self, key: i32) {
        match &mut self.root {
            None => self.root = Some(Box::new(Node::new(key))),
            Some(node) => node.insert(key),
        }
    }
}


fn main() {
    let bst: BST = BST::new();

    println!("Hello, world!");
}
