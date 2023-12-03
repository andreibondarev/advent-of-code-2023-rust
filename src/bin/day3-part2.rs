use tokio;
use regex::Regex;
use std::fs::read_to_string;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let filename = "./inputs/day3/input.txt";
  let content = read_to_string(filename).unwrap();
  let lines: Vec<&str> = content.lines().collect();

  let special_symbols = vec!['*'];
  
  let mut gears: HashMap<String, Vec<i32>> = HashMap::new();

  for (index, line) in lines.iter().enumerate() {
    let re = Regex::new(r"\W*(\d+)").unwrap();

    // Retrieve the next line unless this is the last line
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

        // These should always exist.
        let prev_char = line.chars().nth(search_start_index);
        let next_char = line.chars().nth(search_end_index);

        let part = matched.as_str().parse::<i32>().unwrap();

        if special_symbols.contains(&prev_char.unwrap()) {

          // Find the gear symbol
          if prev_char.unwrap() == '*' {
            let key = format!("{},{}", index, search_start_index);

            match gears.get_mut(key.to_string().as_str()) {
              Some(gear) => {
                  gear.push(part);
              },
              None => {
                  let mut vector: Vec<i32> = Vec::new();
                  vector.push(part);

                  gears.insert(key.clone(), vector);
              }
            };
          }
          
        } else if special_symbols.contains(&next_char.unwrap()) {

          // Find the gear symbol
          if next_char.unwrap() == '*' {
            let key = format!("{},{}", index, search_end_index);

            match gears.get_mut(key.to_string().as_str()) {
              Some(gear) => {
                  gear.push(part);
              },
              None => {
                  let mut vector: Vec<i32> = Vec::new();
                  vector.push(part);

                  gears.insert(key.clone(), vector);
              }
            };
          }

        } else if next_line.is_some() && next_line.unwrap()[(search_start_index)..(search_end_index+1)].chars().any(|c| special_symbols.contains(&c)) {

          // Find the gear symbol
          let idx = next_line.unwrap()[(search_start_index)..(search_end_index+1)].find('*');

          if idx.is_some() {
            let key = format!("{},{}", (index+1), (idx.unwrap() + search_start_index));

            match gears.get_mut(key.to_string().as_str()) {
              Some(gear) => {
                  gear.push(part);
              },
              None => {
                  let mut vector: Vec<i32> = Vec::new();
                  vector.push(part);

                  gears.insert(key.clone(), vector);
              }
            };
          }


        } else if prev_line.is_some() && prev_line.unwrap()[(search_start_index)..(search_end_index+1)].chars().any(|c| special_symbols.contains(&c)) {
          // Find the gear symbol
          let idx = prev_line.unwrap()[(search_start_index)..(search_end_index+1)].find('*');

          if idx.is_some() {
            let key = format!("{},{}", index-1, (idx.unwrap() + search_start_index));

            match gears.get_mut(key.to_string().as_str()) {
              Some(gear) => {
                  gear.push(part);
              },
              None => {
                  let mut vector: Vec<i32> = Vec::new();
                  vector.push(part);

                  gears.insert(key.clone(), vector);
              }
            };
          }

        } else {
          // Do nothing
        }
      }
    }  
  }

  let mut values_to_be_summed: Vec<i32> = Vec::new();

  for (_key, value) in gears.iter() {
    if value.len() == 2 {
      let local_sum = value[0] * value[1];

      values_to_be_summed.push(local_sum);
    }
  }
  
  let sum: i32 = values_to_be_summed.iter().sum();

  // Print the answer to Day 3 Part 2
  println!("Answer: {:?}", sum);

  Ok(())
}
