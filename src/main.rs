trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>, post: &mut Post) -> Box<dyn State> {
        post.content = String::new();
        Box::new(Draft {})
    }
    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }
}

struct Draft;

struct PendingReview;

struct Published;

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

    fn reject(self: Box<Self>, _: &mut Post) -> Box<dyn State> {
        self
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

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

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn add_text(&mut self, string: &str) {
        self.content.push_str(string);
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject(self))
        }
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate salad as lunch");

    assert_eq!(post.content(), "");

    post.request_review();

    assert_eq!(post.content(), "");

    post.reject();

    post.add_text("Nuptias crescere!");

    post.add_text("You have to preach, and hurt mineral by your existing.");

    assert_eq!(post.content(), "");

    post.request_review();

    post.add_text("kjdskjhjksdnxjhsdkjlfgksjhss");

    post.approve();

    assert_eq!(
        post.content(),
        "Nuptias crescere!You have to preach, and hurt mineral by your existing.kjdskjhjksdnxjhsdkjlfgksjhss"
    );

    post.reject();

    assert_eq!(
        post.content(),
        "Nuptias crescere!You have to preach, and hurt mineral by your existing.kjdskjhjksdnxjhsdkjlfgksjhss"
    );
}
