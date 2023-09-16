#[derive(Default,Debug)]
struct Node<T> {
    val:i32,
    left:Option<Box<Node<T>>>,
    right:Option<Box<Node<T>>>
}


#[derive(Default,Debug)]
struct Tree{
    root: Option<Box<Node<T>>>
}


impl Bst for Tree{
    
}


fn main() {
    println!("Hello, world!");
}
