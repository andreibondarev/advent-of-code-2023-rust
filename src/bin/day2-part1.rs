use tokio;
use std::fs::read_to_string;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let filename = "./inputs/day2/input.txt";

  // Bag loaded with these cubes:
  let red_cubes: i32 = 12;
  let green_cubes: i32 = 13;
  let blue_cubes: i32 = 14;

  let mut possible_game_ids: Vec<i32> = Vec::new();

  for game_line in read_to_string(filename).unwrap().lines() {
    // Extract game number
    let game_number = extract_game_number(game_line);

    // Split into sets
    let start_index = game_line.find(':').expect("No colon found in string") + 1;
    let sets_string = &game_line[start_index..].trim();

    // Split the string at each semicolon and collect the results into a vector
    let game_sets: Vec<&str> = sets_string
      .split(';')
      .map( |s| s.trim()).collect();

    // Print the results
    let mut this_game_is_possible = true;

    for set in game_sets {
      let number_of_red_cubes = extract_number_of_cubes(set, "red").unwrap_or(0);
      let number_of_green_cubes = extract_number_of_cubes(set, "green").unwrap_or(0);
      let number_of_blue_cubes = extract_number_of_cubes(set, "blue").unwrap_or(0);

      if number_of_red_cubes > red_cubes || number_of_green_cubes > green_cubes || number_of_blue_cubes > blue_cubes {
        this_game_is_possible = false;
      }
    }

    if this_game_is_possible {
      possible_game_ids.push(game_number.unwrap());
    }
  }

  // Sum all calibration values
  let sum: i32 = possible_game_ids.iter().sum();
  // Print the answer to Day 2 Part 1
  println!("Answer: {:?}", sum);

  Ok(())
}

fn extract_game_number(game_str: &str) -> Option<i32> {
  let re = Regex::new(r"Game (\d+)").unwrap();
  re.captures(game_str).and_then(|cap| {
      cap.get(1).map(|match_| match_.as_str().parse::<i32>().ok()).flatten()
  })
}

fn extract_number_of_cubes(game_str: &str, color: &str) -> Option<i32> {
  let regex_pattern = format!(r"(\d+) {}", color);
  let re = Regex::new(&regex_pattern).unwrap();

  re.captures(game_str).and_then(|cap| {
      cap.get(1).map(|match_| match_.as_str().parse::<i32>().ok()).flatten()
  })
}
