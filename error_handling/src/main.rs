use std::{
    fs::{File, self},
    io::{Error, ErrorKind, Read},
};

fn main() {
    let _v = vec![1, 2, 3, 5, 5];
    //this panicks, panics are unrecoverable errors
    // println!("{}",v[99]);

    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    //since Result enum type implemnts many functions with closures below would
    //be a better implementation rather than nested match statements
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //unrwap or else takes a closure to get what to do in an error situation
    //expect takes a string as a message in error situation
    //just go to definition and be amazed, rust got fucking good docs and source code
}

//propagation of errors
fn get_username_from_file(path: &str) -> Result<String, Error> {
    let f = File::open(path);
    
    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

//this is almost the same way of the above function
fn read_username_from_file(path: &str) -> Result<String, Error> {
    let mut s = String::new();

    // ? operator takes the value from open function if it's an OK result
    // or return from the whole function with the Err if that is the result
    // ? operator also converts the Err type in the result to the Error type in 
    // the function return type, it does this with from function

    // let mut f = File::open("hello.txt")?;
    // f.read_to_string(&mut s)?;
    // Ok(s)

    //Or even shorter way
    // File::open(path)?.read_to_string(&mut s)?;
    // Ok(s)

    //ORR EVEN SHORTER WAY
    //since reading from a file is so common, instead of opening a file than reading from it
    //this function directly reads from a file and return a string result
    fs::read_to_string(path)



    // ? operator can only be used inside functions that returns a result
    // so we cannot use it in main function for example
}


//To put boundaries and make things more strict we can create new types
// the Guess type below might be used in the guessing game from earlier
// pub struct Guess{
//     value: i32,
// }
//
// impl Guess{
//     pub fn new(value: i32) -> Guess{
//         if value < 0  || value > 100{
//             panic!("Guess value must be between 1 and 100, got {}.", value);
//         }
//         Guess { 
//             value
//         }
//     }
//     pub fn value(&self) -> i32{
//         self.value
//     }
// }
