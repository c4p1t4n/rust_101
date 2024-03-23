//Statements are instructions that perform some action and do not return a value.
//Expressions evaluate to a resultant value. Let’s look at some examples.
fn statements_expressions(){
    // on this example the code into brackets is an expression because return a value
    // the expression x+1 don't have a ';' at the end. If the line have a ';' then is not a expression but is a statement
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
fn five() -> i32 {
   return 5
}



fn main() {
    statements_expressions();
    let x = five();
    println!("The value of x is: {x}");
}

