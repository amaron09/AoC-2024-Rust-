use std::fs;
use std::collections::HashMap;

struct Rule {
  first: i32,
  last: i32,
}

pub fn day5() {
  let contents = fs::read_to_string("./inputs/day5.txt")
        .expect("Something went wrong");

    let mut rules: Vec<Rule> = Vec::new();
    let mut page_updates: Vec<HashMap<i32, i32>> = Vec::new();

    // Iterate through each line in the file
    for line in contents.lines() {
        // Process the rules section
        if line.contains('|') {
            let rule_parts: Vec<&str> = line.split('|').collect();

            let first = rule_parts[0].trim().parse::<i32>().unwrap_or(0);
            let last = rule_parts[1].trim().parse::<i32>().unwrap_or(0);
            rules.push(Rule { first, last });
        }

        // Process the page updates section
        if line.contains(',') {
          let updates_row: Vec<i32> = line
              .split(',')
              .filter_map(|instruction| instruction.trim().parse::<i32>().ok())
              .collect();

          // Create a HashMap for the row, where key and value are the same
          let mut row_map: HashMap<i32, i32> = HashMap::new();
          for &value in &updates_row {
              row_map.insert(value, value); // Insert value as both key and value
          }

          // Store the HashMap for the current row
          page_updates.push(row_map);
      }
    }

    // Print rules to verify
    for rule in &rules {
        println!("Rule: first = {}, last = {}", rule.first, rule.last);
    }

    let mut valid_indexes:Vec<i32> = Vec::new();

    for (index, update) in page_updates.iter().enumerate() {
      /* println!("Row {} updates: {:?}", index + 1, update); */
      let mut is_valid = true;

      for (update_value_index, (&update_key, &update_value)) in update.iter().enumerate() {
        println!("Row {} updates: {:?}", index + 1, update_value);
        for rule in &rules {
          if update_value == &rule.first {
            if update_value_index >= rule.last as usize {
              is_valid = false;
              break
            }
          }

          if update_value == &rule.last {
            if update_value_index <= rule.first as usize {
              is_valid = false;
              break
            }
          }
        }
      }

      if is_valid {
        valid_indexes.push(index as i32);
        break;
      }
    }

    /* for (index, row_map) in page_updates.iter().enumerate() {
      println!("Row {} updates: {:?}", index + 1, row_map);
  } */
} 