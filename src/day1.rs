use crate::utils::read_lines;

pub fn solve() {
  let mut calibration_values: Vec<Vec<u32>> = Vec::new();
  let mut calibration_results: Vec<Vec<&u32>> = Vec::new();
  let mut calibration_sums: Vec<i32> = Vec::new();
  read_lines("resources/day1/exercise1.txt").unwrap().for_each(|line| {
    let mut numbers_per_calibration: Vec<u32> = Vec::new();
    for character in line.unwrap().chars() {
      if character.is_numeric() {
        numbers_per_calibration.push(character.to_digit(10).unwrap())
      }
    }
    calibration_values.push(numbers_per_calibration)
  });

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
