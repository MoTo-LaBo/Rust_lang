trait Fruist {
    fn price(&self) -> u32;
}
struct Apple;

impl Fruist for Apple {
    fn price(&self) -> u32 {
        10
    }
}

struct Banana;
impl Fruist for Banana {
    fn price(&self) -> u32 {
        5
    }
}

trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

trait Message {
    fn message(&self) -> String {
        String::from("Message")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}
impl Message for NewsArticle {}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn run() {
    // apple, banana の instance
    let append = Apple {};
    let banana = Banana {};
    get_price(append);
    get_price(banana);

    // Tweet の instance
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably already know people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    // NewsArticle の instance
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NH_.",
        ),
    };
    println!("{}", article.summarize());

    // notify
    notify(&article);

    // notify other
    notify_another(&article);
    // notify_another(&tweet);
}

fn get_price<T: Fruist>(fruist: T) {
    println!("price is: {}", fruist.price());
}
// Summary trait を持っている data型であれば item に渡す事ができる
// data reference(参照) を渡す事ができる
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Summary + Message trait 両方を実装している data型だけ受け取れる制約をかける
fn notify_another(item: &(impl Summary + Message)) {
    println!("Breaking news! {}", item.summarize());
    println!("Message! {}", item.message());
}
