use myapp::traits::{NewsArticle, Notify, Summary, Tweet};
fn main() {
    let tweet = Tweet {
        username: "张伟".to_string(),
        content: String::from("xxxxx"),
        reply: false,
        retweet: false,
    };
    let news = NewsArticle {
        headline: "我是标题".to_string(),
        location: "我是地址".to_string(),
        author: "张三".to_string(),
        content: "大家好，我是张三".to_string(),
    };

    pub fn notify(item: &impl Summary) {
        //Trait 作为参数
        println!("Breaking news: {}", item.summarize());
    }
    // Trait Bound 语法
    pub fn notify_t<T: Summary>(item: &T) {
        println!("Breaking news: {}", item.summarize());
    }
    pub fn notify_t1(item: &impl Summary) {}

    //Trait Bound 适用于更复杂的情况

    pub fn notify_tt<T: Summary, U: Notify>(item: &T, item2: &U) {
        println!("Breaking news: {}, {}", item.summarize(), item2.notify());
    }
    pub fn notify_tt1<T, U>(item: &impl Summary, item2: &impl Notify) {
        println!("Breaking news: {}, {}", item.summarize(), item2.notify());
    }
    pub fn notify_tt2<T, U>(item: &T, item2: &U)
    where
        T: Summary,
        U: Notify,
    {
        println!("Breaking news: {}, {}", item.summarize(), item2.notify());
    }

    pub fn notify_ttt<T: Summary + Notify>(item: &T) {
        println!("Breaking news: {}, {}", item.summarize(), item.notify());
    }
    pub fn notify_ttt1(item: &(impl Summary + Notify)) {
        println!("Breaking news: {}, {}", item.summarize(), item.notify());
    }
    tweet.print_word();
    news.print_word();
    notify(&tweet);
    println!("1 new tweet:{}", tweet.summarize());
}
