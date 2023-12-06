fn main() {
    //VARIABLE WAY
    // let width = 21;
    // let height = 2;
    // println!("The are of the rectangle is {} square pixels",area_variable(width, height));

    //TUPLE WAY
    // let rect = (21,2);
    // println!("The are of the rectangle is {} square pixels",area_tuple(rect));

    //STRUCT WAY
    let rect = Rectangle {
        width: 21,
        height: 2,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 2,
    };

    let rect3 = Rectangle {
        width: 30,
        height: 2,
    };
    // println!(
    //     "The are of the rectangle is {} square pixels",
    //     area_struct(&rect)
    // );
    // println!("And this is the rectangle : {}", rect); //ERROR
    // println!("And this is the rectangle : {:?}", rect); //':?' is for debug printing
    // println!("And this is the rectangle : {:#?}", rect); //':#?' is for a more formatted debug

    //METHOD WAY
    println!("The are of the rectangle is {} square pixels\n", rect.area());
    println!("Is first rectangle can hold the second? : {}\n", rect.can_hold(&rect2));
    println!("Is first rectangle can hold the third? : {}\n", rect.can_hold(&rect3));

    let sq = Rectangle::get_square(3);
    println!("Created square of size 5 = {:?}\n", sq )

}

fn area_variable(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(edges: (u32, u32)) -> u32 {
    edges.0 * edges.1
}

//THIS IS A FUNCTION
fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

//AN IMPLEMENTATION BLOCK
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        // self.area() > other.area()
        self.width >= other.width && self.height >= other.height
    }

    //THIS IS A METHOD, cuz it's tied to a struct, enum or a trait object
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //THIS IS AN 'ASSOCIATIVE FUNCTION' not a METHOD
    //LIKE 'String::from', not on the instance, on the struct itself 
    fn get_square(size: u32) -> Rectangle{
        Rectangle { width: size, height: size } 
    }
}

// //CAN SEPERATE IMPL BLOCKS LIKE THIS IF YOU WANT
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         // self.area() > other.area()
//         self.width >= other.width && self.height >= other.height
//     }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
