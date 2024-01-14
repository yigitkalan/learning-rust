pub fn sub_main() {
    //----------------------------------------VECTORS
    // let v: Vec<i32> = Vec::new();

    let mut v = Vec::new();
    //if we push directly into vector type error goes away
    v.push(4);
    println!("{:?}", v);

    let v1 = vec![1, 2, 3, 4, 5];
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

    for _i in &v1 {
        // println!("{}", _i);
    }

    let mut ve = vec![100, 23, 65];
    for i in &mut ve {
        *i += 50;
    }

    let mut row: Vec<SpreadsheetCell> = Vec::new();
    row.push(SpreadsheetCell::Int(2));
    row.push(SpreadsheetCell::Text(String::from("HELLooo")));

    let test_vec = vec![1, 2, 3, 6, 1, 2, 55, 6];
    println!("{}", median(&test_vec));
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn mean(nums: &Vec<i32>) -> f64 {
    let len = nums.len();
    let total: i32 = nums.iter().sum();
    total as f64 / len as f64
}

fn median(nums: &Vec<i32>) -> i32 {
    let mut numa = nums.clone();

    //parameter is a anonymous function (lambda) this reverse sorts
    // numa.sort_by(|a , b| b.cmp(a));
    numa.sort();
    println!("{:?}",numa);
    match numa.get(numa.len() / 2) {
        Some(A) => *A,
        None => -1,
    }
}
