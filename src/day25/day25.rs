use std::fs;

fn calculate_height(matrix: &[Vec<char>]) -> Vec<usize> {
    let mut height = vec![0; 5];

    for y in 1..6 {
        for i in 0..5 {
            if matrix[y][i] == '#' {
                height[i] += 1;
            }
        }
    }

    height
}

fn vector_addition(vector1: &[usize], vector2: &[usize]) -> Vec<usize> {
    vector1
        .iter()
        .zip(vector2.iter())
        .map(|(v1, v2)| v1 + v2)
        .collect()
}

fn check_fit(vector1: &[usize], vector2: &[usize]) -> bool {
    vector_addition(vector1, vector2)
        .iter()
        .all(|&v| v <= 5)
}

pub fn day25() {
    // Read file
    let data = fs::read_to_string("inputs/day25.txt")
        .expect("Failed to read file");

    // Parse blocks
    let blocks: Vec<Vec<String>> = data
        .split("\n\n")
        .map(|block| block.lines().map(|line| line.to_string()).collect())
        .collect();

    // Transform blocks into items
    let items: Vec<Vec<Vec<char>>> = blocks
        .iter()
        .map(|block| block.iter().map(|line| line.chars().collect()).collect())
        .collect();

    // Keys and Locks
    let keys_and_locks = items.iter().fold(
        (vec![], vec![]), // (keys, locks)
        |(mut keys, mut locks), item| {
            let is_key = item[0].iter().any(|&c| c == '.');
            let test = calculate_height(&item);

            if is_key {
                keys.push(test);
            } else {
                locks.push(test);
            }

            (keys, locks)
        },
    );

    let (keys, locks) = keys_and_locks;

    // Calculate unique keys
    let unique_keys: usize = keys.iter().fold(0, |acc, curr| {
        let fit = locks.iter().filter(|lock| check_fit(curr, lock)).count();
        acc + fit
    });

    println!("uniqueKeys: {}", unique_keys);
}