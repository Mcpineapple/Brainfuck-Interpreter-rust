
use std::fs::File;
use std::io::{self, prelude::*};
use::std::path::Path;



fn get_file_as_str() -> String {

    println!("Input file name: ");

    let mut path = String::new();

    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line.");

    if let Some('\n')=path.chars().next_back() {
        path.pop();
    }
    
    let path = Path::new(path.as_mut_str());
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut program = String::new();
    match file.read_to_string(&mut program) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("Executing :\n{}", program),
    };

    program
}

fn main() {

    let program : String = get_file_as_str();

    let program : Vec<char> = program.chars().collect();

    let program_size : usize = program.len();

    let mut pointer : usize = 0;

    let mut array : [u8; 30_000] = [0; 30_000];

    let mut array_pointer : usize = 0;

    let mut loop_memory : Vec<usize> = Vec::new();

    let mut character : char;

    loop {
        let character = program[pointer]; 

        match character {

            '+' => {
                match array[array_pointer] {
                    255 => array[array_pointer] = 0,
                    _ => array[array_pointer] += 1,
                };
            },

            '-' => {
                match array[array_pointer] {
                    0 => array[array_pointer] = 255,
                    _ => array[array_pointer] -= 1,
                };
            },

            '>' => {
                match array_pointer {
                    29_999 => array_pointer = 0,
                    _ => array_pointer += 1,
                };
            },

            '<' => {
                match array_pointer {
                    0 => array_pointer = 29_999,
                    _ => array_pointer -= 1,
                };
            },

            '.' => {
                let array_char = array[array_pointer] as char;

                print!("{}", array_char);
                
            },

            ',' => {
                let input: Option<u8> = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u8);
                array[array_pointer] = match input {
                    Some(input_char) => input_char,
                    None => panic!("Input error."),
                };
            },

            '[' => {
                if array[array_pointer] == 0 {
                    let mut brackets = 1;
                    while (pointer < program_size) && (brackets > 0) { 
                        pointer += 1;
                        let character = program[pointer]; 
                        match character {
                            '[' => brackets +=1,
                            ']' => brackets -=1,
                            _ => (),
                        };
                    }
                } else {
                    loop_memory.push(pointer);
                }

            },

            ']' => {
                if array[array_pointer] == 0 {
                    loop_memory.pop();
                } else {
                    pointer = match loop_memory.pop() {
                        Some(point) => point,
                        _ => panic!("Vector Crash."),
                    };
                    loop_memory.push(pointer);
                }
            },

            _ => (), 
        }

        pointer += 1;

        if pointer >= program_size {
            return
        }
    }
}
