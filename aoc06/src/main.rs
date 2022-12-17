use std::collections::{HashSet, hash_map::RandomState};
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::str::Chars;
use utf8_chars::BufReadCharsExt;

struct PacketBuff {
    buf: Vec<char>,
}

impl PacketBuff {
    pub fn new() -> Self {
        Self{buf: Vec::new()}
    }

    fn remove_old(&mut self) {
        let old = self.buf.remove(0);
    }

    pub fn push_char(&mut self, c: char) -> bool {
        if self.buf.len() > 3 {
            self.remove_old()
        }

        self.buf.push(c);

        if self.buf.len() > 3 {
            let set: HashSet<char, RandomState> = HashSet::from_iter(self.buf.iter().map(|cr|*cr));
            set.len() == 4
        }
        else {
            false
        }
    }
}

fn get_start_packet_pos<I>(chars: I) -> usize
where I: Iterator<Item=char>
{
    let mut buf = PacketBuff::new();

    for (i, ch) in chars.enumerate() {
        if buf.push_char(ch) {
            return i + 1
        }
    }

    panic!("no start packet found!");
}

fn main() {
    let file = File::open("data.txt").unwrap();
    let mut rdr = BufReader::new(file);
    let p = get_start_packet_pos(rdr.chars().map(|cr|cr.unwrap()));

    println!("Start packet at p={}", p);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(5, get_start_packet_pos("bvwbjplbgvbhsrlpgdmjqwftvncz".chars()));
        assert_eq!(6, get_start_packet_pos("nppdvjthqldpwncqszvftbrmjlhg".chars()));
        assert_eq!(10, get_start_packet_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars()));
        assert_eq!(11, get_start_packet_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars()));
    }
}
