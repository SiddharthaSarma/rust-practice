use traits:: {NewsArticle, Tweet, Summary};

fn main() {
    // A trait defines functionality a particular type has and can share with other types.
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("ofcourse, as you probably already know, people."),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let news = NewsArticle {
        headline: String::from("Penguins win the Stanley cup championship"),
        content: String::from("The Pittsburgh team once again are the best hockey team"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
    };
    println!("New article available: {}", news.summarize());
}
