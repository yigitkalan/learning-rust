use std::io::{self, Write};

fn main() {
    let number = 3;

    if number < 5 {
        println!("number is small");
    } else if number < 3 {
        println!("number is really small ")
    } else {
        println!("number is big");
    }

    print!("Enter a number : ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read the line");

    let num: i32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };

    let is_big = if num > 3 { " " } else { " not " };
    println!("the number is{}big", is_big);

    let mut count = 0;

    let _res = loop {
        count += 1;
        if count == 10 {
            //can return values from loop using break
            break count * 2;
        }
    };

    while count < 100 {
        count += 1;
    }

    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut index = 0;

    //imperative way
    // while index < arr.len() {
    //     println!("the value is {}", arr[index]);
    //     index += 1;
    // }

    //declarative way
    for element in arr.iter() {
        println!("the value is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
