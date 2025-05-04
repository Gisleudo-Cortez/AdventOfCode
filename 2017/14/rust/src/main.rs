use std::fs;

fn knot_hash(input: &str) -> String {
    let mut list: Vec<u8> = (0..=255).collect();
    let mut lengths: Vec<u8> = input.bytes().collect();
    lengths.extend([17, 31, 73, 47, 23]);

    let list_len = list.len();
    let mut pos = 0;
    let mut skip = 0;

    for _ in 0..64 {
        for &len_u8 in &lengths {
            let len = len_u8 as usize;
            if len > list_len {
                continue;
            }

            let mut sublist = Vec::with_capacity(len);
            for i in 0..len {
                sublist.push(list[(pos + i) % list_len]);
            }
            sublist.reverse();

            for i in 0..len {
                list[(pos + i) % list_len] = sublist[i];
            }
            pos = (pos + len + skip) % list_len;
            skip += 1;
        }
    }

    let dense_hash: Vec<u8> = list
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, &x| acc ^ x))
        .collect();

    // Convert to hex string
    dense_hash
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}

fn hex_char_to_bin(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'a' => "1010",
        'b' => "1011",
        'c' => "1100",
        'd' => "1101",
        'e' => "1110",
        'f' => "1111",
        _ => "",
    }
}

fn dfs(grid: &mut Vec<Vec<u8>>, r: usize, c: usize) {
    if r >= grid.len() || c >= grid[0].len() || grid[r][c] != 1 {
        return;
    }

    grid[r][c] = 2; // Mark as visited 

    if r > 0 {
        dfs(grid, r - 1, c);
    } // Up
    dfs(grid, r + 1, c); // Down
    if c > 0 {
        dfs(grid, r, c - 1);
    } // Left
    dfs(grid, r, c + 1); // Right 
}

fn solve(input_key: String) -> (usize, usize) {
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(128);
    let mut used_count = 0;

    for r in 0..128 {
        let row_key = format!("{}-{}", input_key, r);
        let hash = knot_hash(&row_key);

        let mut binary_row = String::with_capacity(128);
        for hex_char in hash.chars() {
            binary_row.push_str(hex_char_to_bin(hex_char));
        }

        let mut grid_row: Vec<u8> = Vec::with_capacity(128);
        for bit in binary_row.chars() {
            let val = bit.to_digit(10).unwrap() as u8;
            grid_row.push(val);
            if val == 1 {
                used_count += 1;
            }
        }
        grid.push(grid_row);
    }

    let mut region_count = 0;
    for r in 0..128 {
        for c in 0..128 {
            if grid[r][c] == 1 {
                region_count += 1;
                dfs(&mut grid, r, c); // Explore and mark the entire region
            }
        }
    }

    (used_count, region_count)
}

fn main() {
    let input_key = fs::read_to_string("../input.txt")
        .expect("Failed reading input file. Ensure '../input.txt' exists.")
        .trim()
        .to_string();

    let (part1_result, part2_result) = solve(input_key);

    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    println!("Total used squares = {}", part1_result);
    println!("{sep} Part 2 {sep}");
    println!("Number of regions = {}", part2_result);
}
