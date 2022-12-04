pub fn part1(lines:Vec<String>)->i32 {
  return calculate_overlap(lines, true);
}

pub fn part2(lines:Vec<String>)->i32 {
  return calculate_overlap(lines, false);
}

fn calculate_overlap(lines:Vec<String>, part1:bool)->i32 {
  let mut result:i32 = 0;
  
  for line in lines{
    let split = line.split(",").collect::<Vec<&str>>();
    let r1 = get_range(split[0]);
    let r2 = get_range(split[1]);
    let r1len = r1.len();
    let r2len = r2.len();
    let matches: Vec<_> = r1.into_iter().filter(|item| r2.contains(item)).collect();

    if part1 {
      if matches.len() == r1len || matches.len() == r2len {
        result += 1;
      }
    }else{
      if matches.len() > 0{
        result += 1;
      }
    }
    
  }

  return result;
}

fn get_range(range:&str)->Vec<i32> {
  let split = range.split("-").collect::<Vec<&str>>();
  let rs = split[0].parse::<i32>().unwrap();
  let re = split[1].parse::<i32>().unwrap();

  return (rs ..= re).collect::<Vec<i32>>();
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

    assert_eq!(part1(lines), 2);
  }

  #[test]
  fn test2_part1() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 459);
  }

  #[test]
  fn test1_part2() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 4);
  }

  #[test]
  
  fn test2_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 779);
  }
}