use tokio;
use regex::Regex;
use std::fs::read_to_string;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let filename = "./inputs/day3/input.txt";
  let content = read_to_string(filename).unwrap();
  let lines: Vec<&str> = content.lines().collect();

  let special_symbols = vec!['/', '-', '*', '$', '@', '=', '#', '&', '+', '%'];
  let mut values_to_be_summed: Vec<i32> = Vec::new();
  
  for (index, line) in lines.iter().enumerate() {
    let re = Regex::new(r"\W*(\d+)").unwrap();

    println!("Index: {}", index);

    // Retrieve the next line
    let next_line = if index == (lines.len() - 1) { None } else { Some(lines[index+1]) };
    // Retrieve the previous line unless this is the first line
    let prev_line = if index == 0 { None } else { Some(lines[index-1]) };

    // Iterate over all numeric matches
    for cap in re.captures_iter(line) {
      if let Some(matched) = cap.get(1) {
        // Get the start and end indices of the match
        let start_index = matched.start();
        let end_index = matched.end() - 1;
        
        // Start searching for surrounding special symbols
        
        // If the start index is 0, then just re-use that index. If not look at the previous character.
        let search_start_index = if start_index == 0 { 0 } else { start_index - 1 };
        // If the end index is the last index of the line, then just re-use that index. If not look at the next character.
        let search_end_index = if end_index == (line.len() - 1) { end_index } else { end_index + 1 };

        println!("Found number: {} at indices {}..{}", matched.as_str(), start_index, end_index);

        // These should always exist.
        let prev_char = line.chars().nth(search_start_index);
        let next_char = line.chars().nth(search_end_index);

        println!("Prev char: {:?}", prev_char);
        println!("Next char: {:?}", next_char);

        if special_symbols.contains(&prev_char.unwrap()) {
          values_to_be_summed.push(matched.as_str().parse::<i32>().unwrap());

        } else if special_symbols.contains(&next_char.unwrap()) {
          values_to_be_summed.push(matched.as_str().parse::<i32>().unwrap());

        } else if next_line.is_some() && next_line.unwrap()[(search_start_index)..(search_end_index+1)].chars().any(|c| special_symbols.contains(&c)) {
          println!("Next line chars: {:?}", next_line.unwrap()[(search_start_index)..(search_end_index+1)].chars());

          values_to_be_summed.push(matched.as_str().parse::<i32>().unwrap());

        } else if prev_line.is_some() && prev_line.unwrap()[(search_start_index)..(search_end_index+1)].chars().any(|c| special_symbols.contains(&c)) {
          println!("Prev line chars: {:?}", prev_line.unwrap()[(search_start_index)..(search_end_index+1)].chars());

          values_to_be_summed.push(matched.as_str().parse::<i32>().unwrap());

        } else {
          // Do nothing
        }
      }
    }  
  }

  let sum: i32 = values_to_be_summed.iter().sum();
  // Print the answer to Day 3 Part 1
  println!("Answer: {:?}", sum);

  Ok(())
}
