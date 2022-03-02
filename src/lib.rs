mod html;
#[cfg(test)]
mod test {
    use crate::html::Html;

    #[test]
    fn init() {
        let html = Html::new("html");
        let html = html.borrow();
        assert_eq!(html.tag, "html");
        assert!(html.children.is_none());
        assert!(html.parent.is_none());
    }
    #[test]
    fn add_child() {
        let mut html = Html::new("html");
        Html::add_child(&mut html, Html::new("div"));
        let html = html.borrow_mut();
        let children = html.children.as_ref().unwrap();
        assert_eq!(children.len(), 1);
    }
    #[test]
    fn mutable_child() {
        let mut html = Html::new("html");
        Html::add_child(&mut html, Html::new("div"));
        let html = html.borrow_mut();
        {
            let mut div = html.get_child("div").unwrap();
            Html::add_child(&mut div, Html::new("p"));
        }
        html.get_child_ref("div")
            .unwrap()
            .get_child_ref("p")
            .unwrap();
    }
    #[test]
    fn remove_first() {
        let mut html = Html::new("html");
        Html::add_child(&mut html, Html::new("div"));
        Html::add_child(&mut html, Html::new("div"));
        let mut html = html.borrow_mut();
        assert!(html.children.as_ref().unwrap().len() == 2);
        {
            let err = html.remove_first("div");
            err.unwrap();
        }
        assert!(html.children.as_ref().unwrap().len() == 1);
    }
    #[test]
    fn parents() {
        let mut html = Html::new("html");
        Html::add_child(&mut html, Html::new("head"));
        {
            let html = html.borrow_mut();
            let mut head = html.get_child("head").unwrap();
            Html::add_child(&mut head, Html::new("meta"));
            Html::add_child(&mut head, Html::new("meta"));
            Html::add_child(&mut head, Html::new("script"));
        }
        {
            let html = html.borrow();
            let head = html.get_child_ref("head").unwrap();
            assert!(head.parent.as_ref().unwrap().upgrade().is_some());
        }
    }
    // #[test]
    // fn complex() {
    //     let mut html = Html::new("html");
    //     Html::add_child(&mut html, Html::new("head"));
    //     Html::add_child(&mut html, Html::new("body"));
    //     {
    //         let html = html.borrow_mut();
    //         {
    //             let mut head = html.get_child("head").unwrap();
    //             Html::add_child(&mut head, Html::new("meta"));
    //             Html::add_child(&mut head, Html::new("meta"));
    //             Html::add_child(&mut head, Html::new("script"));
    //         }
    //         {
    //             let mut body = html.get_child("body").unwrap();
    //             Html::add_child(&mut body, Html::new("div"));
    //             Html::add_child(&mut body, Html::new("div"));
    //             Html::add_child(&mut body, Html::new("a"));
    //         }
    //     }
    // }
}
