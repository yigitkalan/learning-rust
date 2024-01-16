use std::fmt::{Debug, Display};

fn main() {
    println!("Hello, world!");
    let p1 = Project {
        title: String::from("Osk"),
        subject: String::from("operating systems"),
        creator: String::from("yig"),
    };
    println!("{}", p1.general_summary())
}

//Traits are kinda like interfaces, they define common behaivor
pub trait Summary {
    //while the method can be abstract like this
    //we can fill it to be a concrete method as a default

    //if the method in the trait is abstract, we have to define
    //and implement the method in the impelemted struct
    fn summarize(&self) -> String;

    fn general_summary(&self) -> String {
        format!("Read more from {}...", self.summarize())
    }
}
pub struct Project {
    pub title: String,
    pub subject: String,
    pub creator: String,
}

impl Summary for Project {
    fn summarize(&self) -> String {
        format!(
            "{} by {}, in the area of {}",
            self.title, self.creator, self.subject
        )
    }
}

pub struct Post {
    pub content: String,
    pub username: String,
    pub likes: i32,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("{}'s post has {} likes'", self.username, self.likes)
    }
}

//this is just like using an interface instead of its concrete classes
//we can take any kind that implements Summary
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
//the way above is more suitable for simpler cases
//for example if we're taking 3 params we can use below instead

//method above is a shortened way of writing below
//this is also called "trait bound", suitable for rather complex
//situations
pub fn notifyy<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn send<T: Summary + Display>(item: T) {}
//we can say that we are getting a type that implements both
//Summary and Display
pub fn send2(item: impl Display + Summary) {}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {}
//INSTEAD
fn some_function2<T, U>(t: T, u: U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

//We can use "trait bound" syntax or shorter syntax for returning values
// for example we might return a type that implements summary type



//we can also implement a struct specifically for some types that,
//implements specified traits
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}



//Below is in the standart library and it says that implement ToString
//for any type that implements Display, this might be a great way 
// impl<T: Display> ToString for T {
//  // --snip--
// }
