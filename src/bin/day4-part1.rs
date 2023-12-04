use std::fs::read_to_string;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let filename = "./inputs/day4/input.txt";
  let content = read_to_string(filename).unwrap();
  let lines: Vec<&str> = content.lines().collect();

  let points_to_award = [0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512];

  let mut values_to_be_summed: Vec<i32> = Vec::new();
  
  for (_index, line) in lines.iter().enumerate() {
    let parts: Vec<&str> = line
      .split('|')
      .map( |s| s.trim()).collect();
    
    let mut winning_numbers: Vec<&str> = parts[0]
      .split(' ')
      .map( |s| s.trim())
      .collect();

    winning_numbers.retain(|&x| x != "");

    let mut our_numbers: Vec<&str> = parts[1]
      .split(' ')
      .map( |s| s.trim())
      .collect();

    our_numbers.retain(|&x| x != "");

    let mut _match = 0;

    for number in our_numbers {
      if winning_numbers.contains(&number) {
        _match += 1;
      }
    }

    let points = points_to_award[_match];

    values_to_be_summed.push(points)
  }

  let sum: i32 = values_to_be_summed.iter().sum();
  // Print the answer to Day 4 Part 1
  println!("Answer: {:?}", sum);

  Ok(())
}
