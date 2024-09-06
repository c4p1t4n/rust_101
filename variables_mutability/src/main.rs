// fn error_immutable_variable() {
    
//     let  x = 5;
//     println!("The value of x is: {x}");
//     x = 7;
//     println!("The value of x is: {x}");
// }
fn shadowing(){
    // Shadowing is different of mut because you can change the type of the variable
    // after each operation with shadowing the variable continue to be immutable
    println!("Shadowing");
    let x = 5;
    println!("The value of x is: {x}");
    let x = 7+x;
    println!("The new value of x is: {x}");
    shadowing_example_01();
}
fn shadowing_example_01(){
    // in this case we tranform the type of variable from string to integer
    // this is god because i don't need to create a new variable like spaces_len or something like that
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}
// fn error_shadowing_example_02(){
//     // in this case we can change the type of the variable because the type is different
//     let mut spaces = "   ";
//     spaces = spaces.len();
//     println!("The value of spaces is: {spaces}");
// }



fn mutable_variable(){
    println!("Mutable variable");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 7;
    println!("The new value of x is: {x}");
}
fn constant(){
    // constants are different of variables because can't use the mut keyword
    // constants need be signed
    println!("Constant");
    const THREE_HOURS_IN_SECONDS: u32 =  60 * 60 * 3;
    println!("Constant value {THREE_HOURS_IN_SECONDS}");
}

fn main() {
    // immutable_variable();
    shadowing();
    mutable_variable();
    constant();
}