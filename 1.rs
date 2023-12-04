use regex::Regex;

fn extract_numbers(input: &str) -> Vec<u32> {
    // Define a regular expression to match numbers
    let re = Regex::new(r"(?:[1-9]|one|two|three|four|five|six|seven|eight|nine)").expect("Invalid regex");

    // Use the captures_iter method to iterate over all matches
    let matches: Vec<_> = re.find_iter(input).collect();
    
    let mut numbers = Vec::new();
    for mat in matches {
        let matched_str = &input[mat.start()..mat.end()];
        print!("{}\n",matched_str);
        let number = match matched_str.parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                // Handle the case when parsing as a number fails
                parse_written_number(matched_str).unwrap()            }
        };

        numbers.push(number);
    }
    numbers
}
fn parse_written_number(input: &str) -> Option<u32> {
    match input.to_lowercase().as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        // Add more cases for other numbers if needed
        _ => None,
    }
}

fn string_to_vector(s: String) -> Vec<String> {
    // Split the string on line breaks
    let lines = s.split("\n");

    // Convert the iterator of lines to a vector of strings
    let mut result = Vec::new();
    for line in lines {
        result.push(line.to_string());
    }

    result
}



fn get_first_and_last_number_combined(input: &str) -> Option<u32> {
    let numbers = extract_numbers(input);
    if let Some(first_element) = numbers.first() {
        if let Some(last_element) = numbers.last() {
            print!("elements: {},{}\n",first_element,last_element);
           return Some(10*first_element+last_element)
        }
    }
    None
} 

fn get_sum_of_all_numbers(input: Vec<String>) -> u32 {
    let mut result = 0;
    for vec in input {
        let number = get_first_and_last_number_combined(vec.as_str());
        if number != None {
            result = result + number.unwrap()
        }
    }
    result
}


fn main() {
    let content = std::fs::read_to_string("1.txt")
        .expect("file not found!");
    // Parse the string into a vector of integers
    let vec = string_to_vector(content);
    print!("{}",get_sum_of_all_numbers(vec))
}