use std::fs;
use std::io::Error;


fn vec_lines(file_path: &str) -> Result<Vec<String>, Error> {
    fs::read_to_string(file_path)?
        .lines()
        .map(|line| Ok(line.to_string()))
        .collect()
}




fn part_1() -> Result<(), Error>{
    let data: &Vec<String> = &vec_lines("../input.txt")?;
    let mut total: i32 = 0;
    for line in data.iter(){
        let dimension: Vec<i32> = line
        .split('x')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
        
        let (l,w,h) = (dimension[0], dimension[1], dimension[2]);
        let lw = l*w;
        let wh = w*h;
        let hl = h*l;

        let smallest = lw.min(wh).min(hl);
        let area = (2*lw) + (2*wh) + (2*hl) + smallest;
        total += area;

    }
    println!("{total}");
    Ok(())
}

fn part_2() -> Result<(), Error>{
    let data: &Vec<String> = &vec_lines("../input.txt")?;
    let mut total: i32 = 0;
    for line in data.iter(){
        let mut dimension: Vec<i32> = line
        .split('x')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
        dimension.sort();
        let warp = (dimension[0] * 2) + (dimension[1] * 2);
        let bow = dimension[0] * dimension[1] * dimension[2];
        let ribbon = warp+bow;
        total += ribbon;

    }
    println!("{total}");
    Ok(())
}

fn main(){
    println!("Part 1");
    part_1();
    println!("Part 2");
    part_2();
    }