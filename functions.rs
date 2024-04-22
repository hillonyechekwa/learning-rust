//* by now it should be obvious that the fn keyword is used to declear a function in rust

fn main() {
    // let result = sum(10, 5);
    // println!("Hello World");
    // println!("Result {}", result);
    let result = sum(10, 5);
    //* remember the unit tuple (). This is a function that returns that as a value.
}

fn sum(x, y) {
    return x + y;
}



//* let's write a function that not the main function

//* fn sum(x,y){
    //* this function will complie with some errors.

    //* this is because all functions with parameters and a return value need to be annotated.
    //* return X + y;
//* }


// fn sum(x: u64, y: u64) -> u64 {
//     //* here, we've annotated the function's parameters and also it's return value.
//     return x + y
// }

//* rust allows us to rewrite our sum function to look like this:
// fn sum(x: u64, y: u64) -> u64 {
//    //* every statement in a rust function must end with a semicolon, an expression doesn't end with a semicolon.
//    //? x + y 

//     //* rust allows us to implicitly return expressions in a function.
// }

//* just like javascript, rust has anonymous functions, they're also called lambda functions and closures.

//? |x| {x}

//* this function returns the first and only parameter passed to it

//? x => x

//* in rust, paramters for an anonymous fn must be surronded by \. let's rewrite our sum function now
// ?let sum = |x,y| x + y;

//* an anonymous function can be called just the same way a classic rust function is called. The difference here is annonymous functions don't need type annotations, the types are inferred.



