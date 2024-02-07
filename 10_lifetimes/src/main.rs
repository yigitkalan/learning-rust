use std::fmt::Display;

fn main() {
    println!("Hello, world!");
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    //
    //by converting to this we can ensure that the reference
    //to the x sure lives long enough to satisfy use of r
    // let r;
    // let x = 5;
    // r = &x;
    // println!("r: {}", r);

    let s1 = String::from("hello");
    let s2 = String::from("worldd");
    let result = longest(&s1, &s2);
    println!("The longest string is: {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let test;
    {
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        test = ImportantExcerpt {
            part: first_sentence,
        };
    }
    println!("{}", test.part);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This annotation means an instance of ImportantExcerpt can’t
// outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_announcement<'a, T>(ann: T, x: &'a str, y: &'a str) -> &'a str
where
    T: Display,
{
    println!("Announcement! : {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
