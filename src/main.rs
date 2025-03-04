mod control_flow;
mod rust_fundamental;
use control_flow::is_prime::is_prime;
use rust_fundamental::{
    mutable_slice, slice, slice_manipulation,
    structs::{is_adult, Person},
};

fn main() {
    let name = Person {
        age: 55,
        _name: "yunus".to_string(),
    };
    is_prime(4);
    mutable_slice::transform_even_odd(&mut [4, 5, 35, 4]);
    slice_manipulation::update_slice(&mut [44, 5, 6], &[4, 6], 4);
    slice::find_largest_in_slice(&[4, 5, 6]);
    is_adult(&name);
    //let is_prime = control_flow;

    // let m = [3, 4, 6, 8, 3, 4];
    // largest(&m);
    // let result = is_prime(5);
    // assert_eq!(result, true);

    // let result = is_prime(4);
    // assert_eq!(result, false);
    // assert_eq!(fibonacci(2),1)
}

//describe number
// fn describe_number(n: i32) -> String {
//     let is_even = n % 2;
//     if n > 0 && is_even == 0 {
//         "Positive even".to_string()
//     } else if n > 0 && is_even != 0 {
//         "Positive odd".to_string()
//     } else if n < 0 && is_even != 0 {
//         "Negative odd".to_string()
//     } else if n < 0 && is_even == 0 {
//         "Negative even".to_string()
//     } else {
//         "Zero".to_string()
//     }
// }

// // fibonacci
// fn fibonacci(n: u32) -> u32 {
//     // let mut b = 0;
//     // //let mut a  = 0;
//     // while b < n {
//     //     b += 1;
//     //     //a = b;
//     // };

//     // b
//     let a = (n - 1) + (n - 2);
//     a
// }

// pub fn sum_of_evens(start: i32, end: i32) -> i32 {
//     // Your code here...
//     let mut current = 0 ;
//     for element in start..=end{
//         if element % 2  == 0{
//             current = current + element
//         }
//     };
//     current
// }
