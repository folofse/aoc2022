pub fn count(lines:Vec<String>)->i32 {
  let mut most_cal:i32 = 0;
  let mut current_cal:i32 = 0;

  for line in lines {
    
    if !line.is_empty() {
        current_cal += line.parse::<i32>().unwrap();
        if current_cal > most_cal {
          most_cal = current_cal;  
        }
    } else {
      current_cal = 0;
    }
  }

  return most_cal
}

pub fn top3(lines:Vec<String>)->i32 {
  let mut all_cal:Vec<i32> = Vec::new();
  let mut current_cal:i32 = 0;

  for line in lines {
    
    if !line.is_empty() {
        current_cal += line.parse::<i32>().unwrap();
    } else {
      all_cal.push(current_cal);
      current_cal = 0;
    }
  }

  all_cal.push(current_cal); // Last line as well

  all_cal.sort();
  all_cal.reverse();

  return all_cal[0] + all_cal[1] + all_cal[2];
}

#[cfg(test)]
mod tests {
  use crate::count;
  use crate::top3;
  use shared::read_lines;

  #[test]
  fn count_test() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(count(lines), 24000);
  }

  #[test]
  fn count_test_2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(count(lines), 74394);
  }

  #[test]
  fn top3_test_1() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(top3(lines), 45000);
  }
  #[test]
  fn top3_test_2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(top3(lines), 212836);
  }
}