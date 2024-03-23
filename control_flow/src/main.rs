// fn if_in_let_statement(){
//     let condition = false;
//     let number = if condition { 5 } else { 6 };
//     println!("The value of number is: {number}");
// }

// fn control_flow_loop(){
//     loop{
//         println!("again!");
//     }
// }
// fn returning_values_from_loop(){
//     let mut  counter = 0;
//     let result = loop{
//         counter = counter+1;
//         if counter == 2{
//             break counter * 2;
//         }
//     };
//     println!("The result is: {}", result);
// }
// fn loop_labels(){
//     'counting_up: loop {

//         'inner: loop {
//             break 'inner
//             // break 'counting_up;
//             // if count == 2 {
//             //     break 'counting_up;
//         };
//         break 'counting_up
//     };
//     println!("Exited the loop!");
// }

// fn loop_through_collection(){
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;
//     while index <  a.len() - 1{
//         println!("The value is: {}", a[index]);
//         index = index + 1;
//     }
//     for element in a {
//         println!("The value is: {}", element);
//     }
// }

// fn loop_count_down(){
//     for number in (1..=4).rev() {
//         println!("{number}");
//     }
//     println!("LIFTOFF!!!");
// }
// fn convert_f_to_c(f: f64) -> f64 {
//     (f - 32.0) * 5.0 / 9.0
// }
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    // if_in_let_statement()
    // control_flow_loop();
    // returning_values_from_loop();
    // loop_labels();
    // loop_through_collection()
    // loop_count_down();
    let fibonnaci_value = fibonacci(20);
    let fibonnaci_value: String = fibonnaci_value.to_string();
    println!("{fibonnaci_value}");
}