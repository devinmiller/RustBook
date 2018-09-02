extern crate traits;

use traits::aggregator::{Tweet,NewsArticle,Summary};

fn notify<T>(item: T) where T: Summary{
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = get_tweet();

    //println!("1 new tweet: {}", tweet.summarize());
    notify(tweet);

    let article = get_article();

    //println!("New article available: {}", article.summarize());
    notify(article);
}

fn get_tweet() -> impl Summary {
    Tweet {
        username: String::from("adeptus_astartes"),
        content: String::from("Your heritical taint will be purged from this world, xenos filth!"),
        reply: false,
        retweet: false
    }
}

fn get_article() -> impl Summary {
    NewsArticle {
        headline: String::from("Local Adeptus Astartes Purges Taint"),
        location: String::from("Baal Secundus"),
        author: String::from("Ordo Xenos"),
        content: String::from("Exterminatus!")
    }
}
