use std::{fs, io::Write, usize};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    
    /// The day to test
    #[arg(short, long, default_value_t = 1)]
    day: u8,

    /// The input file to use.
    #[arg(short, long)]
    input: String,
}

fn day_one(input: &str) {
    let mut result: usize = 0;

    for line in input.lines()  {
        let mut iter = line.chars();  
        let mut str_result = String::new();
        let mut ch: Option<char> = None;

        loop {
            // if len is > 0 then this means we can walk the string from the end.
            if str_result.len() > 0 {
                ch = iter.next_back();
            } else {
                ch = iter.next();
            }

            if let Some(next_char) = ch {
                if next_char.is_digit(10) {
                    str_result.push(next_char);
                    if str_result.len() == 2 {
                        break;
                    }
                }
            } else {
                if str_result.len() == 1  {
                    str_result = str_result.repeat(2);
                }
                break;
            }
        }
    
        let x = str_result.parse::<usize>().unwrap();
        result += x;
        println!("{}", x);
    }
    
    println!("{}", result);
}

fn main() {
    println!("Hello, world!");
    let args = Args::parse();

    // read in the file and pass a reference to the day function.
    let input_file_text = fs::read_to_string(args.input).unwrap();
    day_one(&input_file_text);    

}
