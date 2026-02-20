//This is a single line Comment
//Entry Point for Execution
fn main() {
    //Use _ at the start of the var name to mark it unused.
    //Immutable variable
    let _imut = 0;
    //Mutable variable
    let mut _mut = 1;
    //Shadow Variable
    let mut _mut = 2.0;

    /*
    Defining Variables
    Architecture Specific Signed Integers
    Capacity: -2^n-1 to 2^n-1 -1
    */
    let _int_i8: i8 = 8;  // -128 to 127
    let _int_i16: i16 = 16;
    let _int_i32: i32 = 32;
    let _int_i64: i64 = 64;
    let _int_i128: i128 = 128;

    /*
    Architecture Specific Unsigned Integers
    Capacity: 0 - 2^n-1
    */
    let _uint_u8: u8 = 8; // 0 to 255
    let _uint_u16: u16 = 16;
    let _uint_u32: u32 = 32;
    let _uint_u64: u64 = 64;
    let _uint_i128: u128 = 128;

    //Floating Point Types
    let _f_64: f64 = 2.0;  //More Precision
    let _f_32: f32 = 3.0;


    /*
    Compound Types: Tuples and Arrays
    Defining Tuples
    Use tuples to group together different types of variables.
    Note: These are immutable.
     */
    let tup : (i32, i64, f32) = (400, 20, 2.0);
    println!("Accessing second: {}", tup.1);
    //Unpacking a Tuple
    let (_a, b, _c) = tup;
    println!("Accesing the second: {}", b);
    //Modifying a Tuple
    let mut m_tup : (i32, f64, u8) = (-2, 2.0, 3);
    m_tup.1 += 2.5;
    println!("Modified the second: {}", m_tup.2);\

    /*
    Defining Arrays 
    Same data type allocated in stacks
    */
    let names = ["A", "B", "C", "D"];

    //Defining Array with Type and Fixed Size
    let def_1: [i32; 5] = [1,2,3,4,5];
    //Concise defintion for Duplicates [value, repeat]
    let def_2: [3; 5];
}

