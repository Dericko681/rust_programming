use adt_list::ADTList;
use linked_list::LinkedList;
//od traits;
mod max_items;
mod linked_list;
mod adt_list;
fn main() {
    let mut list: LinkedList<i32> = LinkedList::<i32>::new();
    println!("newly created list {:?}", list);
    list.insert(237);
    println!("mutated list: {:?}", list);

    list.insert(25);
    list.insert(24);
    list.insert(26);
    list.insert(250);
    let pos = 3;
    println!("element at position {pos} is {:?}", list.get(pos));
    println!("updated list {:?}", list);
   }

 