use std::fs;

fn knot_hash(path: &str) -> (usize, String) {
    let input: Vec<usize> = fs::read_to_string(path)
        .expect("Error reading input file")
        .split(",")
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect();
    let mut list: Vec<usize> = (0..=255).collect();
    let list_len = list.len();
    let mut current_position: usize = 0;

    for (skip_size, &length) in input.iter().enumerate() {
        let mut temp = Vec::new();
        for i in 0..length {
            let idx = (current_position + i) % list_len;
            temp.push(list[idx]);
        }
        temp.reverse();

        for (i, _item) in temp.iter().enumerate().take(length) {
            let idx = (current_position + i) % list_len;
            list[idx] = temp[i];
        }
        current_position = (current_position + length + skip_size) % list_len;
    }
    let out_1 = list[0] * list[1];
    // PART 2
    let mut list: Vec<usize> = (0..=255).collect();

    let mut input: Vec<usize> = fs::read_to_string(path)
        .expect("Error reading input file")
        .trim()
        .as_bytes()
        .iter()
        .map(|&b| b as usize)
        .collect();

    input.extend([17, 31, 73, 47, 23]);
    let mut current_position: usize = 0;
    let mut skip_size: usize = 0;
    for _ in 0..64 {
        for &length in &input {
            let mut temp = Vec::new();
            for i in 0..length {
                let idx = (current_position + i) % list_len;
                temp.push(list[idx]);
            }
            temp.reverse();

            for (i, _item) in temp.iter().enumerate().take(length) {
                let idx = (current_position + i) % list_len;
                list[idx] = temp[i];
            }
            current_position = (current_position + length + skip_size) % list_len;
            skip_size += 1;
        }
    }
    let dense_hash: Vec<usize> = list
        .chunks(16)
        .map(|block| block.iter().fold(0, |acc, &x| acc ^ x))
        .collect();
    let out_2 = dense_hash
        .iter()
        .map(|num| format!("{:02x}", num))
        .collect::<String>();

    (out_1, out_2)
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let (pt1, pt2) = knot_hash(path);
    println!("The result of multiplying the first two numbers is: {pt1}");
    println!("{sep} Part 2 {sep}");
    println!("The knot hash is : {pt2}");
}
