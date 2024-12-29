// use std::fs;
// use std::io::Error;


// fn vec_lines(file_path: &str) -> Result<Vec<String>, Error> {
//     fs::read_to_string(file_path)?
//         .lines()
//         .map(|line| Ok(line.to_string()))
//         .collect()
// }




// fn part_1() -> Result<(), Error>{
//     let data: &Vec<String> = &vec_lines("../input.txt")?;
//     let mut total: i32 = 0;
//     for line in data.iter(){
//         let dimension: Vec<i32> = line
//         .split('x')
//         .filter_map(|s| s.parse::<i32>().ok())
//         .collect();
        
//         let (l,w,h) = (dimension[0], dimension[1], dimension[2]);
//         let lw = l*w;
//         let wh = w*h;
//         let hl = h*l;

//         let smallest = lw.min(wh).min(hl);
//         let area = (2*lw) + (2*wh) + (2*hl) + smallest;
//         total += area;

//     }
//     println!("{total}");
//     Ok(())
// }

// fn part_2() -> Result<(), Error>{
//     let data: &Vec<String> = &vec_lines("../input.txt")?;
//     let mut total: i32 = 0;
//     for line in data.iter(){
//         let mut dimension: Vec<i32> = line
//         .split('x')
//         .filter_map(|s| s.parse::<i32>().ok())
//         .collect();
//         dimension.sort();
//         let warp = (dimension[0] * 2) + (dimension[1] * 2);
//         let bow = dimension[0] * dimension[1] * dimension[2];
//         let ribbon = warp+bow;
//         total += ribbon;

//     }
//     println!("{total}");
//     Ok(())
// }

// fn main(){
//     println!("Part 1");
//     part_1();
//     println!("Part 2");
//     part_2();
//     }

use std::fs;
use std::io::Error;

struct Present {
    length: i32,
    width: i32,
    height: i32,
}

impl Present {
    fn from_str(s: &str) -> Option<Self> {
        let mut dims = s
            .split('x')
            .filter_map(|s| s.parse::<i32>().ok());
        
        match (dims.next(), dims.next(), dims.next()) {
            (Some(l), Some(w), Some(h)) => Some(Present {
                length: l,
                width: w,
                height: h,
            }),
            _ => None,
        }
    }

    fn surface_area(&self) -> i32 {
        let sides = [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];
        let smallest = sides.iter().min().unwrap_or(&0);
        2 * sides.iter().sum::<i32>() + smallest
    }

    fn ribbon_length(&self) -> i32 {
        let mut sides = [self.length, self.width, self.height];
        sides.sort_unstable();
        let perimeter = 2 * (sides[0] + sides[1]);
        let volume: i32 = sides.iter().product();
        perimeter + volume
    }
}

fn read_presents(file_path: &str) -> Result<Vec<Present>, Error> {
    fs::read_to_string(file_path)?
        .lines()
        .filter_map(Present::from_str)
        .collect::<Vec<_>>()
        .into_iter()
        .map(Ok)
        .collect()
}

fn calculate_total_paper(presents: &[Present]) -> i32 {
    presents.iter().map(Present::surface_area).sum()
}

fn calculate_total_ribbon(presents: &[Present]) -> i32 {
    presents.iter().map(Present::ribbon_length).sum()
}

fn main() -> Result<(), Error> {
    let presents = read_presents("../input.txt")?;
    
    println!("Part 1");
    println!("{}", calculate_total_paper(&presents));
    
    println!("Part 2");
    println!("{}", calculate_total_ribbon(&presents));
    
    Ok(())
}