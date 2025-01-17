use std::{collections::{HashMap, HashSet}, ops::Add, vec};

fn main() {
    println!("Hello, Rust!");
    println!();

    // Rust Data Types
    // Integer
    // Integer data type could be signed (i8, i16, i32, i64, i128)  
    // or unsigned (u8, u16, u32, u64, u128)
    let i: u8 = 12;
    let j: u8 = 58;

    println!("Integer addition i + j => {}", i + j);
    println!();


    // Character and String
    // Character store a single character in a string,
    // while Strings store words
    let input_char: char = 'z';
    let input_string1: String = String::from("string 1");
    let input_string2: &str = "string 2";

    println!("Character and string values inputChar, inputString1 and inputString2 => {}, {}, {}", input_char, input_string1, input_string2);
    println!();


    // Boolean
    // Bolean values could be true or false
    let a: bool = true;
    let b: bool = false;

    println!("Boolean variables a => {} and b => {}", a, b);
    println!();



    // Rust Data Operations
    println!("Logical AND: a & b => {}", a & b);
    println!("Logical OR: a | b => {}", a | b);
    println!("Logical XOR: a ^ b => {}", a ^ b);

    println!("Equality: a == b => {}", a == b);
    println!("Comparison: a > b => {}", a > b);
    println!();



    // Sequence Data Types
    // Tuples
    let date = get_date(String::from("12/03/11 GMT+1"));

    let (day, month, year, timezone) = date.clone();

    println!("Date: {}, {}, {}, {}", day, month, year, timezone);

    let our_date: (u32, u32, u32, String) = (12, 03, 11, String::from("GMT+4"));

    println!("Date: {}, {}, {}, {}", our_date.0, our_date.1, our_date.2, our_date.3);
    println!();


    // Arrays
    let num_array = [10, 9, 8, 7, 6];

    for i in 0..num_array.len(){
        println!("Index: {} and Value: {}", i, num_array[i])
    }

    for value in num_array.iter(){
        println!("Element: {}", value)
    }
    println!();



    // Loops
    // For Loop
    let matrix = [[5, 10, 15], [20, 25, 30]];

    let rows = matrix.len();
    let columns = matrix[0].len();

    for i in 0..rows {
        for j in 0..columns{
            println!("Element in row {} and column {} is: {}", i, j, matrix[i][j]);
        }
    }
    println!();

    // While loop
    let mut row = 0;
    let mut col;

    while row < rows {
        col = 0;
        
        while col < columns {
            println!("Element in row {} and column {} is: {}", row, col, matrix[row][col]);
            col += 1;
        }
        row += 1;
    }
    println!();

    
    // Loop loop
    let mut row_index = 0;
    let mut col_index = 0;

    loop {
        println!("Element in row {} and column {} is: {}", row_index, col_index, matrix[row_index][col_index]);

        col_index += 1;

        if col_index == columns {
            col_index = 0;
            row_index += 1;

            if row_index == rows {
                break;
            }
        }
    }
    println!();



    // Struct User Defined Type
    let mut surfer = Surfer {
        height: 6,
        weight: 75,
        max_wave_height: 0,
        board_name: String::from("Small shark!")
    };

    surfer.say_aloha();
    println!(
        "Pre Surfing: Height: {}; Weight: {}; Wave Height: {}, Board Name: {}",
        surfer.height, surfer.weight, surfer.max_wave_height, surfer.board_name);
    
    surfer.ride_wave(10);
    surfer.change_board_name(String::from("Big shark!"));

    println!(
        "Post Surfing: Height: {}; Weight: {}; Wave Height: {}, Board Name: {}",
        surfer.height, surfer.weight, surfer.max_wave_height, surfer.board_name);
    println!();



    // Enum Types
    let mut balance: u32 = 50;
    println!("Initial balance: {}", balance);
    pay(&mut balance, 0);
    pay(&mut balance, 100);
    deposit(&mut balance, 0);
    deposit(&mut balance, 50);
    pay(&mut balance, 100);
    println!("Final balance: {}", balance);
    println!();



    // Control Flow
    // If Structure
    // FizzBuzz problem - If a number is divisible by 3, write Fizz
    // If a number is divisible by 5, write Buzz
    // If a number is divisible by 3 and 5, write FizzBuzz, for 1 to the input
    println!("Fizz Buzz for input {}: ", 15);
    fizz_buzz(15);
    println!();

    println!("Fizz Buzz for input {}: ", 10);
    fizz_buzz(10);
    println!();

    println!("Fizz Buzz for input {}: ", 9);
    fizz_buzz(9);
    println!();

    println!("Fizz Buzz for input {}: ", 8);
    fizz_buzz(8);
    println!();

    
    
    // Advanced Rust Concept
    // Vectors
    let _vec_1: Vec<u32> = Vec::new();
    let mut vec_2 = vec![1, 2, 3, 4, 5];

    vec_2.push(6);
    for element in vec_2.iter(){
        println!("Element: {}", element);
    }

    let popped_element = vec_2.pop().unwrap();
    println!("Popped Element: {}", popped_element);
    println!("New Length: {}", vec_2.len());
    println!();


    
    // Sets
    let mut set: HashSet<u32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);

    set.insert(1);
    for element in set.iter(){
        println!("Element: {}", element);
    }
    println!();

    set.remove(&1);
    let contains = set.contains(&1);
    println!("Contains 1: {}", contains);
    println!();

    
    // Hash Maps
    let mut map: HashMap<u32, String> = HashMap::new();
    map.insert(1, String::from("One"));
    map.insert(2, String::from("Two"));
    map.insert(3, String::from("Three"));

    for (key, value) in map.iter(){
        println!("Key: {} and Value: {}", key, value);
    }
    println!();
    
    for key in map.keys(){
        println!("Key: {}", key);
    }
    println!();

    for value in map.values(){
        println!("Value: {}", value);
    }
    println!();

    let value = map.get(&1).unwrap();
    println!("Value for key 1: {}", value);
    println!();

    // Pattern Matching
    // Pattern matching in Tuples
    let tuple = (1, 2, 3);
    match tuple {
        (x, y, 3) => {
            println!("x: {}, y: {}", x, y);
        },
        _ => println!("No match")
    }

    // Pattern matching in Arrays
    let array = [1, 2, 3, 4, 5  ];
    match &array[..]{
        [first, second, rest @ ..] => {
            println!("First: {}, Second: {}, Rest: {:?}", first, second, rest);
        },
        _ => println!("No match")
    }

    // Pattern matching in Functions
    let arr = [1, 2, 3, 4, 5];
    match find_element(&arr, 2){
        Some(index) => println!("Element found at index: {}", index),
        None => println!("Element not found")
    }

    match find_element(&arr, 10){
        Some(index) => println!("Element found at index: {}", index),
        None => println!("Element not found")
    }
    println!();

    match divide(10, 2){
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error)
    }
    
    match divide(10, 0){
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error)
    }
    println!();


    // Closures
    let add = |a: i32, b: i32| -> i32 {a + b};
    let base: u32 = 10;
    let pow = |exp: u32| -> u32 {base.pow(exp as u32)};
    println!("Addition: {}", add(10, 20));
    print!("Power: {}", pow(2));
    println!();


    // Generics and Traits
    fn max<T: PartialOrd>(a: T, b: T) -> T {
        if a > b {
            a
        }else {
            b
        }
    }

    println!("Max of 10 and 20: {}", max(10, 20));
    println!();

    fn addition<T: Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }
    println!("Addition of 10 and 20: {}", addition(10, 20));
    println!()

}

