use std::fs::read_to_string;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let filename = "./inputs/day6/input.txt";
  let content = read_to_string(filename).unwrap();
  let lines: Vec<&str> = content.lines().collect();

  let mut values_to_be_multiplied: Vec<i32> = Vec::new();

  let times: Vec<i32> = lines[0]
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

  let distances: Vec<i32> = lines[1]
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

  let races = [
    Race { time: times[0], record_distance: distances[0] },
    Race { time: times[1], record_distance: distances[1] },
    Race { time: times[2], record_distance: distances[2] },
    Race { time: times[3], record_distance: distances[3] }
  ];

  for race in races {
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
  }

  let answer = values_to_be_multiplied.iter().product::<i32>();

  println!("Answer: {:?}", answer);

  Ok(())
}

#[derive(Debug)]
struct Race {
  time: i32,
  record_distance: i32
}