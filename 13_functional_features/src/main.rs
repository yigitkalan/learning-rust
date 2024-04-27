use std::{thread, time::Duration};

fn main() {
    //Closure ------------------------------------------------------------------------------------
    let x = vec![1, 2, 3];
    let _equal_to_x = move |z: Vec<i32>| z == x;
    // with move keyword we turn this closure into a FnOnce type like
    // println!("can't use x here: {:?}", x); //This gives errorr

    let simulated_user_specified_value = 10;
    let simulated_random_number = 8;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    //Iterator ------------------------------------------------------------------------------------

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    //sum is an example of a consuming method on iterator
    let sum: i32 = v1_iter.sum();
    println!("{}", sum);

    let v2: Vec<i32> = vec![2, 4, 6, 7, 8, 9, 10];
    let x: Vec<_> = v2.iter().map(|x| x + 2).collect();
    assert_eq!(x, vec![4, 6, 8, 9, 10, 11, 12]);
}

//Closure ------------------------------------------------------------------------------------

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));

        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!(
                "Today run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// fn add_one_v1 (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x| { x + 1 };
// let add_one_v4 = |x| x + 1 ;

//Iterator ------------------------------------------------------------------------------------

#[derive(PartialEq, Debug)]
struct Shoe {
    style: String,
    size: u32,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, my_size: u32) -> Vec<Shoe> {
    //iter uses the source by taking reference to its items
    //while into_iter consumes the source
    shoes
        .into_iter()
        .filter(|shoe| shoe.size == my_size)
        .collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_counter() {
    let mut c = Counter::new();
    assert_eq!(c.next(), Some(1));
    assert_eq!(c.next(), Some(2));
    assert_eq!(c.next(), Some(3));
    assert_eq!(c.next(), Some(4));
    assert_eq!(c.next(), Some(5));
    assert_eq!(c.next(), None);
}

#[test]
fn with_other_iterator() {
    //when we use zip the resulting iterator will have the length of the shorter one
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(f, l)| f * l)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}
