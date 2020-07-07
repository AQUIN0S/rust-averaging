/// Various methods to find averages, such as the mean, median and mode, of a list of integers
pub mod averages {

  /// Given a list of integers, return the sum divided by the length of the list
  pub fn mean(values: &Vec<i32>) -> f64 {
      let mut sum = 0;
      
      for value in values {
          sum += value;
      }

      return sum as f64 / values.len() as f64;
  }

  /// Given a list of integers, return the middle integer value when the list is sorted
  pub fn median(values: &Vec<i32>) -> Option<i32> {

    let mut values = values.clone();
    values.sort();

    match values.get(values.len() / 2) {
      Some(value) => Some(value.clone()),
      None => None,
    }

  }

  /// Return the most common value in a list of integers
  pub fn mode(values: &Vec<i32>) -> Option<i32> {
    use std::collections::HashMap;

    let mut values_freq: HashMap<i32, u32> = HashMap::new();
    let mut mode_val = None;
    let mut max_freq: u32 = 0;

    for value in values {
      match values_freq.get_mut(value) {
        Some(frequency) => {
          *frequency += 1;
          if *frequency > max_freq {
            max_freq = *frequency;
            mode_val = Some(*value);
          }
        },
        None => {
          values_freq.insert(*value, 1);
          if mode_val == None {
            mode_val = Some(*value);
          }
        },
      }
    }

    return mode_val;
  }

}