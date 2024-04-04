fn main() {
    let n = 5;
    if n < 0 {
        println!("{} is a negative value", n)
    } else if n > 0 {
        println!("{} is a positive value", n)
    } else {
        println!("{} is zero", n)
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            n / 2
        };
    println!("{} -> {}", n, big_n);    
   
    let mut count = 0;

    println!("Let Count until infinity!");
    loop {
        count +=1;

        if count == 3 {
            println!("THREE");


            continue;
        }

        if count == 5 {
            println!("That is enough! OK ");
            break();
        }

    }

    // FOR AND ITERATORS

    // - iter()
    // This borrows each element of the collection through each iteration. 
    // Thus leaving the collection untouched and available for reuse after the loop.
    let names = vec!["Mubarak", "Muhammad", "Aminu"];
    for name in names.iter() {
        match name {
            &"Mubarak" => println!("There is rustacean among us"),
            _=> println!("Hello {}", name)
        }
    }
    println!("Names: {:?}", names); 

    // into_iter
    // into_iter - This consumes the collection so that on each iteration the exact data is provided.
    //  Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop

    let names = vec!["John", "Juliet", "Frank"];
    for name in names.into_iter() {
        match name {
            "Juliet" => println!("There is an awesome rustacean among us"),
            _=> println!("Hello {}", name)
        }
    }

    // we will not get the names at this point since the iterator
    //  has consume all the collection in the loop, and is no longer 
    // available for reused

   // println!("Names : {:?} ", names)

   // iter_mut

   let mut names = vec!["Bob", "Frank", "Ferris"];
   for name in names.iter_mut() {
        match name {
            &mut "Frank" => println!("There is an awesome rustacean among us whole love Bitcoin"),
            _=> println!("Hello Everyone")
        }
   }

   println!("names: {:?}", names);

   //MATCH
   let number = 13;
   
   println!("Tell me more about {}", number);

   match number {
        1 => println!("One!"),
        2 | 3 | 4 | 5 | 7 | 11 | 13 => println!("This is a prime number"),
        13..=19 => println!("A teen"), // for an inclusive range
        _ => println!("Ain't Specials"),
   }

   let boolean = true;

   let binary = match boolean {
        false => 0,
        true => 1,
   };


   println!("{} -> {}", boolean, binary)




}
