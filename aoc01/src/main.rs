
use std::fs::File;
use std::io::{BufReader, Result, Lines, BufRead};
use std::path::Path;
use std::str::FromStr;

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn parse_lines_get_calories(lines: Lines<BufReader<File>>) -> Vec<u32>
{
    let mut cals: Vec<u32> = Vec::new();
    let mut cur = 0u32;
    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {
                if cur > 0u32 {
                    cals.push(cur);
                    cur = 0u32;
                }
            }
            else {
                cur += u32::from_str(line.as_str()).expect("int parse error");
            }
        }
    }

    if cur > 0u32 {
        cals.push(cur);
    }

    cals
}

fn main() {
    let lines = read_lines("calories.csv").expect("foo");
    let calories = parse_lines_get_calories(lines);
    let mut indices: Vec<usize> = (0..calories.len()).collect();
    indices.sort_by_key(|i| u32::MAX - calories[*i]);

    let top1 = indices[0];
    let top1_calories = calories[top1];
    println!("Part 1");
    println!(
        "The elf with the most calories is #{} with {} calories.",
        top1, top1_calories
    );

    let top3_calories: u32 = indices[0..3].iter().map(|i| calories[*i]).sum();
    println!("Part 2");
    println!("The top 3 elves have, in total, {top3_calories} calories.");
}
