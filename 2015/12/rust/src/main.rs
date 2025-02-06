use std::fs;
use regex::Regex;
use serde_json::Value;

fn total_sum_1(data: &str) -> i32 {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(data)
    .filter_map(|m| m.as_str().parse::<i32>().ok())
    .sum()
}

fn sum_non_red(data: &Value) -> i32 {
    match data {
        Value::Number(n)        => n.as_i64().unwrap_or(0) as i32,
        Value::Array(arr) => arr.iter().map(sum_non_red).sum(),
        Value::Object(obj) => {
            if obj.values().any(|v| v == "red") {
                return 0;
            }
            obj.values().map(sum_non_red).sum()
        }
        _ => 0,
    }
}

fn total_sum_2(data: &str) -> i32 {
    let json_data: Value = serde_json::from_str(data).unwrap_or(Value::Null);
    sum_non_red(&json_data)
}


fn main() {
    let data = fs::read_to_string("../input.txt").expect("Error reading file").trim().to_string();
    let sep = "=".repeat(20);
    println!("{} Part 1 {}", sep, sep);
    println!("Sum: {}", total_sum_1(&data));
    println!("{} Part 2 {}", sep, sep);
    println!("Sum: {}", total_sum_2(&data));
}
