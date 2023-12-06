fn main() {
    let my_vec = vec![2,3,5,23,27,19];
    println!("{} is the largest num", find_largest(&my_vec));
    // let mut largest = my_vec[0];
    //
    // for num in my_vec{
    //     if num > largest{
    //         largest = num;
    //     }
    // }

    let my_vec = vec![77,3,277,32,1];
    println!("{} is the largest num", find_largest(&my_vec));

    //this process needs duplication for applying in
    //another vector, instead create a function

}

fn find_largest(slicee: &[i32]) -> i32{
    let mut largest = slicee[0];

    //in here the '&' before num is for destructuring not for
    //taking a reference of num, to match the pattern;
    // there are incoming &i32's from iter() method and 
    // we are saying that match each &i32 with &num, so
    // in this case num is the i32 (number itself)
    for &num in slicee.iter(){
        if num > largest{
            largest = num;
        }
    }
    largest

}
