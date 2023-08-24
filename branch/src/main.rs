fn main() {
    // let num1 = 7;
    // if num1 < 5 {
    //     println!(" Condition is meet")
    // }else {
    //     println!("Condtion is not meet")
    // }
    let condition = true;
    let number = if condition { 5 } else { 6 }; // assign the result of an IF Expression to a variable
    println!("The value of number is: {number}");

}



// Control Flow
    // The most common constructs that let you control the flow of execution of Rust code are if expressions and loops.
    
    // if Expression
        // An if expression allows you to branch your code depending on conditions.
        // provide a condition, if the condtion is true, run this, else do not run the block 
        // Blocks of code associated with the conditions in if expressions are sometimes called arms, just like the arms in match
        // It’s also worth noting that the condition in this code must be a bool. If the condition isn’t a bool, we’ll get an error
        // Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide if with a Boolean as its condition.
        // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable,
// Repetition with Loops
    // Rust provides several loops, which will run through the code inside the loop body to the end and then start immediately back at the beginning.
    // Rust has three kinds of loops: loop, while, and for.

