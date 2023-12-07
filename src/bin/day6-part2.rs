use std::fs::read_to_string;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let filename = "./inputs/day6/input2.txt";
  let content = read_to_string(filename).unwrap();
  let lines: Vec<&str> = content.lines().collect();

  let mut values_to_be_multiplied: Vec<i64> = Vec::new();

  let time: i64 = lines[0].parse().unwrap();

  let distance: i64 = lines[1].parse().unwrap();

  let race = Race { time: time, record_distance: distance };

  let mut lower_bound = 0;
  let mut upper_bound = 0;

  for button_pressed_time in 0..race.time {
    // distance = speed x time
    let distance = (1 * button_pressed_time) * (race.time - button_pressed_time);

    if distance > race.record_distance {
      lower_bound = button_pressed_time;
      break;
    }
  }

  for button_pressed_time in (0..race.time).rev() {
    let distance = (1 * button_pressed_time) * (race.time - button_pressed_time);

    if distance > race.record_distance {
      upper_bound = button_pressed_time + 1;
      break;
    }
  }

  values_to_be_multiplied.push(upper_bound - lower_bound);

  let answer = values_to_be_multiplied.iter().fold(1, |acc, &x| acc * x);

  println!("Answer: {:?}", answer);

  Ok(())
}

#[derive(Debug)]
struct Race {
  time: i64,
  record_distance: i64
}