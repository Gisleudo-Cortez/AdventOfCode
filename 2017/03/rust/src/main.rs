use std::{collections::HashMap, fs};

struct Point {
    x: i32,
    y: i32,
}

fn find_layer(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }
    let mut layer: i32 = 0;
    while (2 * layer + 1).pow(2) < n {
        layer += 1;
    }
    layer
}

fn manhattan_distance(a: Point, b: Point) -> i32 {
    (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)).try_into().unwrap()
}

fn find_position(n: i32) -> Point {
    if n == 1 {
        return Point { x: 0, y: 0 };
    }

    let layer = find_layer(n);
    let side_len = 2 * layer;
    let max_val = (2 * layer + 1).pow(2);
    let steps = max_val - n;

    let (x, y) = match steps {
        s if s < side_len => {
            // bottom edge: right to left
            (layer - s, -layer)
        }
        s if s < 2 * side_len => {
            // left edge: bottom to top
            (-layer, -layer + (s - side_len))
        }
        s if s < 3 * side_len => {
            // top edge: left to right
            (-layer + (s - 2 * side_len), layer)
        }
        s => {
            // right edge: top to bottom
            (layer, layer - (s - 3 * side_len))
        }
    };

    Point { x, y }
}

fn part_1(n: i32) -> i32 {
    let n_point = find_position(n);
    let origin = Point { x: 0, y: 0 };
    manhattan_distance(origin, n_point)
}

fn sum_neighbors(map: &HashMap<(i32, i32), i32>, x: i32, y: i32) -> i32 {
    let mut sum = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            sum += map.get(&(x + dx, y + dy)).unwrap_or(&0);
        }
    }
    sum
}

fn part_2(n: i32) -> i32 {
    let mut map = HashMap::new();
    map.insert((0, 0), 1);

    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut x = 0;
    let mut y = 0;
    let mut step = 1;
    let mut dir_index = 0;

    loop {
        for _ in 0..2 {
            let (dx, dy) = dirs[dir_index % 4];
            for _ in 0..step {
                x += dx;
                y += dy;
                let val = sum_neighbors(&map, x, y);
                if val > n {
                    return val;
                }
                map.insert((x, y), val);
            }
            dir_index += 1;
        }
        step += 1;
    }
}

fn main() {
    let input = fs::read_to_string("../input.txt")
        .expect("Error reading input file")
        .trim()
        .parse::<i32>()
        .unwrap();
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1 = part_1(input);
    println!("The distance to the origin is: {part1}");
    println!("{sep} Part 2 {sep}");
    let part2 = part_2(input);
    println!("The first value higher than the input is: {part2}");
}
