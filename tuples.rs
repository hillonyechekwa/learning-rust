//* tuples are another complex primitive compound type in Rust. JavaScript doesn't have tuples.

//* Tuples are a collection of multiple values of various types but with a fixed length.


fn main() {
    let mut product = ("Iphone 13 Pro Max", 1399, true); //* product, cost , availability
    //* can be set to
    product("Samsung Galaxy S22 Ultra", 1599, false);
    //* types of each value at each position must remain the same. 


    //* there's a special type of tuple in rust called the unit tuple and it's represented as
    ()
    //* it has no value and is what is returned by functions that return nothing in rust.


    //* destruturing tuples in rust
    let (_, _, is_available) = product;
}