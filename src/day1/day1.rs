use std::fs;

pub fn day1() {
    let contents = fs::read_to_string("./inputs/day1.txt")
        .expect("Something went wrong");

    let mut sum: i32 = 0;
    let mut similarity_score = 0;

    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();
    
    for line in contents.lines() {
        let mut locations = line.splitn(2, "   ");
        
        let left = locations.next().unwrap_or("0").trim().parse::<i32>().unwrap_or(0);
        let right = locations.next().unwrap_or("0").trim().parse::<i32>().unwrap_or(0);

        left_column.push(left);
        right_column.push(right);
    }

    left_column.sort_unstable();
    right_column.sort_unstable();

    for i in 0..left_column.len() {
        let left_value = left_column[i];
        let right_value = right_column[i];
        sum += (left_value - right_value).abs();

        /* For part 2 */
        let matching_indices: Vec<usize> = right_column.iter()
            .enumerate()
            .filter_map(|(index, &value)| if value == left_value { Some(index) } else { None })
            .collect();
        similarity_score += (left_column[i] as i32) * (matching_indices.len() as i32);
    }
    println!("Difference: {}", sum);
    println!("Similarity score: {}", similarity_score);
}
