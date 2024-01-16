fn main() {
    another_function(3, false);

    let x = {
        let y = 2;
        //ending with no semicolon as the return of this expression
        y + 1
    };
    println!("{}", x);
    println!("{}", five());
    println!("{}", plus_one(4));
}


//function parameters' types should be annotiated
fn another_function(x: i32, b: bool) {
    if b == true {
        println!("VALUE OF X is {}", x);
    }
}


fn five() -> i32 {
    5
}

fn plus_one(number: i32) -> i32 {
    number + 1
}

//Statements are instructions that perform some action and do not return a value. 
//Expressions evaluate to a resulting value. 
