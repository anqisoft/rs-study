// eg17-12
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // eg17-13: +
        pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // eg17-14: +
    pub fn content(&self) -> &str {
        ""
    }

    // eg17-15: +
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    // eg17-16: +
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    // eg17-17: +
pub fn content(&self) -> &str {
    self.state.as_ref().unwrap().content(self)
}
}

trait State {
    // eg17-15: +
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    // eg17-16: +
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // eg17-17: +
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    // eg17-15: +
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // eg17-16: +
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// eg17-15: +
struct PendingReview {}

// eg17-15: +
impl State for PendingReview {
    // eg17-15
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // eg17-16: +
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    // eg17-17: +
}


    // eg17-16: +
struct Published {}

    // eg17-16: +
impl State for Published {
    // eg17-16:
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // eg17-16:
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // eg17-17: +
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}