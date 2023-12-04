use std::fs::read_to_string;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let filename = "./inputs/day4/input.txt";
  let content = read_to_string(filename).unwrap();
  let lines: Vec<&str> = content.lines().collect();

  let mut card_counts  = vec![1; lines.len()];
  
  for (index, line) in lines.iter().enumerate() {
    let parts: Vec<&str> = line
      .split('|')
      .map( |s| s.trim()).collect();
    
    let mut winning_numbers: Vec<&str> = parts[0]
      .split(' ')
      .map( |s| s.trim())
      .collect();

    winning_numbers.retain(|&x| !x.is_empty());

    let mut our_numbers: Vec<&str> = parts[1]
      .split(' ')
      .map( |s| s.trim())
      .collect();

    our_numbers.retain(|&x| !x.is_empty());

    let mut _match = 0;

    for number in our_numbers {
      if winning_numbers.contains(&number) {
        _match += 1;
      }
    }

    for i in 0.._match {
      card_counts[index+i+1] += 1 * card_counts[index];
    }
  }

  let sum: i32 = card_counts.iter().sum();
  // Print the answer to Day 4 Part 2
  println!("Answer: {:?}", sum);

  Ok(())
}
