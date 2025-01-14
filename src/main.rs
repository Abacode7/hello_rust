fn main() {
    println!("Hello, world!");

    // Rust Data Types
    // Integer
    // Integer data type could be signed (i8, i16, i32, i64, i128)  
    // or unsigned (u8, u16, u32, u64, u128)
    let i: u8 = 12;
    let j: u8 = 58;

    println!("Integer addition i + j => {}", i + j);

    // Character and String
    // Character store a single character in a string,
    // while Strings store words
    let input_char: char = 'z';
    let input_string1: String = String::from("string 1");
    let input_string2: &str = "string 2";

    println!("Character and string values inputChar, inputString1 and inputString2 => {}, {}, {}", input_char, input_string1, input_string2);

    // Boolean
    // Bolean values could be true or false
    let a: bool = true;
    let b: bool = false;

    println!("Boolean variables a => {} and b => {}", a, b);

    // Rust Data Operations
    println!("Logical AND: a & b => {}", a & b);
    println!("Logical OR: a | b => {}", a | b);
    println!("Logical XOR: a ^ b => {}", a ^ b);

    println!("Equality: a == b => {}", a == b);
    println!("Comparison: a > b => {}", a > b);
}
