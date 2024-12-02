use std::fs;

fn check_validity(vec: &[i32]) -> bool {
  let increasing = vec[1] - vec[0] > 0;
  let decreasing = vec[1] - vec[0] < 0;

  vec.windows(2).all(|element| {
    let difference = element[1] - element[0];

    if increasing {
      difference >= 1 && difference <= 3
    } else if decreasing {
      difference <= -1 && difference >= -3
    } else {
      false
    }
  })
}

fn tolerance_check(vec: &[i32]) -> bool {
  /* Hurr durr, lets bruteforce 🙃 */
  for i in 0..vec.len() {
    let mut tolerance_check = vec.to_vec();
    tolerance_check.remove(i);

    if check_validity(&tolerance_check) {
      return true;
    }
  }
  false
}

pub fn day2() {
    let contents = fs::read_to_string("./inputs/day2.txt")
        .expect("Something went wrong");

    let mut sum = 0;

    for line in contents.lines() {
      let level_readings: Vec<i32> = line.split(' ').filter_map(|reading| reading.parse::<i32>().ok()).collect();
      if check_validity(&level_readings) {
        println!("true");
        sum += 1;
      } else {
        if tolerance_check(&level_readings) {
          sum += 1;
        }
        println!("false");
      }
    }

    println!("{}", sum);
}
