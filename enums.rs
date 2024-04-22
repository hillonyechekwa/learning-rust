//* Enums are a set of named constats that represent distinct elements of a collection.


//* enums are in rust look like this


#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}


fn main() {
    let red = Color::Red; //* set to an instance of red from the Color enum
    let lavender = Color:Rgb(230, 230, 250);
    let lilac = Color::Hsl(300, 26, 71);
    println!("Lilac is {:?}", lilac);)
    println!("{:?} is not Hill's favorite color.", lavender);
    println!("Apples are not {:?}", red)
}


//* the :: operator is used to access an enums values.
//* #[derive(Debug)] is used to tell the compiler that we'll like to automatically generate the debug trait for the Color enum.

//* the values of the Color enum are more appropriately referred to as variants of the enum.


//* variants can also have values associated with them.

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
}


//* with this we could add an Hsl Value

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Hsl(u16, u8, u8),
}

//* enum variants can have values of different types too. You can even nest them and include enums in enums.


//* exercise:
//* properly represent the values of the HSL variant with a percentage enum and a degreee enum.

//* there's a lot more to enums than these. In a near future post we'll look at the Option enum.