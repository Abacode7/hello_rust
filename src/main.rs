fn main() {
    println!("Hello, world!");
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