fn main() {
    // const MAX_LEVEL: u32 = 10_000;
    //type of a const must be annotated

    let mut x = 5;
    println!("value of x {}", x);
    x = 6;
    println!("value of x {}", x);

    //// gives an error with mut
    // let mut x = "   ";
    // x = x.len();

    //this gives no error with shadowing
    let x = "    ";
    let x = x.len();
    println!("{}", x);

    //TYPES
    //1 SCALAR TYPES
    //1.1 INTEGER TYPYES

    //signed integers
    let _x: i8 = 127;
    let _x: i16 = 123;
    //...
    let _x: i128 = 123123123123;

    //unsigned (only positive) integers
    let _x: u8 = 127;
    let _x: u16 = 123;
    //...
    let _x: u128 = 123123123123;

    //usize or isize (32 or 64 bit based on hardware)
    let _x: usize = 0xff;
    println!("{}", _x);
    let _x: isize = 0o77;
    println!("{}", _x);

    //1.2 FLOATING POINT TYPES
    let _x = 2.0; // f64
                  //f64 is double precision
    let _x: f64 = 2.0; // f64
                       //f32 is single precision
    let _y: f32 = 3.0; // f32

    // //gives error
    // let sum = 3.2 + 2;
    // let multiplication = 33.2 * 2;

    //1.3 BOOLEAN TYPE
    let _b: bool = true;

    //1.4 CHARACTER TYPE
    let _c = 'z';
    let _z = 'Ƶ';


    //2 COMPOUND TYPES

    //2.1 TUPLE TYPE
    //fixed length can be different types in it 
    let _tupp: (i32, u8, i16) = (-12, 6, -123);
    let _tupp = (-12, 6, -123);

    //this is called 'destructuring'
    let (x,_y,_z) = _tupp;
    println!("first value is {}", x);

    //another way_
    println!("first value is {}", _tupp.0);

    //2.2 ARRAY TYPE
    //allocated on the stack rather than the heap
    //fixed length and all elements are the are the same type
    let _a = [2,2,4,5,6,7];
    //contains six i32 types
    let _a: [i32;6] = [2,2,4,5,6,7];
    println!("{}",_a[3]);
    //an arraw with five fours
    let _a = [4;5];
    println!("{}",_a[3]);

}
