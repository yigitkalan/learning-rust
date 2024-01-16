fn main() {
    let mut usr1 = User{
        username: String::from("SY"),
        email: String::from("test@gmail.com"),
        age: 21,
        active: false,
    };
    usr1.email = String::from("changed@gmail.com");
    println!("{}",usr1.email);

    let _usr2 = User{
        username: String::from("dicl"),
        email: String::from("dicl@gmail.com"),
        // age: usr1.age,
        // active: usr1.active
        //instead use 'struct update syntax'
        ..usr1
    };
    
    let origin = Point(0,2,0);
    println!("{}",origin.1);
}

fn _build_user(email: String, username: String) -> User{
    User{
        // email: email,
        // username: username,
        //since field and parameter names are the same we can use 'field init shorhand' syntax
        email,
        username,
        age: 18,
        active: true
    }
     
}

struct User{
    username: String,
    email: String,
    age: u8,
    active: bool
}

//Tuple Struct
//this is a tuple with name and it is seperate from the normal tupels
struct Point(i32,i32,i32);


//Unit-Like Structs are structs without any fields, just with traits
