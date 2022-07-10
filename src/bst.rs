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

    pub fn add(&mut self, val: i32) { 
        let new_node = Box::new(Node {
            val: val, 
            left: None,
            right: None,
        });
        Self::place_node(&mut self.root, new_node);
    }
    
    fn place_node(root: &mut Link<i32>, new_node: Box<Node<i32>>) {
        if root.is_none() {
            *root = Some(new_node);
        } else if root.as_ref().unwrap().val > new_node.val {
            Self::place_node(&mut root.as_mut().unwrap().left, new_node);
        } else if root.as_ref().unwrap().val < new_node.val {
            Self::place_node(&mut root.as_mut().unwrap().left, new_node);
        }
    }
    

    pub fn to_string(&self) -> String {
        (*Self::in_order(&self.root, String::new())).to_string()
    }

    fn in_order(root: &Link<i32>, mut s: String) -> String {
        if root.is_none() {
            println!("is none: {}", s);
            String::new()
        } else {
            s = format!("{} {} ", s, Self::in_order(&root.as_ref().unwrap().left, s.clone()));
            s = format!("{} {} ", s, root.as_ref().unwrap().val);
            s = format!("{} {} ", s, Self::in_order(&root.as_ref().unwrap().right, s.clone()));
            println!("this shit: {}", s);
            s
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