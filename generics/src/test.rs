 fn removeAt(&self, pos: u32)  {
        let mut iterator = 0_u32;
        let mut current_node = &self.head;
        while iterator <= pos {
            if let Some(node) = current_node {
                if iterator == pos {
                    let mut required = Some(node);
                    required.unwrap().next = None;
                    //required
                }
                if iterator == pos -1 {
                    let mut required = Some(node);
                    required.unwrap().next = required.unwrap().next.unwrap().next;
                    required;
                }
               
            iterator = iterator - 1;        
        }
        None;
    }
}


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
