use std::fs::File;
use std::io::{BufRead, BufReader};

type Item = char;
type Priority = u8;

fn get_priority(item: Item) -> Result<Priority, ()> {
    match item {
        'a'..='z' => Ok((item as u8) - b'a' + 1),
        'A'..='Z' => Ok((item as u8) - b'A' + 27),
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests_for_get_priority {
    use super::get_priority;

    #[test]
    fn priorities_look_right() {
        assert_eq!(get_priority('a'), Ok(1));
        assert_eq!(get_priority('b'), Ok(2));
        assert_eq!(get_priority('z'), Ok(26));

        assert_eq!(get_priority('A'), Ok(27));
        assert_eq!(get_priority('B'), Ok(28));
        assert_eq!(get_priority('Z'), Ok(52));

        assert!(get_priority('£').is_err());
    }
}

struct RucksackSearchingElf {
    contents_of_current_rucksack: [bool; 52],
}

impl RucksackSearchingElf {
    fn new() -> Self {
        RucksackSearchingElf { contents_of_current_rucksack: [false; 52] }
    }

    fn search_rucksack(&mut self, rucksack: &str) -> Option<Priority> {
        // Reset from any previous searches
        self.contents_of_current_rucksack = [false; 52];

        let contents = rucksack.chars();
        let number_of_items = rucksack.len();
        for (index, item) in contents.enumerate() {
            let priority = get_priority(item).expect("invalid rucksack item!");
            let priority_index = (priority - 1) as usize;
            let is_in_first_half = index < number_of_items / 2; 
            if is_in_first_half {
                self.contents_of_current_rucksack[priority_index] = true;
            } else {
                let is_already_in_rucksack = self.contents_of_current_rucksack[priority_index];
                if is_already_in_rucksack { return Some(priority); }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests_rucksack_searching_elf {
    use super::get_priority;
    use super::RucksackSearchingElf;

    macro_rules! should_find {
        ($elf: ident, $rucksack: literal, $item: literal) => {
            let priority = get_priority($item).unwrap();
            assert_eq!($elf.search_rucksack($rucksack), Some(priority), "fix priority for rucksack {} should be {} (item {})", $rucksack, priority, $item);
        }
    }

    #[test]
    fn repeated_items_get_found() {
        let mut elf = RucksackSearchingElf::new();
        should_find!(elf, "vJrwpWtwJgWrhcsFMMfFFhFp", 'p');
        should_find!(elf, "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L');
        should_find!(elf, "PmmdzqPrVvPwwTWBwg", 'P');
        should_find!(elf, "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 'v');
        should_find!(elf, "ttgJtRGJQctTZtZT", 't');
        should_find!(elf, "CrZsJsPPZsGzwwsLwLmpwMDw", 's');
    }
}

fn main() {
    let input_path = std::env::args()
        .nth(1)
        .expect("input path must be provided as a command line argument");
    let input_path = std::env::current_dir()
        .expect("unable to get current working directory")
        .join(&input_path);
    let input = File::open(input_path).expect("unable to open input file");
    let lines = BufReader::new(input).lines();

    let mut sum_of_priorities: u32 = 0;
    let mut elf = RucksackSearchingElf::new();
    for line in lines {
        let rucksack = line.expect("unable to read rucksack contents");
        let priority = elf.search_rucksack(&rucksack)
            .expect("all rucksacks should have a repeated item");
        sum_of_priorities += priority as u32;
    }
    dbg!(sum_of_priorities);
}
