

// variable binding


fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_an_integer = an_integer;

     println!("An integer: {:?}", copied_an_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // declare first and assign later
    let a_binding;
    {
        let x = 4;

        a_binding = x * x;

        println!("A Binding: {}", a_binding)
    }

    let another_binding;

    another_binding = 4;

    println!("another binding: {}", another_binding);

    println!("Hello, world!");
}
