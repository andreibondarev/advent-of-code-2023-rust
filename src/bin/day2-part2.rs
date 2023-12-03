use std::fs::read_to_string;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let filename = "./inputs/day2/input.txt";

  let mut values_to_be_summed: Vec<i32> = Vec::new();

  for game_line in read_to_string(filename).unwrap().lines() {
    // Split into sets
    let start_index = game_line.find(':').expect("No colon found in string") + 1;
    let sets_string = &game_line[start_index..].trim();

    // Split the string at each semicolon and collect the results into a vector
    let game_sets: Vec<&str> = sets_string
      .split(';')
      .map( |s| s.trim()).collect();

    // Minimum number of red cubes needed
    let mut red_cubes_needed: i32 = 0;
    // Minimum number of green cubes needed
    let mut green_cubes_needed: i32 = 0;
    // Minimum number of blue cubes needed
    let mut blue_cubes_needed: i32 = 0;

    for set in game_sets {
      let number_of_red_cubes = extract_number_of_cubes(set, "red").unwrap_or(0);
      let number_of_green_cubes = extract_number_of_cubes(set, "green").unwrap_or(0);
      let number_of_blue_cubes = extract_number_of_cubes(set, "blue").unwrap_or(0);

      if number_of_red_cubes > red_cubes_needed {
        red_cubes_needed = number_of_red_cubes;
      }

      if number_of_green_cubes > green_cubes_needed {
        green_cubes_needed = number_of_green_cubes;
      }
      
      if number_of_blue_cubes > blue_cubes_needed {
        blue_cubes_needed = number_of_blue_cubes;
      }
    }

    let power = red_cubes_needed * green_cubes_needed * blue_cubes_needed;

    values_to_be_summed.push(power);
  }

  // Sum all calibration values
  let sum: i32 = values_to_be_summed.iter().sum();
  // Print the answer to Day 2 Part 1
  println!("Answer: {:?}", sum);

  Ok(())
}

fn extract_number_of_cubes(game_str: &str, color: &str) -> Option<i32> {
  let regex_pattern = format!(r"(\d+) {}", color);
  let re = Regex::new(&regex_pattern).unwrap();

  re.captures(game_str).and_then(|cap| {
      cap.get(1).and_then(|match_| match_.as_str().parse::<i32>().ok())
  })
}
