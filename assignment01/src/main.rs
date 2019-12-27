//Custom Types ...
#[derive(Debug)]
struct NewsArticles {
    author : String,
    content: String,
}

impl Summary for NewsArticles{
    fn Summarize(&self)-> String{
        format!("The Author of this Book is :{} and This is The Content :{}",self.author,self.content)
    }
}

struct Tweets {
    username : String,
    content: String,
}

impl Summary for Tweets{
    fn Summarize(&self)-> String{
        format!("The Username of Author is :{} and This is The Content :{}",self.username,self.content)
    }
}
//End Custom types


//Summary Signature
pub trait Summary {
    fn Summarize(&self)-> String;
}
//End Summary Signature


fn main() {

    let Articles = NewsArticles{
        author:"Faizan Ali".to_string(),
        content:"IOT Q2 My First Assignment We can done!!!!!".to_string(),
    };
    println!("{}",Articles.Summarize());

    let newTweet = Tweets{
        username:"Faizan12m@gmail.com".to_string(),
        content:"I have done my Assigment01 and has been upload".to_string(),
    };
    println!("{}",newTweet.Summarize());

    
}
