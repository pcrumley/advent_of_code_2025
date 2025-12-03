fn get_joltage(bank: &str, num_batteries: usize) -> u64 {
    let mut ptr = 0;
    let mut sum = 0;
    for i in (0..num_batteries).rev() {
        let end = bank.len() - i;
        let joltage = bank[ptr..end].chars().max().unwrap();
        sum += joltage.to_digit(10).unwrap() as u64 * 10i64.pow(i as u32) as u64;
        ptr += bank[ptr..].find(joltage).unwrap() + 1;
    }
    sum
}

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut part_a = 0;
    let mut part_b = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        part_a += get_joltage(&line, 2);
        part_b += get_joltage(&line, 12);
    }
    println!("Part a: `{part_a}``");
    println!("Part b: `{part_b}`");
}
