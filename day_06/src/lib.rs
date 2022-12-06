pub fn part1(lines:Vec<String>)->usize {
  return find_unique(lines[0].clone(), true);
}

pub fn part2(lines:Vec<String>)->usize {
  return find_unique(lines[0].clone(), false);
}

fn find_unique(line:String, is_part1:bool) -> usize {
  let mut packet:Vec<char> = Vec::new();
  let mut index:usize = 0;
  let lines = line.chars();
  
  for c in lines {
    if is_part1 {
      if index > 3 {
        packet = line.clone().chars().collect::<Vec<char>>().drain(index-4..index).collect();
        packet.sort();
        packet.dedup();
        if is_part1 && packet.len() == 4 {
          return index;
        }
      }
    }else {
      if index > 13 {
        packet = line.clone().chars().collect::<Vec<char>>().drain(index-14..index).collect();
        packet.sort();
        packet.dedup();
        if packet.len() == 14 {
          return index;
        }
      }
    }
    index += 1;
  };

  return 0;
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

    assert_eq!(part1(lines), 7);
  }

  #[test]
  fn test2_part1() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 1912);
  }

  #[test]
  fn test1_part2() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 19);
  }

  #[test]
  fn test2_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 2122);
  }
}