use std::{ops::Deref, borrow::BorrowMut, fmt::Debug};

#[derive(Debug,Clone)]
struct Node<T:Debug> where T:Clone{
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
    val: Option<T>
}

trait DublyListOps<T:Debug> where T:Clone{
    fn insert(&mut self, val:T);
    fn find(&mut self, val:T)-> &Option<Box<Node<T>>>;
    fn delete(&mut self, val:T);

    fn print(&mut self);
}

struct DublyLinkList<T:Debug> where T:Clone{
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>
}


impl<T:Debug> DublyLinkList<T>where T:Clone{
    fn new()->Self{
        DublyLinkList{
            head: None,
            tail:None
        }
    }
}

impl<T:Debug + std::cmp::PartialEq> DublyListOps<T> for DublyLinkList<T> where T:Clone{
    fn insert(&mut self, val: T) {
        let n = Node {
                     next: None,
                     prev: None,
                     val: Some(val)
        };
        let mut curr_node= &mut self.head;
        while let Some(ref mut node) = *curr_node{
            // println!("{:?}",node);
            curr_node = &mut node.next;
        }
        *curr_node = Some(Box::new(n));
    }

    fn find(&mut self, val: T) -> &Option<Box<Node<T>>> {
        let mut curr_node= &mut self.head;
        println!("Searching");
        while let Some(ref mut node) = *curr_node{
            // println!("{:?}",node);
            if node.val.clone().unwrap() ==val{
               Some(node)
            }
            curr_node = &mut node.next;
        }
        
    }
    fn delete(&mut self, val: T) {
        todo!()
    }

    fn print(&mut self) {        
        let mut pointer = &self.head;
        println!("Printing..");
        loop{
            match pointer.as_ref(){            
                Some(p) => {
                    println!("{:?}",p.val);
                    pointer= &p.next;
                },
                None => {
                    println!("--End--");
                    break;
                },
            }
        }

   
    }
}


fn main() {
    println!("Hello, world!");
    let mut dll =DublyLinkList::<i32>::new();
    dll.insert(1);
    dll.insert(2);
    dll.insert(3);

    dll.find(3);

    dll.print();

}


#[cfg(test)]
mod tests{

}
