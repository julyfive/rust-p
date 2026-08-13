pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        //概括
        format!("{},by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        //概括
        format!("{}: {}", self.username, self.content)
    }

    fn print_word(&self) {
        //覆盖默认行为
        println!("Hello World, My name is Tweet")
    }
}

pub trait Summary {
    //定义一个trait
    fn summarize(&self) -> String; //定义一个方法签名
    fn summarize_own(&self) {
        self.print_word();
    }
    fn print_word(&self) {
        //定义一个默认方法
        println!("Hello World")
    }
}

pub trait Notify {
    fn notify(&self) -> String;
}
