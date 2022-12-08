
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

fn index_of_max(v: &Vec<u32>) -> usize {
    let mut max = v.first().expect("empty vector");
    let mut i_max = 0usize;

    for (i, vi) in v.iter().enumerate() {
        if vi > max {
            max = vi;
            i_max = i;
        }
    }

    i_max
}

fn main() {
    let lines = read_lines("calories.csv").expect("foo");
    let calories = parse_lines_get_calories(lines);
    let index_of_elf_with_most = index_of_max(&calories);
    println!(
        "The elf with the most calories is #{} with {} calories.",
        index_of_elf_with_most+1,
        calories[index_of_elf_with_most]
    );
}
