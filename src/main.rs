/*Every value in Rust is a certain 'data type', this
tells Rust what kind of data is being specified so 
it can know how to work with it. */

//SCALAR TYPES
/* Rust has 4 primary scalar types: integers, floating-point numbers, Booleans, and characters

//Integer Types - a number without a decimal point or fractional component
*/
fn main() {
    let integer: u32 = 123;
    println!("{}", integer);

    another_integer();
}
// above we definean unsigned integer of 32bits, an unsigned integer can only be positive, hence 
//no need to sign it (-/+)

fn another_integer(){
    let integer: i32 = -47;
    println!("{}", integer);
}



