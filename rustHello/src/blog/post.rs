
///演示rust的 OOP：参考：https://doc.rust-lang.org/stable/book/ch17-03-oo-design-patterns.html#requesting-a-review-of-the-post-changes-its-state


struct Post {
    state: Option<Box<dyn State>>,
    content:String,
}

struct Draft {}

struct PenddingReview {}

struct Published {}


trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self,post: &'a Post) -> &'a str {
        ""
    }
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PenddingReview{})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

}


impl State for PenddingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }

}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}


impl Post {
    ///新增
    fn new(content: &str) -> Post {
        Post {
            state: Some(Box::new(Draft{})),
            content: content.to_string()
        }
    }
    ///添加文本
    fn add_txt(&mut self, content: &str) {
        self.content.push_str(content);
    }
    ///返回文本内容
    fn content(&self) -> &str {
       let aa = self.state.as_ref();
       let bb= aa.unwrap();
        bb.content(self)
    }
    ///请求审核，如果当前状态正确，就进入下一状态，request_review返回下一个状态
    fn request_review(&mut self) {
        match self.state.take() {
            Some(s) => {
                self.state=Some(s.request_review());
            },
            None =>{
                panic!("State is not valid when request review!")
            }
        }
    }
    ///审核通过，如果当前状态正确，就进入下一状态，approve返回下一个状态
    fn approve(&mut self) {
        match self.state.take() {
            Some(s) =>{
                self.state=Some(s.approve());
            },
            None =>{
                panic!("State is not valid when approve!")
            }
        }
    }
}


pub fn run() {

    let mut p =Post::new("hello rust content is very good");
    let c =p.content();

    assert_eq!("",c);
    p.request_review();

    let c = p.content();
    assert_eq!("",c);

    // 只有通过审核后，才能看到文章内容
    p.approve();
    let c = p.content();
    assert_eq!("hello rust content is very good",c)


}






























