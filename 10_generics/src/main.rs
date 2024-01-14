fn main() {
    let my_vec = vec![77, 3, 277, 32, 1];
    println!("{} is the largest num", find_largest(&my_vec));

    //this process needs duplication for applying in
    //another vector, instead create a function

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let both_float = Point { x: 1.0, y: 4.0 };
    println!("X of first point {}", both_float.x());
}

fn find_largest(slicee: &[i32]) -> i32 {
    let mut largest = slicee[0];

    //in here the '&' before num is for destructuring not for
    //taking a reference of num, to match the pattern;
    // there are incoming &i32's from iter() method and
    // we are saying that match each &i32 with &num, so
    // in this case num is the i32 (number itself)
    for &num in slicee.iter() {
        if num > largest {
            largest = num;
        }
    }
    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//INSTEAD

// fn largest<T>(list: &[T]) -> T {
//THIS WILL GIVE AN ERROR FOR NOW
// let mut largest = list[0];
// for &item in list.iter() {
//     if item > largest {
//         largest = item;
//     }
// }
// largest
// }

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
