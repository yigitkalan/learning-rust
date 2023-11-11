use std::collections::HashMap;

pub fn sub_main() {
    // let mut scores = HashMap::new();
    // scores.insert(String::from("blue"), 10);
    // scores.insert(String::from("yellow"), 12);
    //
    // let teams = vec![String::from("blue"), String::from("yellow")];
    // let initial_scores = vec![10, 50];
    //
    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("blue");
    let mut map = HashMap::new();
    //types which doesnt implement copy trait will be moved
    map.insert(field_name, field_value);
    // after this field name and field value are invalid

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 12);

    let team_name = String::from("yellow");
    let score = scores.get(&team_name);
    println!("Score is : {}\n", score.unwrap());

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwriting a value
    scores.insert(String::from("blue"), 25);

    // Only inserting a value if the key has no value
    scores.entry(String::from("yellow")).or_insert(50);
    scores.entry(String::from("red")).or_insert(50);
    // println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();

    //operating based on the value of key
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    let test_vec = vec![1,1, 2, 4, 5, 2, 1, 6, 7, 3, 1, 2];
    find_mod(&test_vec);
}

fn find_mod(nums: &Vec<i32>) {
    if nums.len() <= 1 {
        if nums.len() == 0 {
            return;
        }
    }
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        let count = counts.entry(*num).or_insert(0);
        *count += 1;
    }

    //Below gets all keys with the maximum occurance
    // let max = counts.values().max().unwrap();
    // let maxs: Vec<&i32> = counts
    //     .keys()
    //     .filter(|a| counts.get(a) == Some(max))
    //     .collect();
    // println!("{:?}", maxs);

    //Instead can use max_by_key to get the first element with the max value
    if let Some((mod_key, _)) = counts.iter().max_by_key(|&(_, count)| count) {
        println!("{:?}", mod_key);
    }
}
