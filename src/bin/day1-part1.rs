use tokio;
use std::fs::read_to_string;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "./inputs/day1/input.txt";

    // Array to store all calibration values
    let mut calibration_values: Vec<i32> = Vec::new();

    // Read file line by line
    for line in read_to_string(filename).unwrap().lines() {
        // Vector to store all integers in a line
        let mut my_vec: Vec<i32> = Vec::new();

        // Loop through each character in a line
        for char in line.to_string().chars() {
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
    // Print the answer to Part 1
    println!("{:?}", sum);

    Ok(())
}

fn is_integer(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}

