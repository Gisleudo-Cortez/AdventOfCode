use std::fs;

fn check_valid_triangle(triangles: Vec<String>) -> i32 {
    let mut valid_count = 0;

    for line in triangles {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Ok(a), Ok(b), Ok(c)) = (
            parts[0].parse::<i32>(),
            parts[1].parse::<i32>(),
            parts[2].parse::<i32>(),
        ) {
            if a + b > c && a + c > b && b + c > a {
                valid_count += 1;
            }
        }
    }
    valid_count
}

fn check_valid_triangle_2(triangles: Vec<String>) -> i32 {
    let mut valid_count = 0;
    let mut column_a: Vec<i32> = Vec::new();
    let mut column_b: Vec<i32> = Vec::new();
    let mut column_c: Vec<i32> = Vec::new();

    for line in triangles {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let a = parts[0].parse::<i32>().unwrap();
        let b = parts[1].parse::<i32>().unwrap();
        let c = parts[2].parse::<i32>().unwrap();

        column_a.push(a);
        column_b.push(b);
        column_c.push(c);
    }
    let all_columns = [column_a, column_b, column_c];

    for col in all_columns.iter() {
        for chunk in col.chunks(3) {
            let a = chunk[0];
            let b = chunk[1];
            let c = chunk[2];

            if a + b > c && a + c > b && b + c > a {
                valid_count += 1;
            }
        }
    }
    valid_count
}

fn main() {
    let data: Vec<String> = fs::read_to_string("../input.txt")
        .expect("Error reading input file")
        .lines()
        .map(|s| s.to_string())
        .collect();
    let sep = "=".repeat(20);
    let part1 = check_valid_triangle(data.clone());
    println!("{sep} Part 1 {sep}");
    println!("Total valid triangles: {part1}");
    let part2 = check_valid_triangle_2(data);
    println!("{sep} Part 2 {sep}");
    println!("Total Valid triangles: {part2}");
}
