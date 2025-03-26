use std::error::Error;
use std::fs;
use std::str;

// --- Part 1 ---
fn solve_part1(data: &[u8]) -> Result<u64, &'static str> {
    let mut decompressed_length: u64 = 0;
    let n = data.len();
    let mut i = 0;

    while i < n {
        if data[i] == b'(' {
            // Find the closing parenthesis relative to current position 'i'
            let close_paren_offset = data[i + 1..]
                .iter()
                .position(|&b| b == b')')
                .ok_or("Unmatched parenthesis")?;
            let close_paren_idx = i + 1 + close_paren_offset;

            let marker_content_bytes = &data[i + 1..close_paren_idx];
            let marker_content_str =
                str::from_utf8(marker_content_bytes).map_err(|_| "Invalid UTF-8 in marker")?;

            let (len_str, rep_str) = marker_content_str
                .split_once('x')
                .ok_or("Invalid marker format: missing 'x'")?;

            let length: usize = len_str.parse().map_err(|_| "Invalid length in marker")?;
            let repetitions: u64 = rep_str
                .parse() // Use u64 directly for repetitions
                .map_err(|_| "Invalid repetitions in marker")?;

            decompressed_length += (length as u64) * repetitions;

            // Move index past marker and the data block it refers to
            i = close_paren_idx + 1 + length;
            if i > n && length > 0 {
                // Ensure skip doesn't overshoot if length > 0
                return Err("Marker referred past end of data");
            }
        } else {
            decompressed_length += 1;
            i += 1;
        }
    }
    Ok(decompressed_length)
}

// --- Part 2 ---
fn calculate_decompressed_length_recursive(
    data: &[u8],
    start: usize,
    end: usize,
) -> Result<u64, &'static str> {
    let mut decompressed_length: u64 = 0;
    let mut i = start;

    while i < end {
        if data[i] == b'(' {
            // Find closing parenthesis within the current segment [i+1..end)
            let close_paren_offset = data[i + 1..end]
                .iter()
                .position(|&b| b == b')')
                .ok_or("Unmatched parenthesis within segment")?;
            let close_paren_idx = i + 1 + close_paren_offset; // Index relative to start of data

            let marker_content_bytes = &data[i + 1..close_paren_idx];
            let marker_content_str =
                str::from_utf8(marker_content_bytes).map_err(|_| "Invalid UTF-8 in marker")?;

            let (len_str, rep_str) = marker_content_str
                .split_once('x')
                .ok_or("Invalid marker format: missing 'x'")?;

            let length: usize = len_str.parse().map_err(|_| "Invalid length in marker")?;
            let repetitions: u64 = rep_str
                .parse() // Use u64 directly
                .map_err(|_| "Invalid repetitions in marker")?;

            let sub_start = close_paren_idx + 1;
            let sub_end = sub_start + length;

            // Check bounds: Ensure the referenced segment is within the current segment
            if sub_end > end {
                return Err("Marker references data outside its segment");
            }

            // Recursively calculate the length of the sub-segment
            let sub_length = calculate_decompressed_length_recursive(data, sub_start, sub_end)?;

            decompressed_length += sub_length * repetitions;

            // Move index past the marker and the data block it refers to
            i = sub_end;
        } else {
            decompressed_length += 1;
            i += 1;
        }
    }
    Ok(decompressed_length)
}

fn solve_part2(data: &[u8]) -> Result<u64, &'static str> {
    calculate_decompressed_length_recursive(data, 0, data.len())
}

// --- Main execution ---
fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "../input.txt";
    let input_data = fs::read_to_string(input_path)?;
    let trimmed_data = input_data.trim(); // Remove leading/trailing whitespace
    let bytes = trimmed_data.as_bytes();
    let sep = "=".repeat(20);

    match solve_part1(bytes) {
        Ok(length) => println!("{sep} Part 1 {sep}\nDecompressed length = {}", length),
        Err(e) => eprintln!("Part 1 Error: {}", e),
    }

    match solve_part2(bytes) {
        Ok(length) => println!("{sep} Part 2{sep}\nDecompressed length = {}", length),
        Err(e) => eprintln!("Part 2 Error: {}", e),
    }

    Ok(())
}