fn get_date(date: String) -> (u32, u32, u32, String){
    let day: u32 = date[0..2].parse().ok().unwrap();
    let month: u32 = date[3..5].parse().ok().unwrap();
    let year: u32 = date[6..8].parse().ok().unwrap();
    let timezone = String::from(&date[9..]);

    (day, month, year, timezone)
}


struct Surfer {
    pub height: u32,
    pub weight: u32,
    pub max_wave_height: u32,
    pub board_name: String,
}

impl Surfer {
    fn say_aloha(&self){
        println!("Alahoa!!")
    }

    fn ride_wave(&mut self, wave_height: u32){
        if wave_height > self.max_wave_height{
            self.max_wave_height = wave_height;
            println!("Yoohoo...new wave height record set");
        }else {
            println!("Awesome!...always a pleasure");
        }
    }

    fn change_board_name(&mut self, new_board_name: String){
        self.board_name = new_board_name;
    }
}

#[derive(Debug)]
enum Errors {
    InvalidAmount,
    NotEnoughFunds,

}

fn deposit(balance: &mut u32, amount: u32){
    if amount <= 0 {
        println!("{:?}", Errors::InvalidAmount);
        return;
    }

    *balance += amount;
}

fn pay(balance: &mut u32, amount: u32){
    if amount == 0 {
        println!("{:?}", Errors::InvalidAmount);
        return;
    }

    if amount > *balance {
        println!("{:?}", Errors::NotEnoughFunds);
        return;
    }

    *balance -= amount;
}


fn fizz_buzz(input: u32) {
    for i in 1..input+1{
        if i % 15 == 0 {
            println!("FizzBuzz");
        }else if i % 3 == 0 {
            println!("Fizz");
        }else if i % 5 == 0 {
            println!("Buzz");
        }else {
            println!("{}", i);
        }
    }
}

fn find_element(array: &[i32], element: i32) -> Option<usize>{
    for (index, value) in array.iter().enumerate(){
        if *value == element {
            return Some(index);
        }
    }

    None
}


fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    }else {
        Ok(a / b)
    }
}