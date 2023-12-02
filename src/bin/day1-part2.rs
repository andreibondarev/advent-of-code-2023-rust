use tokio;
use std::fs::read_to_string;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "./inputs/day1/input.txt";

    // Array to store all calibration values
    let mut calibration_values: Vec<i32> = Vec::new();

    // 2-D array to store all spelled out numbers
    let digits = [
        ["one", "1"],
        ["two", "2"],
        ["three", "3"],
        ["four", "4"],
        ["five", "5"],
        ["six", "6"],
        ["seven", "7"],
        ["eight", "8"],
        ["nine", "9"]
    ];

    // 2-D array to store all of the run on numbers
    let run_on_digits: &[(&str, &[&str])] = &[
        ("oneight", &["1", "8"]),
        ("twone", &["2", "1"]),
        ("threeight", &["3", "8"]),
        ("fiveight", &["5", "8"]),
        ("sevenine", &["7", "9"]),
        ("eightwo", &["8", "2"]),
        ("eighthree", &["8", "3"]),
        ("nineight", &["9", "8"]),
    ];

    // Read file line by line
    for line in read_to_string(filename).unwrap().lines() {
        // Vector to store all integers in a line
        let mut my_vec: Vec<i32> = Vec::new();

        // Copy the line to a new string
        let mut new_line = line.to_string();

        for &(word, digits) in run_on_digits {
            // Find a match for the run-on number
            let _match = new_line.find(word);

            if _match != None {
                let mut contains_a_number = false;

                // Check if the preceeding string slice contains any numbers
                new_line[0.._match.unwrap()].chars().for_each(|c| {
                    if is_integer(&c.to_string()) {
                        contains_a_number = true;
                    }
                });

                if contains_a_number {
                    // If it does then use the second word in the run-on word
                    new_line = new_line.replace(word, digits[1]);
                } else {
                    // If it doesn't then use the first word in the run-on word
                    new_line = new_line.replace(word, digits[0]);
                }
            }
        }

        // Replace all regular spelled out numbers
        for pair in digits {
            let word = pair[0];
            let number = pair[1];
    
            new_line = new_line.replace(word, number);
        }

        // Loop through each character in a line
        for char in new_line.chars() {
            // Check if character is an integer
            if is_integer(&char.to_string()) {
                // Push integer to vector
                my_vec.push(char.to_string().parse::<i32>().unwrap());
            }
        }

        // Get the first and last integer in the vector
        let first_int = my_vec[0].to_string();
        let last_int = my_vec[my_vec.len() -1].to_string();

        // Concatenate the first and last integer into a string
        let calibration_value = format!("{}{}", first_int, last_int);
        // Push the string to the calibration values array
        calibration_values.push(calibration_value.parse::<i32>().unwrap());
    }

    // Sum all calibration values
    let sum: i32 = calibration_values.iter().sum();
    // Print the answer to Day 1 Part 1
    println!("Answer: {:?}", sum);

    Ok(())
}

fn is_integer(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}

