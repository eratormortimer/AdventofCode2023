use regex::Regex;


fn smallest_game_possible(content: Vec <&str>) -> u32{
    let pattern = Regex::new(r"(\d+) (\w+)").expect("Invalid regex");
    let mut result: u32 = 0;
    for game in content {
        let mut allowed_thresholds = vec![("green", 0), ("red", 0), ("blue", 0)];
        let sets : Vec<&str>= game.split(":").collect::<Vec<_>>()[1].split(";").collect();
        for set in sets {
            let dice: Vec<&str> = set.split(",").collect();
            for pair in dice {
            if let Some(captures) = pattern.captures(pair) {
                // Extract the number and keyword
                let number: u32 = captures[1].parse().unwrap();
                let keyword = &captures[2];
                if let Some(index) = allowed_thresholds.iter().position(|&(col, _)| col == keyword) {
                    // Update the threshold if the number in the string is larger
                    if number > allowed_thresholds[index].1 {
                        allowed_thresholds[index].1 = number;
                    }
                } else {
                }
            }
            }
        }
        result += allowed_thresholds.iter().map(|&(_, threshold)| threshold).product::<u32>();
    }
    result
}
fn game_is_possible(content: Vec <&str> ,red:u32, green: u32, blue: u32) -> u32 {
    let mut result: u32 = 0;
    let pattern = Regex::new(r"(\d+) (\w+)").expect("Invalid regex");
    let allowed_thresholds: [(&str, u32); 3] = [("green", green), ("red", red), ("blue", blue)];
    'outer: for line in content {
        println!("{}",line);
        let game: u32 = line.split(":").collect::<Vec<_>>()[0].split(" ").collect::<Vec<_>>()[1].parse().unwrap();
        let sets : Vec<&str>= line.split(":").collect::<Vec<_>>()[1].split(";").collect();
        for set in sets {
            let dice: Vec<&str> = set.split(",").collect();
            for pair in dice {
            if let Some(captures) = pattern.captures(pair) {
                // Extract the number and keyword
                let number: u32 = captures[1].parse().unwrap();
                let keyword = &captures[2];
                match allowed_thresholds.iter().find(|&&(kw, _)| kw == keyword) {
                    Some(&(_, threshold)) if number > threshold => {
                        continue 'outer;
                    }
                    _ => {
                    }
                }
            }
            }
        }
        result+=game;
    }
    result
}
fn main() {
    let content = std::fs::read_to_string("2.txt")
        .expect("file not found!");
    // Parse the string into a vector of integers
    let split_content = content.split("\n").collect();
    print!("{:?}",split_content);
    // let result1 = game_is_possible(split_content,12,13,14);
    // println!("{}",result1);
    println!("\n{:?}",smallest_game_possible(split_content));
}