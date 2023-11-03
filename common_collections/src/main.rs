fn main() {
    // let v: Vec<i32> = Vec::new();

    let mut v = Vec::new();
    //if we push directly into vector type error goes away
    v.push(4);
    println!("{:?}", v);

    let mut v1 = vec![1, 2, 3, 4, 5];
    //FIRST WAY OF ACCESING THE ELEMENTS
    let third: &i32 = &v1[2];
    //gives error cuz we are trying to have both mutable and immutable references
    // v1.push(4);
    println!("The third element is {} ", third);

    //SECOND WAY OF ACCESSING THE ELEMENTS
    match v1.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }



    for i in &v1{
        println!("{}",i);
    }
}
