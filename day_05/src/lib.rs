use regex::Regex;

struct Movements{
  m: usize,
  f: usize,
  t: usize
}
struct PilesAndMovements {
  piles: Vec<Vec<String>>,
  moves: Vec<Movements>,
}

pub fn part1(lines:Vec<String>)->String {
  let p_and_m = get_piles_and_movements(lines);
  let mut piles = p_and_m.piles;

  for m in p_and_m.moves{
    for _ in 0..m.m {
      let val_to_move = piles[m.f-1].remove(0);
      piles[m.t-1].insert(0, val_to_move);
    }
  }
  
  return get_result(piles);
}

pub fn part2(lines:Vec<String>)->String {
  let p_and_m = get_piles_and_movements(lines);
  let mut piles = p_and_m.piles;

  for m in p_and_m.moves{
    let mut group_to_move:Vec<String> = piles[m.f-1].drain(0..m.m).collect();
    group_to_move.reverse();
    for val_to_move in group_to_move{
      piles[m.t-1].insert(0, val_to_move);
    }
  }

  return get_result(piles);
}



fn get_piles_and_movements(lines:Vec<String>) -> PilesAndMovements {
  let mut piles:Vec<Vec<String>> = Vec::new();
  let mut movements:Vec<Movements> = Vec::new();
  let mut is_movement:bool = false;

  let re1 = Regex::new(r"([0-9]\s?)").unwrap();
  let re2 = Regex::new(r"(.{3}\s?)").unwrap();
  let re3 = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

  for line in lines{
    if !is_movement{
      if line == "" {
        is_movement = true;
      }else if !re1.is_match(&line){
        let matches = re2.find_iter(&line)
          .filter_map(|c| 
            Some(c.as_str().replace(" ", "").replace("[", "").replace("]", ""))
          ).collect::<Vec<String>>();
        
        for (i, m) in matches.iter().enumerate(){
          if piles.get(i) == None {
            piles.push(Vec::new());
          }
          if m != ""{
            piles[i].push(m.to_string());
          }
        } 
      }
    }else{
      
      let captures = re3.captures(&line).unwrap();
      let movement = Movements{
        m: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        f: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        t: captures.get(3).unwrap().as_str().parse::<usize>().unwrap()
      };
      movements.push(movement);
    }
  }

  return  PilesAndMovements{
    piles: piles,
    moves: movements
  };
}

fn get_result(piles:Vec<Vec<String>>) -> String {
  let mut result  = "".to_owned();;
  for p in piles{
    result.push_str(&p[0]);
  }
  return result;
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

    assert_eq!(part1(lines), "CMZ");
  }

  #[test]
  fn test2_part1() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), "LBLVVTVLP");
  }

  #[test]
  fn test1_part2() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), "MCD");
  }

  #[test]
  fn test2_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), "TPFFBDRJD");
  }
}