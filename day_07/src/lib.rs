use regex::Regex;
use json::{object, JsonValue, array, stringify};

pub fn part1(lines:Vec<String>)->i32 {
  let tree = build_tree(lines);
  let total_size = crawl_tree_1(&tree["/"], 0);

  return total_size;
}

pub fn part2(lines:Vec<String>)->i32 {
  let tree = build_tree(lines);
  let size_full = get_dir_size(&tree["/"]);
  let size_free = 70000000 - size_full;
  let size_needed = 30000000 - size_free;
  
  return crawl_tree_2(&tree["/"], size_needed, 0);
}

fn build_tree(lines:Vec<String>)->JsonValue {
  let mut tree = object!{};
  let mut paths: Vec<String> = vec![];
  
  let cd = Regex::new(r"^\$ cd (.*)$").unwrap();
  let ls = Regex::new(r"^\$ ls$").unwrap();
  let dir = Regex::new(r"^dir (.*)$").unwrap();
  let file = Regex::new(r"^(\d*) (.*)$").unwrap();

  for line in lines {
    if cd.is_match(&line) {
      let caps = cd.captures(&line).unwrap();
      let dir = caps.get(1).unwrap().as_str();

      if dir == "/" { 
        paths = vec!["/".to_owned()];
      }else if dir == ".." {
        paths.pop();
      } else {
        paths.push(dir.to_owned());     
      }
    } else if ls.is_match(&line) {
      // println!("ls");
    } else if dir.is_match(&line) {
      let caps = dir.captures(&line).unwrap();
      let dir = caps.get(1).unwrap().as_str();

      tree = add_item_to_tree(tree, &paths, &dir, object!{}, false);   
    } else if file.is_match(&line) {
      let caps = file.captures(&line).unwrap();
      let size = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
      let name = caps.get(2).unwrap().as_str();

      tree = add_item_to_tree(tree, &paths, &name, object!{file_name: name, file_size: size}, true);  
    } else {
      println!("unknown {:?}", line);
    }
  }
  return tree;
}

fn add_item_to_tree(tree:JsonValue, paths:&Vec<String>, key:&str, object:JsonValue, is_file:bool)->JsonValue {
  let mut tree = tree;
  let mut current_dir = &mut tree;
  
  for path in paths {
    current_dir = &mut current_dir[path];
  }

  if is_file {
    if current_dir["files"].is_null() {
      current_dir["files"] = array![];
    }
    if current_dir["total_file_size"].is_null() {
      current_dir["total_file_size"] = 0.into();
    }

    current_dir["total_file_size"] = (current_dir["total_file_size"].as_i32().unwrap() + object["file_size"].as_i32().unwrap()).into();
    current_dir["files"].push(object).ok();
  }else if current_dir[key].is_null() {
    current_dir[key] = object
  }
  
  return tree;
}

fn crawl_tree_1(node:&JsonValue, total_size:i32)->i32{
  let mut total_size = total_size;

  for (key, item) in node.entries() {
    if key != "files" && key != "total_file_size" {
      total_size = crawl_tree_1(item, total_size); 
    }
  }
  let dir_size = get_dir_size(node);
  
  if dir_size < 100000 {
    total_size += dir_size;
  }

  return total_size;
}

fn crawl_tree_2(node:&JsonValue, size_needed:i32, result:i32)->i32{
  let mut result = result;

  for (key, item) in node.entries() {
    if key != "files" && key != "total_file_size" {
      result = crawl_tree_2(item, size_needed, result);
    }
  }
  let dir_size = get_dir_size(node);
  
  if dir_size > size_needed && (result == 0 || dir_size < result) {
    result = dir_size;
  }

  return result;
}

fn get_dir_size(node:&JsonValue)->i32 {
  let json_string = stringify(node.clone());
  let reg = Regex::new(r#""total_file_size":(\d*)"#).unwrap();
  let mut  dir_size = 0;

  reg.find_iter(&json_string).for_each(|mat| {
    let caps = reg.captures(mat.as_str()).unwrap();
    let size = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    dir_size += size;
  });

  return dir_size;
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

    assert_eq!(part1(lines), 95437);
  }

  #[test]
  fn test2_part1() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 1297159);
  }

  #[test]
  fn test1_part2() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 24933642);
  }

  #[test]
  fn test2_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 3866390);
  }
}