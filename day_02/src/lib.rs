use std::collections::HashMap;

pub fn part1(lines:Vec<String>)->i32 {
  let mut current_score:i32 = 0;
  let scores = HashMap::from([
    ("A", 1), // Rock
    ("B", 2), // Paper
    ("C", 3), // Scissors
  ]);

  for line in lines {
    let str_split = line.split_whitespace().collect::<Vec<&str>>();
    let a: &str = str_split[0];
    let b_string: String = str_split[1].replace("X", "A").replace("Y", "B").replace("Z", "C");
    let b: &str = &b_string;

    current_score += scores.get(b).unwrap();
    
    if a == b {
      current_score += 3;
    }else if (b == "A" && a == "C") || 
             (b == "B" && a == "A") || 
             (b == "C" && a == "B") {
      current_score += 6;
    }
  }
    
  return current_score;
}

pub fn part2(lines:Vec<String>)->i32 {
  let mut current_score:i32 = 0;
  let scores = HashMap::from([
    ("A", 1), // Rock
    ("B", 2), // Paper
    ("C", 3), // Scissors
  ]);

  for line in lines {
    let str_split = line.split_whitespace().collect::<Vec<&str>>();
    let a: &str = str_split[0];
    let b: &str = str_split[1];


    if b == "X" { // Loose
      current_score += scores.get(get_value(a, false)).unwrap();
    } else if b == "Y" { // Draw
      current_score += 3;
      current_score += scores.get(a).unwrap();
    } else if b == "Z" { // Win
      current_score += 6;
      current_score += scores.get(get_value(a, true)).unwrap();
    } 
  }

  return current_score;

}

fn get_value(val:&str, winner:bool)->&str {
  let mut new_val:&str = "";
  if winner{
    if val == "A" { // Rock
      new_val = "B"; // Scissors
    } else if val == "B" { // Paper
      new_val = "C"; // Rock
    } else if val == "C" { // Scissors
      new_val = "A"; // Paper
    }
  }else{
    if val == "A" { // Rock
      new_val = "C"; // Paper
    } else if val == "B" { // Paper
      new_val = "A"; // Scissors 
    } else if val == "C" { // Scissors
      new_val = "B"; // Rock
    }
  }
  
  println!("{} -> {}, {}", val, new_val, winner);
  return new_val;
}

#[cfg(test)]
mod tests {
  use crate::part1;
  use crate::part2;
  use shared::read_lines;

  #[test]
  fn test1_part1() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 15);
  }

  #[test]
  fn test2_part1() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 10718);
  }

  #[test]
  fn test1_part2() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 12);
  }

  #[test]
  fn test2_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 14652);
  }
}