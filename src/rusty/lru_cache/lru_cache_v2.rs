use std::collections::HashMap;

#[derive(Clone)]
pub struct Node {
    key:i32,
    val:i32,
    prev:Option<usize>,
    next:Option<usize>,
}

pub struct LRUCache {
    cap:usize,
    map:HashMap<i32,usize>,
    nodes:Vec<Node>,
    head:Option<usize>,
    tail:Option<usize>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    pub fn new(capacity: i32) -> Self {
        Self {
            cap:capacity as usize,
            map:HashMap::new(),
            nodes:Vec::new(),
            head:None,
            tail:None,
        }
    }
    
    pub fn move_to_front(&mut self,idx:usize){
        if Some(idx)==self.head {
            return;
        }
        let (prev,next) = {
            let node = &self.nodes[idx];
            (node.prev,node.next)
        };
        if let Some(p) = prev {
            self.nodes[p].next=next;
        }
        if let Some(n) = next {
            self.nodes[n].prev=prev;
        }
        if Some(idx) == self.tail {self.tail=prev;}
        self.nodes[idx].prev=None;
        self.nodes[idx].next=self.head;
        if let Some(h) = self.head {
            self.nodes[h].prev=Some(idx);
        }
        self.head=Some(idx);
        if self.tail.is_none() {self.tail=Some(idx);}
    }
    pub fn pop_tail(&mut self){
        if let Some(t)=self.tail {
            let key=self.nodes[t].key;
            self.map.remove(&key);
            self.tail=self.nodes[t].prev;

            if let Some(new_tail) =self.tail {
                self.nodes[new_tail].next=None;
            } else {
                self.head=None;
            }
        }
    }
    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&idx) = self.map.get(&key) {
            self.move_to_front(idx);
            self.nodes[idx].val
        } else {
            -1
        }
    }
    
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&idx) =self.map.get(&key) {
            self.nodes[idx].val =value;
            self.move_to_front(idx);
            return;
        }
        if self.map.len() == self.cap {
            self.pop_tail();
        }
        let idx = self.nodes.len();
        self.nodes.push(Node {
            key,
            val:value,
            prev:None,
            next:None,
        });
        self.map.insert(key,idx);
        self.move_to_front(idx);

    }
}
