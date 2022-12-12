struct Visible {
  x1:bool,
  x2:bool,
  y1:bool,
  y2:bool,
}
struct Score {
  x1:usize,
  x2:usize,
  y1:usize,
  y2:usize,
}

pub fn part1(lines:Vec<String>)->usize {
  let height_map:Vec<Vec<usize>> = get_height_map(lines);
  let mut num_visible = 0;
  
  height_map.clone().into_iter().enumerate().for_each(|(y, row)|{
    row.into_iter().enumerate().for_each(|(x, col)|{
      if is_visible(x, y, height_map.clone()) {
        num_visible += 1;
      }
    });
  });

  return num_visible;
}

pub fn part2(lines:Vec<String>)->i32 {
  let height_map:Vec<Vec<usize>> = get_height_map(lines);
  let mut high_score = 0;

  height_map.clone().into_iter().enumerate().for_each(|(y, row)|{
    row.into_iter().enumerate().for_each(|(x, col)|{
      let score = get_score(x, y, height_map.clone());
      if score > high_score {
        high_score = score;
      }
    });
  });

  return high_score;
}

fn get_height_map(lines:Vec<String>) -> Vec<Vec<usize>> {
  return lines.iter().map(|line|line.chars().map(|c|c.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>();
}

fn is_visible(x:usize, y:usize, height_map:Vec<Vec<usize>>)->bool {
  // Ignore Edge
  if x == 0 || y == 0 || x == height_map[y].len() - 1 || y == height_map.len() - 1 {
    return true;
  }

  let mut visible = Visible{x1:true, x2:true, y1:true, y2:true};
  let current_height = height_map[y][x];

  for r in 0..height_map.len() {
    for c in 0..height_map[r].len() {
      
      if (r == y && c == x) || // Not self
         (r != y && c != x) { // Not related
        continue;
      }
      
      
      if height_map[r][c] >= current_height {
       // println!("y:{} x:{} height:{} current height:{} y:{}, x:{}",r,c, height_map[r][c], current_height, y, x);
        if r == y {
          if c < x {
            visible.x1 = false;
          }else{
            visible.x2 = false;
          }
        }else if c == x {
          if r < y {
            visible.y1 = false;
          }else{
            visible.y2 = false;
          }
        }
      }
    }
  }

  if visible.x1 || visible.x2 || visible.y1 || visible.y2 {
    return true;
  }else {
    return false;
  }
}

fn get_score(x:usize, y:usize, height_map:Vec<Vec<usize>>)->i32 {
  // Ignore Edge
  if x == 0 || y == 0 || x == height_map[y].len() - 1 || y == height_map.len() - 1 {
    return 0;
  }

  let mut score = Score{x1:0, x2:0, y1:0, y2:0}; // Can always see one tree
  let current_height = height_map[y][x];

  // y1
  for r in (0..y).rev() {
    score.y1 += 1;
    if height_map[r][x] >= current_height {
      break;
    }
  }
  // y2
  for r in y+1..height_map.len() {
    score.y2 += 1;
    if height_map[r][x] >= current_height {
      break;
    }
  }
  // x1
  for c in (0..x).rev() {
    score.x1 += 1;
    if height_map[y][c] >= current_height {
      break;
    }
  }
  // x2
  for c in x+1..height_map[y].len() {
    score.x2 += 1;
    if height_map[y][c] >= current_height {
      break;
    }
  }
  return (score.x1 * score.x2 * score.y1 * score.y2).try_into().unwrap();
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

    assert_eq!(part1(lines), 21);
  }

  #[test]
  fn test2_part1() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 1827);
  }

  #[test]
  fn test1_part2() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 8);
  }

  #[test]
  fn test2_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 335580);
  }
}