use std::collections::hash_map::RandomState;
use std::fs::File;
use std::io::{BufReader, Result, Lines, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::collections::HashSet;
use std::ops::Deref;
use itertools::{Itertools, Chunk};


fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn char2priority(c: char) -> u16
{
    let i = c as u16;
    if i > 96 { i - 96 } else { i - 64 + 26 }
}

fn line2halves(line: String) -> (Vec<char>, Vec<char>)
{
    let mut left = Vec::new();
    let mut right = Vec::new();
    let hn = line.len()/2;

    for (i, c) in line.chars().enumerate() {
        {if i < hn {&mut left} else {&mut right}}.push(c);
    }

    (left, right)
}

fn get_one_common_value<T>(left: Vec<T>, right: Vec<T>) -> T
where T: Eq + Copy + std::hash::Hash
{
    let mut common_set: HashSet<T, RandomState> = HashSet::from_iter(left);
    for ri in right {
        if common_set.contains(&ri) {
            return ri;
        }
    }
    panic!("no common elements found!")
}

fn parse_line_get_common_item(line: String) -> char
{
    let (left, right) = line2halves(line);
    get_one_common_value(left, right)
}

fn parse_lines_get_common_priorities(lines: Lines<BufReader<File>>) -> Vec<u16>
{
    lines
        .map(|l| parse_line_get_common_item(l.unwrap()))
        .map(|c| char2priority(c))
        .collect()
}

fn get_badge_for_group(mut group: Vec<String>) -> char
{
    let a: HashSet<char, RandomState> = HashSet::from_iter(group[0].chars());
    let b: HashSet<char, RandomState> = HashSet::from_iter(group[1].chars());
    let c: HashSet<char, RandomState> = HashSet::from_iter(group[2].chars());

    let intersection = a
        .intersection(&b)
        .map(|cr| *cr)
        .collect::<HashSet<_>>()
        .intersection(&c)
        .map(|cr| *cr)
        .collect::<HashSet<char, RandomState>>();

    assert_eq!(intersection.len(), 1);

    *intersection.iter().next().unwrap()
}

fn parse_lines_get_badges(mut lines: Lines<BufReader<File>>) -> Vec<char>
{
    lines
        .map(|res| res.unwrap())
        .chunks(3)
        .into_iter()
        .map(|chunk| get_badge_for_group(chunk.collect()))
        .collect()
}

fn part1() {
    let lines = read_lines("rucksack_contents.txt")
        .expect("parse failed");
    let common_priorities = parse_lines_get_common_priorities(lines);

    let total: u16 = common_priorities.iter().sum();

    println!("Part 1: total = {total}");
}

fn part2() {
    let lines = read_lines("rucksack_contents.txt")
        .expect("parse failed");

    let total: u16 = parse_lines_get_badges(lines).iter().map(|b| char2priority(*b)).sum();

    println!("Part 2: total = {total}");
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char2()
    {
        assert_eq!(1, char2priority('a'));
        assert_eq!(26, char2priority('z'));
        assert_eq!(27, char2priority('A'));
        assert_eq!(52, char2priority('Z'));
    }

    #[test]
    fn test_line_parse()
    {
        let (left, right) = line2halves("vJrwpWtwJgWrhcsFMMfFFhFp".to_string());
        assert_eq!(left.len(), right.len());
    }

    #[test]
    fn test_cases()
    {
        assert_eq!('p', parse_line_get_common_item("vJrwpWtwJgWrhcsFMMfFFhFp".to_string()));
        assert_eq!('L', parse_line_get_common_item("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string()));
        assert_eq!('P', parse_line_get_common_item("PmmdzqPrVvPwwTWBwg".to_string()));
        assert_eq!('v', parse_line_get_common_item("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string()));
        assert_eq!('t', parse_line_get_common_item("ttgJtRGJQctTZtZT".to_string()));
        assert_eq!('s', parse_line_get_common_item("CrZsJsPPZsGzwwsLwLmpwMDw".to_string()));
    }

    #[test]
    fn test_common()
    {
        assert_eq!(1, get_one_common_value(vec![0, 1, 2], vec![3, 1, 5]));
    }

}
