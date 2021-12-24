pub trait Display {}

// Trait Bound Syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Brokest news! {}", item.summarize());
}

// syntactic sugar for notify
pub fn notify_sugar(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Can use different classes that implement the trait in the same method
pub fn notify_mixed(item1: &impl Summary, item2: &impl Summary) {}

// To force both to use the same type, use trait bound syntax
pub fn notify_same<T: Summary>(item1: &T, item2: &T) {}

// Specify multiple trait bounds with sugar
pub fn motify(item: &(impl Summary + Display)) {}

// And with trait bound syntax
pub fn lotify<T: Summary + Display>(item: &T) {}

// And an alternative way. 
fn some_function<T: Display + Clone, U: Clone + std::fmt::Debug>(t: &T, u: &U) {}
fn other_way<T, U>(t: &T, u: &U)
    where T: Display + Clone, 
    U: Clone + std::fmt::Debug
{}

// Returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("satoshi"),
        content: String::from(""),
        reply: false,
        retweet: false,
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    //fn summarize(&self) -> String;
    
    // default implementation.
    fn summarize(&self) -> String {
         format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    //fn summarize(&self) -> String {
        //format!("{}, by {} ({})", self. headline, self.author, self.location)
    //}

    // use default implementation of summarize.
    
    // Implement summarize_author
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
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
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("jerome_p"),
        content: String::from("Money printer go BRRRR"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Champioship!"),
        location: String::from("Pittsburge, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins picked pottery paired pigs pumping pumpkins"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("New article available! {}", article.summarize());

    notify(&tweet);
    notify_sugar(&tweet);
    notify(&article);
    notify_sugar(&article);
}
