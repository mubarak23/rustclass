fn main() {
    let x = 5u32;

    let y = {
        let x_square = x * x;

        let x_cube = x_square * x;

        x_cube + x_square
    };

    let z = {
        2 * x;
    };


    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    println!("Hello, world!");
}

// In terms of expressions in Rust:

//     An expression is a piece of code that produces a value. In Rust, expressions do not end with semicolons.
//     A block expression ({}) in Rust evaluates to the value produced by the last expression within the block.
//     The value of the last expression in a block is automatically returned unless a semicolon (;) is used to suppress the return value.
//     In the case of y, the block returns the value of x_cube + x_square.
//     In the case of z, the block does not produce a value because the statement 2 * x; is a computation without assignment or usage, so z remains uninitialized.

// Overall, y will hold the calculated value based on the operations inside its block, while z will remain uninitialized and won't print any meaningful value.
