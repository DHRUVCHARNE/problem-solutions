use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc,Weak};

type Link=Option<Rc<RefCell<Node>>>;

struct Node {
    key:i32,
    val:i32,
    prev:Option<Weak<RefCell<Node>>>,
    next:Link
}

pub struct LRUCache {
cap:usize,
map:HashMap<i32,Rc<RefCell<Node>>>,
head:Link,
tail:Link,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        LRUCache {
            cap:capacity as usize,
            map:HashMap::new(),
            head:None,
            tail:None,
        }
        
    }

    fn remove(&mut self,node:Rc<RefCell<Node>>){
        let prev= node.borrow_mut().prev.take().and_then(|w| w.upgrade());
        let next = node.borrow_mut().next.take();

        match &prev {
            Some(p) => p.borrow_mut().next=next.clone(),
            None => self.head = next.clone(),
        }

        match &next {
            Some(n) => n.borrow_mut().prev = prev.map(|p| Rc::downgrade(&p)),
            None => self.tail= prev,
        }
    }

    fn insert_front(&mut self,node : Rc<RefCell<Node>>){
        node.borrow_mut().prev=None;
        node.borrow_mut().next = self.head.clone();
        if let Some(old_head) = &self.head {
            old_head.borrow_mut().prev = Some(Rc::downgrade(&node));
        }

        self.head = Some(node.clone());
        if self.tail.is_none(){
            self.tail=Some(node);
        }
    }
    
    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key).cloned() {
            self.remove(node.clone());
            self.insert_front(node.clone());
            node.borrow().val
        } else {
            -1
        }
    }
    
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node)=self.map.get(&key).cloned() {
            node.borrow_mut().val=value;
            self.remove(node.clone());
            self.insert_front(node);
            return;
        }
        if self.map.len() == self.cap {
            if let Some(lru)=self.tail.clone(){
                let old_key=lru.borrow().key;
                self.remove(lru);
                self.map.remove(&old_key);
            }
        }

        let new_node = Rc::new(RefCell::new(Node {
            key,
            val:value,
            prev:None,
            next:None,
        }));
        self.insert_front(new_node.clone());
        self.map.insert(key,new_node);
    }
}