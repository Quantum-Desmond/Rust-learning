fn main() {
    println!("start of ownership chapter.");

    // summary: when giving a variable a value,
    // it depends if it's simple data or not (similar
    // to POD in C++); move semantic for most, copy if POD

    // let s = String::from("hello");

    // takes_ownership(s);

    // println!("Second call to s: {}", s);
    // this commented line is invalid as s is no longer in scope
    // it was moved

    // let x = 5;

    // makes_copy(x);

    // println!("Second call to x: {}", x);

    let s1 = given_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

}

fn given_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}



fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
// some_string goes out of scope here, so after this function is called
// the string passed to takes_ownership can no longer be called

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
// here, some_integer goes out of scope, but because it's a Copy
// variable, it was copied into makes_copy and so, the variable
// passed to makes_copy is valid after the function call


