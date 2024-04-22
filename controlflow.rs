//* Rust like other modern programing languages have conditionals

//* if and if else conditionals work similarly to how they do in javascript with a few differences

//* if statements in rust don't need expressions to be wrapped in parenthesis

//* rust will not cast expressions to booleans for you like javascript does.



fn main() {
    let year = 2192;

    if year >= 1946 && year < 1965{
        println!("Hello, Boomers!");
    } else if year >= 1965 && year < 1981 {
        println!("Hello, Gen X!");
    } else if year >= 1981 && year < 1997 {
        println!("Hello, Millenials!");
    } else if year >= 1997 && year < 2012 {
        println!("Hello, Generation Z!");
    }else {
        println!("Hello, Unknown Generation!")
    };

    ex1();
    pattern();
}

//* these conditionals look exactly like their equvalent in javascript.

//* write conditional expressions that evaluate to a bool in Rust. Not doing so does print a helpful error message 


fn ex1 () {
    let list:  [u8; 0] = [];
    if list.len() > 0 { //* add a > 0 to the conditional expression to fix the problem
        println!("Not an empty list");
    } else {
        println!("Empty list");
    };
}


//* if you write your if else statement like the one above you'll get this error below.
// error[E0308]: mismatched types
//   --> controlflow.rs:36:8     
//    |
// 36 |     if list.len() {      
//    |        ^^^^^^^^^^ expected `bool`, found `usize`

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0308`.

//* there's things in rust you can't doin javascript conditionals. You can use if else branches in expressions in rust.


//*we can refactor our generation example like this:

fn ex2() {
    let year = 1991;

    let generation = if year >= 1946 && year < 1965 {
        "Boomer"
    } else if year >= 1965 && year < 1981 {
        "Generation X"
    } else if year >= 1981 && year < 1997 {
        "Millennial"
    } else if year >= 1997 && year < 2012 {
        "Generation Z"
    }else {
        "Unknown Generation"
    };

    println!("hello, {}", generation);
}



//* This works beacus if else blocks in rust can be reated like expressions.

//* Just like the implicit returns in functions the final expression in a conditional block is considered the result of that expression



//* Loops
//*  the loop keyword is used to run loops in rust. without anything else, a block of code after the loop keyword is an infinite loops

fn loop_test() {
    loop{
        println!("Na");
    }
}



//* to break out of this loop you need the break keyword.



fn loop_test2() {
    let mut counter = 0;//* initializer 

    loop{
        counter = counter + 1; //*  incrementor
        println!("Na");  //* body
        if counter == 10 { //* limiting statement
            break;
        }
    } //* notice the lack of a semicolon here 

    println!("Hey Jude!");
}



//* Just like conditionals you can also return a value from a loop that is used as an expression

fn loop_test3() {
    let mut counter = 0; //* initializer */

    let ten = loop {
        counter = counter + 1; //* incrementor */
        if counter == 10 { //* limiting statement */
            break counter;
        }
    }; //* notice the semicolon here 

    println!("{}", ten);
}




//* While */

//* they are identical in rust to how they work in javascript 

fn while_test () {
    let mut counter = 0; //* intializer 

    while counter <= 10 { //* limiting condition 
        counter = counter + 1; //* incrementor
        println!("Na") //* body
    }

    println!("Hey Jude!");
}



//* for in 
//* you'll find yourself wiring the for in loop regularly in most rust code. you can write a for in loop (also called an iterator loop ) on many types of cllections, the simplest being an arrya.



fn for_in_(){
    for planet in [
        "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune"
    ] {
        println!("{}", planet);
    }
}


//* this is similar to the for each loop in JavaScript. 


//* to make a collection iterable, one needs to implement the IntoIterator trait. With this you can make any custom type iterable *


//* Pattern Matching 
//*JavaScript has the switch operator. Rust has pattern matching whcih is a lot more than just the switch operator offers.

//* The match operator is what you'll use for pattern matching in Rust.


enum Status {
    Connected,
    Disconnected,
    Failure(String),
}

fn pattern(){
    let len = Status::Failure(String::from("Could not contact DHCP server"));
    // let lan = Status::Connected;
    match lan {
        Status::Connected => {
            println!("Connection established")
        }
        Status::Disconnected => {
            println!("Connection lost")
        }
        Status::Failure(error) => {
            println!("Error: {}", error)
        }
    }
}


//* if we run the above code well get this error:


// error[E0004]: non-exhaustive patterns: `Status::Failure(_)` not covered
//    --> controlflow.rs:187:11
//     |
// 187 |     match lan {
//     |           ^^^ pattern `Status::Failure(_)` not covered
//     |
// note: `Status` defined here
//    --> controlflow.rs:182:5
//     |
// 179 | enum Status {
//     |      ------
// ...
// 182 |     Failure(String),
//     |     ^^^^^^^ not covered
//     = note: the matched value is of type `Status`
// help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
//     |
// 193 ~         },
// 194 +         Status::Failure(_) => todo!()
//     |

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0004`. 

//* This is because all matches in rust need to be exhaustive. An exhaustive match refers to the fact that there are branches of code to handle each and every possible variant of the enum. This prevents a whole lot of runtime panics that could occur just because we forgot to handle a variant */


//* matche's exhaustive nature forces a developer to handle edge cases that would otherwise be ignored and leads to robust code. */


//* there is a way to catch all unspecified variants using the _ placeholder like so: */


fn pattern2() {
    let lan = Status::Connected;
    match lan{
        Status::Connected => {
            println!("Connection established")
        }
        _ => {}
    }
}

//*  if let

//*  if let is more or less syntactic sugar everytime you need to handle just a sinlge variant of an enum wihtout needing a placeholder to catch all the rest.


fn if_let() {
    let status = Status::Failure(String::from("Couldn't resolve hostname"));
    match status {
        Status::Failure(error) => {
            println!("Error: {}", error);
        }
        _ => {}
    }
}


/*
*if let will turn this to:
 */ */


 fn if_let2() {
    let status = Status::Failure(String::from("Couldn't resolve hostname"));

    if let Status::Failure(error) = status {
        println!("ERror: {}", error);
    }
 }

 /*
 * we have intitlized an error variable to the value within the Status::Failure variant only if the current status is set to that variant
  */