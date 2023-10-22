fn main() {
    let mut x = 3;
    //this is just a copy of the value
    let y = x;
    println!("x = {}, y = {}", x, y);
    x = 1;
    println!("x = {}, y = {}\n\n\n\n", x, y);

    ////the pointer, length, and capacity are stored on the stack
    ////the actual data is stored on the heap
    let s1 = String::from("hello");

    ////this is a pointer to the value
    // let s2 = s1;
    // println!("{}", s2);

    //this will not work because s1 is no longer valid
    //this is done because rust does not want to free the same memory twice
    // println!("{}", s1); //doesnt compile

    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // -----------------------------------------------------------------
    // Passing a variable to a function will move or copy, just as assignment does
    let s = String::from("hello");

    //after this line, s is no longer valid
    takes_ownership(s);

    // -----------------------------------------------------------------
    // //if we want to use again we have to return the used value to use later
    // //also we need to return the actual value from the method
    // //this may be inefficient
    // let test_st = String::from("Testt");
    // let (test_st, len) = calculate_length(test_st);
    // println!("The length of '{}' is {}.", test_st, len);

    // -----------------------------------------------------------------
    //instead we can pass a reference to the value
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("2 {}", s2);

    // -----------------------------------------------------------------
    {
        let s5 = &mut s2;
        println!("5 {}", s5);
    } // s5 goes out of scope here, so we can make a new reference with no problems.

    let s3 = &mut s2;
    // let s4 = &mut s2; //this will not work because we can only have one mutable reference to a value
    println!("3 {}", s3);

    // -----------------------------------------------------------------

    // let mut s = String::from("hello");
    //
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    //
    // println!("{}, {}, and {}", r1, r2, r3);
    // A similar rule exists for combining mutable and immutable references.

    // -----------------------------------------------------------------
    // let reference_to_nothing = dangle();
    // ----------------------------------------------------------------- Slices
    println!("\n\n\n\n\n");
    let mut s = String::from("helloworld");
    let hel = &mut s[0..5]; // = &s[..5]
    println!("{hel}");
    // println!("{s}");
    let last = &s[3..s.len()]; // = &s[3..]

    println!("first word : {}",first_word(&String::from("hello world")));
    
}

fn first_word(st: &str) -> &str {
    // let bytes = st.as_bytes();
    // //iter returns each element in a collection
    // //enumerate wraps the result of iter and returns each element as part of a tuple instead
    // for (i, &item) in bytes.iter().enumerate(){
    //     println!("{} {}", i, item);
    //     if item == b' ' {
    //         return i;
    //     }
    // }
    // st.len()
    
    //slice way
    let bytes = st.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &st[..i];
        }
    }
    &st[..]
}
// -----------------------------------------------------------------
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
//     // s is dropped here so the reference is pointing to nothing
//     //DANGER!
// }

// The Rules of References
// •	 At any given time, you can have either but not both of the following: one
// mutable reference or any number of immutable references.
// •	 References must always be valid.

// -----------------------------------------------------------------

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(st: &String) -> usize {
    // st is a reference to a String
    // st.push_str(", world"); // this will not work because we are not allowed to modify something we have a reference to
    st.len()
} // Here, st goes out of scope. But because it does not have ownership of
  // what it refers to, nothing happens.

fn change(strr: &mut String) {
    strr.push_str(", world");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}
