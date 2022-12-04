pub fn part1(lines:Vec<String>)->i32 {
  let mut result:i32 = 0;
  
  for line in lines{
    let (comp1, comp2) = line.split_at(line.len()/2);

    let common_type:Vec<char> = comp1.chars().filter(|c1| 
      comp2.chars().filter(|c2| {
        c1 == c2
      }).count() > 0
    ).collect();

    result +=  get_char_value(common_type[0]);
  }

  return result;
}

pub fn part2(lines:Vec<String>)->i32 {
  let mut result:i32 = 0;
  let mut row:usize = 0;
  
  for _line in &lines{
    row += 1;
    if row % 3 == 0 {
      let group1 = &lines[row-3];
      let group2 = &lines[row-2];
      let group3 = &lines[row-1];

      let badge:Vec<char> = group1.chars().filter(|g1| 
        group2.chars().filter(|g2| {
          group3.chars().filter(|g3| {
            g1 == g2 && g2 == g3
          }).count() > 0
        }).count() > 0
      ).collect();
  
      result +=  get_char_value(badge[0]);
    }
  }

  return result;
}

fn get_char_value(c:char)->i32 {
  let lower = ('a'..='z').into_iter().collect::<Vec<char>>();
  let upper = ('A'..='Z').into_iter().collect::<Vec<char>>();
  let full = [lower, upper].concat();

  full.iter().position(|&r| r == c).unwrap() as i32 + 1
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

    assert_eq!(part1(lines), 157);
  }

  #[test]
  fn test2_part1() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 7967);
  }

  #[test]
  fn test1_part2() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 70);
  }

  #[test]
  fn test2_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 2716);
  }
}