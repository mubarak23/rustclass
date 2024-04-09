pub trait Summary {
  fn summarize(&self) -> String;

  fn summarize_readmore(&self) -> String;

  fn summarize_author(&self) -> String;

}

// implementing a trait on a type
pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String
}

// implementing trait [Summary] on struct type NewsArticle 
impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }

  fn summarize_readmore(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }

  fn summarize_author(&self) -> String {
    format!("@{}", self.author)
  }

  pub fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
  }
  
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

// implementing trait [Summary] on struct type Tweet 
impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }

  fn summarize_readmore(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }

  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}
