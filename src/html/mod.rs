#![allow(dead_code)]
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
struct Html {
    tag: String,
    children: Option<Vec<Rc<RefCell<Html>>>>,
    parent: Option<Weak<Html>>,
}

impl Html {
    fn new(tag: &str) -> Html {
        Html {
            tag: tag.to_owned(),
            children: None,
            parent: None,
        }
    }
    fn add_child(&mut self, tag: &str) {
        if let None = self.children {
            self.children = Some(Vec::new());
        }
        let children = self.children.as_mut().unwrap();
        children.push(Rc::new(RefCell::new(Html::new(tag))));
    }
    fn get_child(&self, tag: &str) -> Option<Ref<Html>> {
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
    fn get_child_mut(&mut self, tag: &str) -> Option<RefMut<Html>> {
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
    fn remove_first(&mut self, tag: &str) -> Result<(), &str> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init() {
        let html = Html::new("html");
        assert_eq!(html.tag, "html");
        assert!(html.children.is_none());
        assert!(html.parent.is_none());
    }
    #[test]
    fn add_child() {
        let mut html = Html::new("html");
        html.add_child("div");
        let children = html.children.as_ref().unwrap();
        assert_eq!(children.len(), 1);
    }
    #[test]
    fn mutable_child() {
        let mut html = Html::new("html");
        html.add_child("div");
        {
            let mut div = html.get_child_mut("div").unwrap();
            div.add_child("p");
        }
        html.get_child("div").unwrap().get_child("p").unwrap();
    }
    #[test]
    fn remove_first() {
        let mut html = Html::new("html");
        html.add_child("div");
        html.add_child("div");
        assert!(html.children.as_ref().unwrap().len() == 2);
        {
            let err = html.remove_first("div");
            err.unwrap();
        }
        assert!(html.children.unwrap().len() == 1);
    }
}
