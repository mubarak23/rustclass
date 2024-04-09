use std::fs::File;
// use std::io::ErrorKind;
// use std::io::{self, Read, ErrorKind}
use std::io::{self, Error, ErrorKind};
use std::io::Read;




fn read_username_from_file() -> Result<String, Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) =>  {
             Ok(username)
            //  println!("value from the file {}", username)
        },
        Err(e) => Err(e),
    }

}

fn read_username_from_file_min() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {

   //  panic!("Something wrong happen here");
    println!("Hello, world!");


    // panic that occur due to bug in our code
    let v = vec![2,4,6,8,9];

  //   v[99];

  // Recoverable Errors with Result
    enum Result<T, E> {
        Ok(T),
        Err(E)
    }
    
  // T represents the type of the value that will be returned in a success case within the Ok variant
  // E represents the type of the error that will be returned in a failure case within the Err variant.

  let greeting_file_result = File::open("hello.txt");

//   let greeting_result = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {:?}", error),
//   };


  // Matching on Different Errors
    let greeting_result = match greeting_file_result {
        Ok(file) => {
            println!("File is found here");
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e), 
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }   
        },
    };

    read_username_from_file();
    read_username_from_file_min();

    let greeting_file = File::open("hellos.txt")?;


}

// Generic Types, Traits, and Lifetimes