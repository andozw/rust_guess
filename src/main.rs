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
}
