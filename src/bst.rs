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
    
    /* returns a string containing the nodes in self as given by an in order traversal */
    pub fn to_string(&self) -> String {
        println!("left: {}", &self.root.as_ref().unwrap().left.as_ref().unwrap().val);
        println!("root: {}", &self.root.as_ref().unwrap().val);
        println!("right: {}", &self.root.as_ref().unwrap().right.as_ref().unwrap().val);
        Self::in_order(&self.root)
    }

    /* private function that takes a node r and a string s containing the nodes visited so 
    far. Performs an in order traversal on the tree rooted at r and returns a string 
    containing the nodes in the order they are visited */
    fn in_order(r: &Link<i32>) -> String {
        if r.is_none() {
            //println!("is none: {}", s);
            String::new()
        } else {
            let mut s = String::new();
            s += &Self::in_order(&r.as_ref().unwrap().left);
            s += r.as_ref().unwrap().val.to_string().as_str() + " ";
            s += &Self::in_order(&r.as_ref().unwrap().right);
            // s = format!("{}{}", s, Self::in_order(&r.as_ref().unwrap().left, s.clone()));
            // s = format!("{}{}", s, r.as_ref().unwrap().val);
            // s = format!("{}{}", s, Self::in_order(&r.as_ref().unwrap().right, s.clone()));
            //println!("this shit: {}", s);
            format!("{} ", s)
        }
    }
}

#[cfg(test)]
mod test {
    use super::BST;
    #[test]
    fn basic () {
        let tree:BST<i32> = BST::new(); 

        //tree.add(4);
        //assert_eq!(tree.as_str(), String::from("4"));

    }

}