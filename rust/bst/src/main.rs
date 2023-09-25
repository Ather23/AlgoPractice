#[derive(Debug, Clone)]
struct Node<T:Ord> where T:Clone{
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

trait Bst<T:Ord> where T:Clone{
    fn insert(&mut self, val:T) -> Option<Box<Node<T>>>;
    fn insert_node(&mut self, node_to_insert:&Node<T>, node: Option<Box<Node<T>>>)->Option<Box<Node<T>>>;
    fn get_height(&mut self)->usize;
    fn traverse_for_height(&mut self,node:Option<Box<Node<T>>>)->usize;
    fn print();
}


#[derive(Default)]
struct Tree<T:Ord> where T:Clone{
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> Bst<T> for Tree<T> where T:Clone {
    fn insert(&mut self,val: T) -> Option<Box<Node<T>>> {
        let new_node = Node{
            val,
            left: None,
            right: None
        };
       self.root =  self.insert_node(&new_node, self.root.to_owned());
       self.root.to_owned()
    }

    fn insert_node(&mut self, node_to_insert:&Node<T>, tree_node: Option<Box<Node<T>>>)-> Option<Box<Node<T>>>{
        match tree_node {
            Some(mut n)=> {
                if node_to_insert.val < n.val{
                    println!("Checking left");
                    n.left =  self.insert_node(node_to_insert,n.left);
                }
                else{
                    println!("Checking right");
                    n.right = self.insert_node(node_to_insert,n.right);
                }
                Some(n)
            },
            None=> {
              Some(Box::new(node_to_insert.to_owned()))
            }
        }
    }

    fn traverse_for_height(&mut self, node:Option<Box<Node<T>>> )->usize{
        match node{
            Some(n)=>{
                if n.left.is_some(){
                    return self.traverse_for_height(n.left) +1
                }

                if n.right.is_some(){
                    return self.traverse_for_height(n.right)+1
                }

            },
            None=> {
                return 0
            }
        }

        return 1
    }

    fn get_height(&mut self)->usize{

        let lh = self.traverse_for_height(self.root.to_owned().unwrap().left.to_owned());   
        let rh = self.traverse_for_height(self.root.to_owned().unwrap().right.to_owned());

        
         if lh > rh {
            return lh
         }
         else {            
            return rh
         }
    }

    fn print(){
        todo!()
    }

}

fn main() {
    let mut tree: Tree<i32> = Tree::default();
    tree.insert(50);
    tree.insert(100);
    tree.insert(25);
    tree.insert(10);
    tree.insert(2);
    tree.insert(0);

    let h = tree.get_height();
    println!("Height is {}",h);
}

#[cfg(test)]
mod tests{
    use crate::{Tree, Bst};

    #[test]
    fn test_for_height(){
        let mut tree: Tree<i32> = Tree::default();
        tree.insert(50);
        tree.insert(100);
        tree.insert(25);

        let h = tree.get_height();
        println!("Height is {}",h);

        assert_eq!(h, 1);
    }

}
