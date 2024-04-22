//* arrays in rust possess similarities and disimilarities to those in a language like JavaScript.

//? let spidermen = ["Tobey", "Andrew", "Tom"];
//* this is both valid js and rust, we can access the elements of this array using zero based indices just like in JS and that's where it ends.

//? spidermen[1]

//* we can't increase the length of an array or make our array heterogenous.
//* arrays in rust can't be increased in length and it's elements must of the same datatype.


//* if we do this:
//? let spidermen = ["Tobey", "Andrew", "Tom", 5];

//we'll get an error.


fn main(){
    let tubbies: [&str; 4] = ["Tinkey-winkey", "Dipsey", "Laa-laa", "Po"];
	println!("Teletubbies: {:?}", tubbies);
    //* let spidermen = ["Tobey", "Andrew", "Tom", 4];
    // let randoms = ["Sly", true, 4, 'c'];
    // let spidermen: [&str; 3] = ["Tobey", "Andrew", "Tom"];
    // println!("{:?}", spidermen); //* :? makes println! use the Debug trait instead of the default display
}


//* since arrays in rust have a strictly defined length and rust doesn't have types like null and undefined, Rust has no sparse array.

//* you also can't push a new item to the array in Rust. The only way to do that is to use a Vector Vec!


//* rust arrays are useful for homogenous lists with static lengths that don't need to be updated.

//* array are usually annotated like this:
// ?[u32; 3]

//* where the character before the semicolon is the type annotation of the elements of the array the number after the semicolon represents the number of elements in the array.


//* we can also initialize an array with a default value and a set length but we can't forget to make it mutable


//? let mut best_scores: [0; 3];
//? best_scores[0]: 4;
//? best_scores[1]: 34;
//? best_scores[2]: 52;


//* unlike JavaScript's lenght property, the length of an array can be deduced using a method called len


//* desctructuring arrays in rust

//? let [my_score, _, _, ] = best_scores;


//* _ represents a throw away value that is not assigned and can not be used


//* we could do this too
//?let [my_score, ..] = best_scores;

//* the rest operator in Rust is .. not like JavaScript's ..