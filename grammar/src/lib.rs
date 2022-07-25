
// trait
// train告诉rust编译器，某种类型具有哪些并且可用与其它类型共享的功能
// trait抽象的定义共享行为
// trait bounds(约束) 泛型类型参数可指定为实现了特定行为的类型

use std::fmt::{Display, Debug};

pub trait Summary {
    fn summarize(&self) -> String;
    // 可以定义默认实现
    // 可以在实现中重写默认实现
    // 在默认实现中，可以调用前面的方法，但是在类型具体实现中，则必须实现这个被调用的方法
    
}
// 在类型中实现train
#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// train bound的写法
pub fn notifyv2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize())
}

// 指定T实现了Summary和Display这两个train
pub fn notifyv3 <T: Summary + Display, U: Clone +Debug>(a: T, b: U) -> String {
    format!("Breaking news! {}", a.summarize())
}

// 由于上面写法使函数前面非常长，为了简介函数前面，可以用where
pub fn notifyv4<T,U>(a:T, b: U) -> String
where 
    T: Summary + Display,
    U: Clone + Debug,
    {
        format!("Breaking news! {}", a.summarize())
}

// 使用train作为返回类型
// 使用impl trait 只能返回确定的同一种类型，返回可能不同类型的代码会报错
pub fn notifyv5<T: Summary> (a:T) -> impl Summary+Debug {
    NewsArticle{
        headline: String::from("horse_ebooks"),
        location: String::from("广州-广电云平"),
        author: String::from("Jack"),
        content: String::from("this is test"),
    }
}
//