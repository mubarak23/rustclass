
use traitsdemo::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("200OKDeveloper"),
        content: String::from("i did not get the memo"),
        reply: false,
        retweet: true,
    };
    
    println!("1 new tweet: {}", tweet.summarize());
    println!("tweet readmore: {}", tweet.summarize_readmore());
       let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
     println!("article Readmore: {}", article.summarize_readmore());


}
