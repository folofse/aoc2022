pub fn task1(lines:Vec<String>)->i32 {
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

pub fn task2(lines:Vec<String>)->i32 {
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
  use crate::task1;
  use crate::task2;
  use shared::read_lines;

  #[test]
  fn task1_test1() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(task1(lines), 24000);
  }

  #[test]
  fn task1_test2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(task1(lines), 74394);
  }

  #[test]
  fn task2_test1() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(task2(lines), 45000);
  }
  
  #[test]
  fn task2_test2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(task2(lines), 212836);
  }
}