struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct BST {
    root: Option<Box<Node>>,
}

impl  BST {
    fn new() -> Self {
        BST { root: None } 
    }

    fn insert(&mut self, value: i32) {

        let mut current = &mut self.root;
        
        loop {
            match current {
                None => {
                    *current = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }));
                    return;
                }

                Some(node) => {
                    if value < node.value {
                        current = &mut node.left;
                    } else if value > node.value {
                        current = &mut node.right;
                    } else {
                        return;
                    }
                }
            } 
        }
    }

    fn search(&self, value: i32) -> bool {
        let mut current = &self.root;

        while let Some(node) = current {
            if value == node.value {
                return true;
            } else if value < node.value {
                current = &node.left;
            } else {
                current = &node.right;
            }
            
        }
        false
    }

}

fn main() {
    let mut bst = BST::new();
    
    println!("Inserindo o número 50");
    bst.insert(50);
    println!("Inserindo o número 95");
    bst.insert(95);
    println!("Inserindo o número 25");
    bst.insert(25);

    println!("Procurando o 41 {}", bst.search(41));
    println!("Procurando o 25 {}", bst.search(25));
}
