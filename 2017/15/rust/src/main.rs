const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;
const DIVISOR: u64 = 2147483647;
const PAIRS_P1: usize = 40_000_000;
const PAIRS_P2: usize = 5_000_000;
const MULTIPLE_A: u64 = 4;
const MULTIPLE_B: u64 = 8;

const START_A: u64 = 618;
const START_B: u64 = 814;

struct Generator {
    value: u64,
    factor: u64,
    multiple_check: Option<u64>,
}

impl Generator {
    fn new(start_value: u64, factor: u64, multiple_check: Option<u64>) -> Self {
        Generator {
            value: start_value,
            factor,
            multiple_check,
        }
    }

    fn next_value(&mut self) -> u64 {
        loop {
            self.value = (self.value * self.factor) % DIVISOR;
            if let Some(multiple) = self.multiple_check {
                if self.value % multiple == 0 {
                    break;
                }
            } else {
                break;
            }
        }
        self.value
    }
}

fn solve_part1(start_a: u64, start_b: u64) -> usize {
    let mut gen_a = Generator::new(start_a, FACTOR_A, None);
    let mut gen_b = Generator::new(start_b, FACTOR_B, None);
    let mut matches = 0;

    for _ in 0..PAIRS_P1 {
        let val_a = gen_a.next_value();
        let val_b = gen_b.next_value();

        if (val_a & 0xFFFF) == (val_b & 0xFFFF) {
            matches += 1;
        }
    }
    matches
}

fn solve_part2(start_a: u64, start_b: u64) -> usize {
    let mut gen_a = Generator::new(start_a, FACTOR_A, Some(MULTIPLE_A));
    let mut gen_b = Generator::new(start_b, FACTOR_B, Some(MULTIPLE_B));
    let mut matches = 0;

    for _ in 0..PAIRS_P2 {
        let val_a = gen_a.next_value();
        let val_b = gen_b.next_value();

        if (val_a & 0xFFFF) == (val_b & 0xFFFF) {
            matches += 1;
        }
    }
    matches
}

fn main() {
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1_result = solve_part1(START_A, START_B);
    println!("The judge's final count for Part 1 is: {part1_result}");

    println!("{sep} Part 2 {sep}");
    let part2_result = solve_part2(START_A, START_B);
    println!("The judge's final count for Part 2 is: {part2_result}");
}
