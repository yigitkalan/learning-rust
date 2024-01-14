pub fn sub_main() {
    println!("\n\n\n\n\n\n\n\n STRINGS\n\n");
    let mut _s = String::new();

    // let data = "initial string";
    // let _s = data.to_string();
    // let _s = "initial string".to_string();
    // let _s = String::from("initial string");
    // let _hello = String::from("Dobrý den");
    // let _hello = String::from("Hello");
    // let _hello = String::from("नमस्ते");
    // let _hello = String::from("こんにちは");
    // let _hello = String::from("안녕하세요");
    // let _hello = String::from("你好");
    // let _hello = String::from("Olá");
    // let _hello = String::from("Здравствуйте");
    // let _hello = String::from("Hola");

    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    let mut s = String::from("lo");
    s.push('l');

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    //add method takes the ownership of first parameter and borrows other ones then
    //return the ownership of the result

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    //can use macro
    let _s = format!("{}-{}-{}", s1, s2, s3);

    //INDEXING
    let _k = String::from("power of vim");
    // println!("{}", k[2]); // gives error cuz rust doesnt support indexing for stings
    let len = String::from("Hola").len();
    println!("{}", len);
    let len = String::from("Здравствуйте").len();
    println!("{}", len);
    //in rust not every character is stored in 2 byte like cyrillic characters above

    let hello = "Здравствуйте";
    let _s = &hello[0..4]; //this is OK
                           // let part = &hello[0..1]; //this panicks cuz it halves the 2byte long character

    for _c in "नमस्ते".chars() {
        // println!("{}", c);
    }

    for _b in "नमस्ते".bytes() {
        // println!("{}", b);
    }

    let _word = "first";
    println!("{}",pig_latin("apple"));
}

fn pig_latin(word: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut temp = word.to_string();

    if vowels.iter().any(|&c| word.starts_with(c)) {
        temp.push_str("-hay");
        temp
    } else {
        let (first, rest) = temp.split_at(1);
        let first = first.to_string() + "ay";
        let rest = rest.to_string() + "-";
        rest + &first
    }
}
