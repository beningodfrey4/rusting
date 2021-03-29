use std::fmt::Display;

pub trait Summary { // this declares the trait Summmary and the methods that need to be defined by types that need this trait
    fn summarize(&self) -> String; // here the methods that define the behavior of the trait is left undefined and every type that implements this trait, 
                                   // needs to implement it for it to qualify
                                   // Note: can have multiple methods that together define behavior of type
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle { // implementation of the methods of Summary trait for NewsArticle type. Now we can say, "NewsArticle implements Summary"
                               // Note: impl of one can call the other, even if unimplimented, but have to make sure type implements all trait methods before calling
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


pub fn notify(item: &impl Summary) { // trait binding, says that item is some type that implements Summary
    println!("Breaking news! {}", item.summarize());
}

pub fn notify<T: Summary>(item: &T) { // same as above but with a trait bound syntax and creates a generic T that can be used more than once
    println!("Breaking news! {}", item.summarize());
}

pub fn notify<T: Summary>(item1: &T, item2: &T) { } // advantage of trait bound syntax. Can have generic type that implements a trait and enforce it to multiple parameters

pub fn notify(item: &(impl Summary + Display)) { } // impl syntax, but for types implementing more than one trait

pub fn notify<T: Summary + Display>(item: &T) { } // same thing with trait bound

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { } // multiple types implementing their own types generically applied on parameters

fn some_function<T, U>(t: &T, u: &U) -> i32 // syntactic sugar for large trait declaring generic functions
    where T: Display + Clone,
          U: Clone + Debug
{ }

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> { // T is any type that implements Display and PartialOrd and this impl block is for Pairs, 
                                        // therefore Pair has an implementation for Ts that implement Display and PartialOrd
                                        // This is usually referred to as "conditional implementation using trait bounds"
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { // Function that takes a list, but only those that contain types T that implements PartialOrd and Copy
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize()); // direct call of trait method on type that implements it.

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
