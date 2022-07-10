pub struct BST<i32> {
    root: Link<i32>
}

type Link<i32> = Option<Box<Node<i32>>>;

struct Node<i32> {
    val: i32,
    left: Link<i32>,
    right: Link<i32>,
}

impl BST<i32> {
    pub fn new() -> Self {
        BST { root: None }
    }

    // add a new node to the tree
    pub fn add(&mut self, val: i32) { 
        let new_node = Box::new(Node { // initialize node
            val: val, 
            left: None,
            right: None,
        });
        Self::place_node(&mut self.root, new_node); // place node in proper spot on tree
    }

    /* private method to figure out the proper spot for the given node n on a tree
    rooted at r and then put it there */
    fn place_node(r: &mut Link<i32>, n: Box<Node<i32>>) {
        if r.is_none() { 
            *r = Some(n);
        } else if n.val < r.as_ref().unwrap().val { 
            Self::place_node(&mut r.as_mut().unwrap().left, n);
        } else if n.val > r.as_ref().unwrap().val {
            Self::place_node(&mut r.as_mut().unwrap().right, n);
        }
    }
    
    /* prints a string containing the nodes in self as given by an in order traversal */
    pub fn print(&self) {
        println!("{}", Self::in_order(&self.root));
    }

    /* returns a string containing the nodes in self as given by an in order traversal */
    pub fn to_string(&self) -> String {
        Self::in_order(&self.root)
    }

    /* private function that takes a node r and a string s containing the nodes visited so 
    far. Performs an in order traversal on the tree rooted at r and returns a string 
    containing the nodes in the order they are visited */
    fn in_order(r: &Link<i32>) -> String {
        let mut s = String::new();
        if r.is_none() {
            s
        } else {
            s += &Self::in_order(&r.as_ref().unwrap().left);
            s += r.as_ref().unwrap().val.to_string().as_str(); s.push(' ');
            s += &Self::in_order(&r.as_ref().unwrap().right);
            s
        }
    }
}

#[cfg(test)]
mod test {
    use super::BST;
    #[test]
    fn tests () {
        let mut tree:BST<i32> = BST::new(); 

        // check that empty tree behaves correctly
        assert_eq!(tree.to_string(), String::from(""));

        // check that tree containing only the root behaves correctly
        tree.add(4);
        assert_eq!(tree.to_string(), String::from("4 "));

        // add left child
        tree.add(1);
        assert_eq!(tree.to_string(), String::from("1 4 ")); 

        // add right child
        tree.add(6);
        assert_eq!(tree.to_string(), String::from("1 4 6 "));

        // try adding value already in the tree 
        tree.add(1); 
        assert_eq!(tree.to_string(), String::from("1 4 6 "));

        // add more children 
        tree.add(2); 
        tree.add(10);
        tree.add(8); 
        tree.add(3); 
        tree.add(5); 
        tree.add(7); 
        tree.add(9); 
        assert_eq!(tree.to_string(), String::from("1 2 3 4 5 6 7 8 9 10 "))
    }

}