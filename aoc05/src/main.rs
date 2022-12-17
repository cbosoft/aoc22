use std::collections::hash_map::RandomState;
use std::fs::File;
use std::io::{BufReader, Result, Lines, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::collections::HashSet;
use std::usize;
use regex::{Regex, Match, CaptureMatches};

struct State {
    pub stacks: Vec<Vec<char>>
}

impl State {
    pub fn new() -> Self {
        Self{stacks: Vec::new()}
    }

    pub fn apply_command(&mut self, number: usize, source: usize, destination: usize) {
        for _ in 0..number {
            let c = self.stacks[source].pop().unwrap();
            self.stacks[destination].push(c);
        }
    }

    pub fn tops(self) -> String {
        let mut rv = String::new();
        for stack in self.stacks {
            if let Some(c) = stack.last() {
                rv.push(*c);
            }
            else {
                rv.push(' ');
            }
        }
        rv
    }
}


fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn parse_state(lines: &Vec<String>) -> State {
    let mut rv = State::new();

    for line in lines.iter().rev() {
        if line.is_empty() || line.starts_with(" 1") {
            continue;
        }

        let n_stacks = (line.len() + 1) / 4;

        if rv.stacks.is_empty() {
            for _ in 0..n_stacks { rv.stacks.push(Vec::new()); }
        }

        let chars: Vec<char> = line.chars().collect();
        for i_stack in 0..n_stacks {
            let i_char = i_stack * 4 + 1;
            if i_char >= chars.len() { continue; }
            let c = chars[i_char];
            if c != ' ' {
                rv.stacks[i_stack].push(c);
            }
        }
    }

    rv
}

fn parse_state_and_run_commands(lines: Lines<BufReader<File>>) -> State
{
    let mut state_lines = Vec::new();
    let mut state: Option<State> = None;

    let command_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in lines {
        let line = line.unwrap();
        if let Some(m) = command_re.captures(&line) {
            if state.is_none() {
                state = Some(parse_state(&state_lines));
            }
            let number = usize::from_str(&m[1]).unwrap();
            let source = usize::from_str(&m[2]).unwrap() - 1;
            let destination = usize::from_str(&m[3]).unwrap() - 1;

            state.as_mut().unwrap().apply_command(number, source, destination);
        }
        else {
            assert!(state.is_none());

            state_lines.push(line);
        }
    }

    state.unwrap()
}

fn main()
{
    let lines = read_lines("crate_state_and_commands.txt");
    let result = parse_and_run_commands(lines.unwrap());
    println!("Part 1: {}", result.tops());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_state() {
        let state_lines = vec![
            "    [V] [G]             [H]".to_string(),
            "[Z] [H] [Z]         [T] [S]".to_string(),
            "[P] [D] [F]         [B] [V] [Q]".to_string(),
            "[B] [M] [V] [N]     [F] [D] [N]".to_string(),
            "[Q] [Q] [D] [F]     [Z] [Z] [P] [M]".to_string(),
            "[M] [Z] [R] [D] [Q] [V] [T] [F] [R]".to_string(),
            "[D] [L] [H] [G] [F] [Q] [M] [G] [W]".to_string(),
            "[N] [C] [Q] [H] [N] [D] [Q] [M] [B]".to_string(),
            " 1   2   3   4   5   6   7   8   9 ".to_string(),
        ];

        let state = parse_state(&state_lines);

        assert_eq!(state.stacks.len(), 9);
        assert_eq!(state.stacks[0], vec!['N', 'D', 'M', 'Q', 'B', 'P', 'Z']);
        assert_eq!(state.stacks[1], vec!['C', 'L', 'Z', 'Q', 'M', 'D', 'H', 'V']);
        assert_eq!(state.stacks[2], vec!['Q', 'H', 'R', 'D', 'V', 'F', 'Z', 'G']);
        assert_eq!(state.stacks[3], vec!['H', 'G', 'D', 'F', 'N']);
        assert_eq!(state.stacks[4], vec!['N', 'F', 'Q']);
        assert_eq!(state.stacks[5], vec!['D', 'Q', 'V', 'Z', 'F', 'B', 'T']);
        assert_eq!(state.stacks[6], vec!['Q', 'M', 'T', 'Z', 'D', 'V', 'S', 'H']);
        assert_eq!(state.stacks[7], vec!['M', 'G', 'F', 'P', 'N', 'Q']);
        assert_eq!(state.stacks[8], vec!['B', 'W', 'R', 'M']);

        assert_eq!(state.tops(), "ZVGNQTHQM".to_string());
    }
}