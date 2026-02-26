fn main() {
    hello_world();

    let x = hello_world_returns();
    println!("Hello World Returns: {x}");
    let y = hello_world_takes(9);
    println!("Hello World Takes: {y}");
    learn_this_well();
}

//Definining a function with no returns
fn hello_world(){
    println!("Hello world with no return");
}

//Function with a return of a value of type i32 (signed int: -2^n-1 - 2^n-1 - 1)
//Must Declare the type of parameter
fn hello_world_returns ()->i32{
    let mut x = 9;
    x += 1;
    x
}
//Functions which return and take an argument input, 
//The mut keyword is important if you're processing or, altering
fn hello_world_takes (mut x: i32) -> i32{
    x += 1;
    x
}

fn learn_this_well (){
    //A Statement in Rust
    let mut x = 1;
    //An Expression in Rust
    let y = {
        x = 9;
        x += 1;
        x
    };
    println!("This {y} works");

    // Can Expressions Return a Tuple?
    let z = {
        let mut tup : (i32, i32) = (1, 2);
        tup.0 += 1;
        tup
    };
    println!("z's a tuple {} {}", z.0, z.1);

}