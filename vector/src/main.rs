
fn main() {
    let arr: [u8; 3] = [1, 2, 3];
    
    let v:Vec<u8> = Vec::from(arr);
    is_vec(v);

    let v:Vec<u8> = vec![1, 2, 3];
    is_vec(v.clone());

    // vec!(..) and vec![..] are same macros, so
    let v: Vec<u8> = vec!(1, 2, 3);
    is_vec(v.clone());
    
    // In code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE Vec::new and `for` to rewrite the below code 
   //  let v1: Vec<[u8; 3]> = vec!(arr);
    let mut v1: Vec<u8> = Vec::new();
    
    is_vec(v1.clone());
 
    for i in &v {
        v1.push(*i)
    } 
 
    assert_eq!(v, v1);

    println!("Success!");

     // Array -> Vec
    // impl From<[T; N]> for Vec
    let arr = [1, 2, 3];
    let v1: Vec<i32> = Vec::from(arr);
    let v2: Vec<i32> = arr.into();
 
    assert_eq!(v1, v2);
 
    
    // String -> Vec
    // impl From<String> for Vec
    let s: String = "hello".to_string(); // Vec<u8>
    let v1: Vec<u8> = s.into();

    let s = "hello".to_string();
    let v2:Vec<u8> = s.into_bytes();
    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s = "hello";
    let v3: Vec<u8> = Vec::from(s);
    assert_eq!(v2, v3);

    // Iterators can be collected into vectors
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);

    println!("Success!");

}

fn is_vec(v: Vec<u8>) {}



// FIX the error and IMPLEMENT the code
// fn main() {
//     let mut v = Vec::from([1, 2, 3]);
//     for i in 0..5 {
//         println!("{:?}", v.get(i)) 
//     }

//     for i in 0..5 {
//        // IMPLEMENT the code here...
//        match v.get(i) {
//            Some(e) => v[i] = e + 1,
//            None => v.push(i + 2)
//        }
//     }
    
//     assert_eq!(v, vec![2, 3, 4, 5, 6]);

//     println!("Success!");
// }

// The capacity of a vector is the amount of space allocated for any future elements that will be added onto the vector.
//  This is not to be confused with the length of a vector, which specifies the number of actual elements within the 
//  vector. If a vector’s length exceeds its capacity, its capacity will automatically be increased, but its elements
//   will have to be reallocated.

// the length must not pass the capacity otherwise a reallocation will occur

