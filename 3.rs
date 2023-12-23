use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
struct number {
    value: u32,
    pos_hor: i32,
    pos_left: i32,
    pos_right: i32,
}
#[derive(Debug)]
struct symbol {
    pos_ver:i32,
}


fn is_symbol_except_point(c: &char) -> bool {
    !c.is_alphanumeric() && *c != '.'
}

// WARUM GEHT DAS NICHT JARO? Q.Q
// fn extract_numbers_and_symbols(line:Vec<char>,line_number:i32)->(Vec<number>,Vec<symbol>){
//     const RADIX: u32 = 10;
//     let mut numbers: Vec<number> = Vec::new();
//     let mut symbols: Vec<symbol> = Vec::new();
//     let mut cur_number = number{value: 0,pos_hor: line_number,pos_left: -1,pos_right: -1};
    
//     let mut iter = line.iter().clone();

//     for (index, ch) in  iter.enumerate() {
//         println!("{}, {}",index,ch);
//         if ch.is_digit(10) {
//             // If the current character is a number, collect all consecutive digits
//             cur_number.pos_left=index as i32;
//             cur_number.value = ch.to_digit(RADIX).unwrap();
//             let iter2 = iter.clone();
//             for (index2,ch2) in iter2.enumerate()  {
//                 if ch2.is_digit(10) {
//                     println!("{:?}, {}",cur_number,ch2);
//                     cur_number.value = cur_number.value*10 + ch2.to_digit(RADIX).unwrap();
//                     iter.next();
//                 } else {
//                     break;
//                 }
//             }
//             cur_number.pos_right=index as i32;
//             numbers.push(cur_number);
//             cur_number = number {value: 0,pos_hor: line_number,pos_left: -1,pos_right: -1};

            
//         } else if is_symbol_except_point(&ch) {
//             let symbol = symbol{pos_hor: line_number,pos_ver: index as i32};
//             symbols.push(symbol);
//         }

//     }
//     return (numbers,symbols)      
// }

fn extract_numbers_and_symbols(line:Vec<char>,line_number:i32)->(Vec<number>,Vec<symbol>){
        const RADIX: u32 = 10;
        let mut numbers: Vec<number> = Vec::new();
        let mut symbols: HashMap<i32,Vec<u32>> = HashMap::new();
        let mut cur_number = number{value: 0,pos_hor: line_number,pos_left: -1,pos_right: -1};
        
    
        for mut index in 0..line.len() {
            let ch = line[index];
            println!("{}, {}",index,ch);
            if ch.is_digit(10) {
                // If the current character is a number, collect all consecutive digits
                cur_number.pos_left=index as i32;
                cur_number.value = ch.to_digit(RADIX).unwrap();
                for index2 in index+1..line.len()  {
                    let ch2 = line[index2];
                    if ch2.is_digit(10) {
                        cur_number.value = cur_number.value*10 + ch2.to_digit(RADIX).unwrap();
                        println!("{:?}, {}",cur_number,ch2);
                    } else {
                        index = index2;
                        break;
                    }
                }
                cur_number.pos_right=(index-1) as i32;
                numbers.push(cur_number);
                cur_number = number {value: 0,pos_hor: line_number,pos_left: -1,pos_right: -1};
    
                
            } else if is_symbol_except_point(&ch) {
                let symbol = symbol{pos_hor: line_number,pos_ver: index as i32};
                symbols.push(symbol);
            }
    
        }
        return (numbers,symbols)      
    }

fn main() {
    let content = std::fs::read_to_string("3.txt")
        .expect("file not found!");
    // Parse the string into a vector of integers
    let split_content = content.lines();
    for (index,line) in split_content.enumerate() {
        print!("{:?}",extract_numbers_and_symbols(line.chars().collect(),index as i32));
    }
    
}