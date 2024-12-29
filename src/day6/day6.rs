use std::fs;

pub fn day6() {
  let contents = fs::read_to_string("./inputs/day6.txt")
    .expect("Something went wrong");

  println!("{}", contents);

  let row_length = contents.lines().next().unwrap();
  println!("{}", row_length.len());

  let guard_position = contents.find('^');
  println!("{:?}", guard_position);

  let mut guard_direction = "up";

  if (guard_direction == "up") {
    let new_position = guard_position - row_length;
    
    guard_position = guard_position - row_length
  } else {
    println!("down");
  }
  /* let mut map: Vec<String> = Vec::new();

  for line in contents.lines() {
    map.push(line.to_string());
  } */

  
} 