
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPressed(char),
    Paste(String),
    Click {x: i64, y:i64}
}

enum VeryVorseWorksOnNumbers {
    Add,
    Substract
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}


fn inspect (event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page Loaaded"),
        WebEvent::PageUnload => println!("Page Unload"),
        WebEvent::KeyPressed(c) => println!("Press \"{}\".", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click{x, y} => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

fn inspect_aliases (event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page Loaaded via type aliases"),
        WebEvent::PageUnload => println!("Page UnLoaaded via type aliases"),
        WebEvent::KeyPressed(c) => println!("Press via aliases \"{}\".", c),
        WebEvent::Paste(s) => println!("pasted via aliases \"{}\".", s),
        WebEvent::Click{x, y} => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

fn main() {
   let pressed = WebEvent::KeyPressed('x');
       // `to_owned()` creates an owned `String` from a string slice.
   let pasted = WebEvent::Paste("My Text Work".to_owned());
   let click = WebEvent::Click { x: 4, y: 6};
   let loadpage = WebEvent::PageLoad;
   let unload = WebEvent::PageUnload;

   // pass the enum variant into the diffirent functions
   inspect(pressed);
   inspect(loadpage);
   inspect(unload);
   inspect(click);
   inspect(pasted);

   println!("Type aliases on enum variants");

   type Operations = WebEvent;

   let press_ops = Operations::KeyPressed('Y');
   let paste_ops = Operations::Paste("My Text Work via Types Aliases".to_owned());
   let load_ops = Operations::PageLoad;
   let un_looad_ops = Operations::PageUnload;

   inspect_aliases(press_ops);
   inspect_aliases(paste_ops);
   inspect_aliases(load_ops);
   inspect_aliases(un_looad_ops)


    let add_operation = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
    let subtract_operation = VeryVerboseEnumOfThingsToDoWithNumbers::Subtract;

    let result_add = add_operation.run(5, 3);
    let result_subtract = subtract_operation.run(5, 3);

    println!("Result of addition: {}", result_add); // Output: 8
    println!("Result of subtraction: {}", result_subtract);

}

// Summary:

// 1. **Enums**: The code demonstrates the use of enums (`WebEvent` and `VeryVerboseEnumOfThingsToDoWithNumbers`). Enums allow you to define a type by enumerating its possible values.

// 2. **Variants**: Enums can have different variants representing different states or types of data. For instance, `WebEvent` has variants like `PageLoad`, `PageUnload`, `KeyPressed`, `Paste`, and `Click`.

// 3. **Associated Data**: Enums can carry associated data with each variant. For example, `KeyPressed` variant carries a character, `Paste` variant carries a string, and `Click` variant carries coordinates.

// 4. **Pattern Matching**: The `match` keyword is used extensively to pattern match on enum variants. This allows different behaviors or actions to be executed based on the variant of the enum.

// 5. **Type Aliases**: The code also demonstrates the usage of type aliases. Type aliases provide alternative names for existing types. In this code, `Operations` is an alias for the `WebEvent` enum.

// 6. **Function Implementation for Enums**: The `VeryVerboseEnumOfThingsToDoWithNumbers` enum has associated functions (`run`) implemented for each variant. These functions perform operations like addition and subtraction based on the enum variant.

// 7. **Usage in Main**: In the `main` function, various instances of enums are created and passed to functions for inspection. Additionally, arithmetic operations are performed using the `VeryVerboseEnumOfThingsToDoWithNumbers` enum.

// 8. **Result Display**: Finally, the results of arithmetic operations are displayed, showing the outcomes of addition and subtraction performed using the enum variants.

// In summary, this code showcases the versatility of enums in Rust, including the ability to define different variants with associated data, perform pattern matching, utilize type aliases for convenience, and implement functions for enum variants.