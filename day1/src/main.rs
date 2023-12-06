use fs;
use std::collections::HashMap;
fn main() {
let input = parseInput("./src/data/test.txt");

// let input = parseInput("./src/data/main.txt");
let test = calculate(input);

println!("Out:{test}");
}


fn parseInput(path: &str) -> String{

    match std::fs::read_to_string(std::path::Path::new(path)) {
        Ok(contents) => {
            println!("File: {}", contents);
            return contents;
        }
        Err(e) => {
            eprintln!("Error in reading file: {}", e);
            return "N/A".to_string();
        }
    }
}

fn calculate(input: String) -> i32{

    let nums_map: HashMap<&str, char> = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]
    .iter()
    .cloned()
    .collect();

    let mut out: i32 = 0;
    for l in input.lines(){
        let mut num: String = "".to_string();
        let mut word: String = "".to_string();
        for c in l.chars(){
           word.push(c);
           if (c as u8 >= 48 && c as u8 <= 57){
                num.push(c);
                word.clear();
                break;
           } 
           else if (nums_map.contains_key(word.to_lowercase().as_str())){
               num.push(*nums_map.get(word.as_str()).unwrap());
               word.clear();
               break;
           }

        }
        for c in l.chars().rev(){
           word.push(c);
           if (c as u8 >= 48 && c as u8 <= 57){
                num.push(c);
                word.clear();
                break;
           } 
           else { 
                let mut s = String::new();
                    for i in word.chars(){
                if (nums_map.contains_key(word.chars().rev().collect::<String>().to_lowercase().as_str())){
                        s.push(i);
                        match nums_map.get(s.to_lowercase().as_str()) {
                            Some(w) => {
                                num.push(*w);
                            }
                            None => {
                                println!("No value found in nums_map for {}", word);
                            }
                        }
                    }
                       break;
                       word.clear();
                   }
            }

        }
    

        let num_temp: Result<i32, _> = num.parse();
        // Handle the parsing result
        match num_temp{
            Ok(parsed_num) => {
                println!("Parsed number: {}", parsed_num);
                // Use parsed_num as a u32 here
                out += parsed_num;
            }
            Err(e) => {
                println!("Error parsing: {}", e);
            }
        }

    }        

    return out;
    
}
