use std::fmt::format;
use std::ops::Index;
use crate::utils::{read_lines};

pub fn solve_exercise_1() {
  let mut calibration_values: Vec<Vec<u32>> = Vec::new();

  read_lines("resources/day1/exercise1.txt").unwrap().for_each(|line| {
    let mut numbers_per_calibration: Vec<u32> = Vec::new();
    for character in line.unwrap().chars() {
      if character.is_numeric() {
        numbers_per_calibration.push(character.to_digit(10).unwrap())
      }
    }
    calibration_values.push(numbers_per_calibration)
  });
}

pub fn solve_exercise_2() {
  let mut calibration_values: Vec<Vec<u32>> = Vec::new();
  read_lines("resources/day1/exercise1.txt").unwrap().for_each(|line| {
    let mut numbers_per_calibration: Vec<u32> = Vec::new();
    for character in replace_digit_names(line.unwrap()).chars() {
      if character.is_numeric() {
        numbers_per_calibration.push(character.to_digit(10).unwrap())
      }
    }
    calibration_values.push(numbers_per_calibration)
  });

  compute_results(&mut calibration_values);
}

fn replace_digit_names(mut line: String) -> String {

  let numbers= vec![
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
  ];

  // Replace each string representation with the corresponding numeric value
  for (word, replacement) in numbers {
    line = line.replace(word, &*format!("{}{}{}", word, replacement, word))
  }
  return line;
}

fn compute_results(calibration_values: &mut Vec<Vec<u32>>) {
  let mut calibration_results: Vec<Vec<&u32>> = Vec::new();
  let mut calibration_sums: Vec<i32> = Vec::new();
  calibration_values.iter().for_each(|result_line| {
    let mut numbers_per_calibration: Vec<&u32> = Vec::new();
    numbers_per_calibration.push(result_line.get(0).unwrap());
    numbers_per_calibration.push(result_line.get(result_line.len() - 1).unwrap());
    calibration_results.push(numbers_per_calibration)
  });

  calibration_results.iter().for_each(|result| {
    calibration_sums.push(format!("{}{}", result.get(0).unwrap().to_string(), result.get(1).unwrap().to_string()).parse::<i32>().unwrap());
  });

  println!("{}", calibration_sums.iter().sum::<i32>())
}