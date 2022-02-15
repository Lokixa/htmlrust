#![allow(dead_code)]
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
pub struct Html {
    pub tag: String,
    pub children: Option<Vec<Rc<RefCell<Html>>>>,
    pub parent: Option<Weak<RefCell<Html>>>,
    pub content: Option<String>,
    pub options: Option<HashMap<String, String>>,
}

impl Html {
    pub fn new(tag: &str) -> Rc<RefCell<Html>> {
        Rc::new(RefCell::new(Html {
            tag: tag.to_owned(),
            children: None,
            parent: None,
            content: None,
            options: None,
        }))
    }
    pub fn add_child(node: &mut Rc<RefCell<Html>>, child: Rc<RefCell<Html>>) {
        let mut borrowed_node = node.borrow_mut();
        if let None = borrowed_node.children {
            borrowed_node.children = Some(Vec::new());
        }
        let children = borrowed_node.children.as_mut().unwrap();
        {
            let mut child = child.borrow_mut();
            child.parent = Some(Rc::downgrade(&node));
        }
        children.push(child);
    }
    pub fn get_child(&self, tag: &str) -> Option<Rc<RefCell<Html>>> {
        if let Some(children) = self.children.as_ref() {
            for child in children {
                let borrowed_child = child.borrow();
                if borrowed_child.tag == tag {
                    return Some(Rc::clone(child));
                }
            }
        }
        return None;
    }
    pub fn get_child_ref(&self, tag: &str) -> Option<Ref<Html>> {
        if let Some(children) = self.children.as_ref() {
            for child in children {
                let child = child.borrow();
                if child.tag == tag {
                    return Some(child);
                }
            }
        }
        return None;
    }
    pub fn get_child_mut(&mut self, tag: &str) -> Option<RefMut<Html>> {
        if let Some(children) = self.children.as_ref() {
            for child in children {
                let child = child.borrow_mut();
                if child.tag == tag {
                    return Some(child);
                }
            }
        }
        return None;
    }
    pub fn remove_first(&mut self, tag: &str) -> Result<(), &str> {
        if let Some(children) = self.children.as_mut() {
            let mut index: isize = -1;
            for i in 0..children.len() {
                if children.get(i).unwrap().borrow().tag == tag {
                    index = i as isize;
                    break;
                }
            }
            if index > -1 {
                children.remove(index as usize);
                return Ok(());
            }
        }

        return Err("No such child");
    }
}
