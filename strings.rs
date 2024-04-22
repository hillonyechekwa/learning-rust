//* strings in rust aren't as simple as strings in more loosely type languages like JavaScript and Python.
//* arrays and strings in javascript are mutable and can continuously grow in size
//* you can keep pushing to a string or an array in javascript while mutating it's length
//* This is because strings in javascript are heap allocated and are not fixed in lenght

//* a couple of articles ago we looked at how arrays in rust are fixed in length, this is because arrays in rust are stack allocated and have a fixed length


//* for example:
fn main() {
    let mut nums = [1, 2, 3];
    nums = [1, 2, 3, 4];
}

//*  this code snippet will not compile and will throw the error:
// error[E0308]: mismatched types
//   --> strings.rs:12:12        
//    | 
// 12 |     nums = [1, 2, 3, 4];
//    |            ^^^^^^^^^^^^ expected an array with a fixed size of 3 elements, found one with 4 elements

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0308`.


//* while this exists this doesn't stop us from having a collection  of data that can grow or shrink in rust. This property though is assigned to a different data type called Vec or Vectors. a Vector is a heap allocated data type that can grow and shrink over time.


//* strings are of two types
/*
str: a string slice similar to an array, fixed length and immutable
String: a String, stored as a Vec of u8 used to store UTF-8 data, that's not fixed length
 */

//* str

// when a string literal is written in rust, it's not exactly a String but a String slice.

// fn main() {
//     let greeting = "Hello, World!";
//     println!("{}", greeting);
// }


// this data type is part of rust core and can be annotated as &str

fn main() {
    let greeting: &str = "Hello, World!";
    println!("{}", greeting);
}

// with or without the string annotation, we get the same output.
// in javascript you can create a string literalusing a pair of double or single quotes, or backticks.
// for rust, string literals must always be surrounded by double quotes, single quotes are reserved for single character data types.

//All rust literals can be multiline

