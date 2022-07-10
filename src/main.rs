mod bst;
use crate::bst::BST;
fn main() {
    //println!("Hello world!");
    let mut tree:BST<i32> = BST::new(); 

    tree.add(4);
    tree.add(1);
    tree.add(5);
    tree.add(12);
    tree.add(-12);
    tree.add(3);
    tree.add(4);
    tree.add(8);

    tree.print();

}




