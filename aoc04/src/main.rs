use std::collections::hash_map::RandomState;
use std::fs::File;
use std::io::{BufReader, Result, Lines, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::{Regex, Match, CaptureMatches};


fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn line2ranges(line: String) -> (HashSet<u8, RandomState>, HashSet<u8, RandomState>)
{
    lazy_static!{
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }

    if let Some(captures) = RE.captures(&line) {
        let l_lb = u8::from_str(&captures[1])
            .expect(format!("could not parse {}", &captures[1]).as_str());
        let l_ub = u8::from_str(&captures[2])
            .expect(format!("could not parse {}", &captures[2]).as_str());
        let r_lb = u8::from_str(&captures[3])
            .expect(format!("could not parse {}", &captures[3]).as_str());
        let r_ub = u8::from_str(&captures[4])
            .expect(format!("could not parse {}", &captures[4]).as_str());

        (HashSet::from_iter(l_lb..l_ub+1), HashSet::from_iter(r_lb..r_ub+1))
    }
    else {
        panic!("Could not parse line! {line}")
    }
}

fn are_sets_subsets(a: HashSet<u8>, b: HashSet<u8>) -> u16 {
    if a.is_subset(&b) || b.is_subset(&a) {
        1
    }
    else {
        0
    }
}

fn do_sets_overlap_at_all(a: HashSet<u8>, b: HashSet<u8>) -> u16 {
    if a.intersection(&b).count() > 0 {
        1
    }
    else {
        0
    }
}

fn part1() {
    let lines = read_lines("pairings.txt").unwrap();
    let subsets_in_total: u16 = lines
        .map(|s| line2ranges(s.unwrap()))
        .map(|(a, b)| are_sets_subsets(a, b))
        .sum();
    println!("Part 2 - # subsets: {}", subsets_in_total);
}

fn part2() {
    let lines = read_lines("pairings.txt").unwrap();
    let overlaps_in_total: u16 = lines
        .map(|s| line2ranges(s.unwrap()))
        .map(|(a, b)| do_sets_overlap_at_all(a, b))
        .sum();
    println!("Part 1 - # overlapping sets: {}", overlaps_in_total);
}

fn main() {
    part1();
    part2();
}
