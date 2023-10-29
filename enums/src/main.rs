fn main() {
    // let four = IPAddrKind::V4;
    // let six = IPAddrKind::V6;

    //// 1
    // let home = IPAddr{kind: IPAddrKind::V4, addr: String::from("127.0.0.1")};
    // let loopback = IPAddr{kind: IPAddrKind::V6, addr: String::from("::1")};
    //// 2
    let home = IPAddrKind::V4(234, 12, 9, 1);
    let home = IPAddrKind::V6(String::from("::1"));


    let m = Message::Write(String::from("Helloo"));
    m.call();

    let some_number = Some(4);
    let absent_number: Option<i32> = None;
}

//// 1
// struct IPAddr{
//     kind: IPAddrKind,
//     addr: String
// }
// enum IPAddrKind{
//     V4,
//     V6
// }

// 2
//ORRRR
enum IPAddrKind {
    //each type can have different data
    V4(u8, u8, u8, u8),
    V6(String),
}

// struct QuitMessage; // unit struct
// struct MoveMessage {
//  x: i32,
//  y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

//EQUIVALENT WAY BELOW

enum Message {
    //stores nothing
    Quit,
    //stores an anonymous struct
    Move { x: i32, y: i32 },
    //stores a string
    Write(String),
    //stores 3 values
    ChangeColor(i32, i32, i32),
}

//CAN DO IMPL BLOCKS FOR ENUMS TOO
impl Message {
    fn call(&self){
        //TODO
    }
    
}
