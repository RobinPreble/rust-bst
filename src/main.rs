mod bst;
use crate::bst::BST;
fn main() {
    //println!("Hello world!");
    let mut tree:BST<i32> = BST::new(); 

    tree.add(4);
    tree.add(1);
    tree.add(5);

}




