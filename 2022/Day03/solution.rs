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

struct RucksackContents {
    contents: [bool; 52],
}
impl RucksackContents {
    fn new() -> Self { RucksackContents { contents: [false; 52] } }
    fn add(&mut self, priority: &Priority) { self.contents[(priority - 1) as usize] = true }
    fn has(&mut self, priority: &Priority) -> bool { self.contents[(priority - 1) as usize] }
    fn reset(&mut self) { self.contents = [false; 52] }
}

struct RucksackSearchingElf {
    contents_of_current_rucksack: RucksackContents,
}

impl RucksackSearchingElf {
    fn new() -> Self {
        RucksackSearchingElf { contents_of_current_rucksack: RucksackContents::new() }
    }

    fn search_rucksack(&mut self, rucksack: &str) -> Option<Priority> {
        // Reset from any previous searches
        self.contents_of_current_rucksack.reset();

        let contents = rucksack.chars();
        let number_of_items = rucksack.len();
        for (index, item) in contents.enumerate() {
            let priority = get_priority(item).expect("invalid rucksack item!");
            let is_in_first_half = index < number_of_items / 2;
            if is_in_first_half {
                self.contents_of_current_rucksack.add(&priority);
            } else {
                let is_already_in_rucksack = self.contents_of_current_rucksack.has(&priority);
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

struct AuthenticatorElf {
    rucksack_one: RucksackContents,
    rucksack_two: RucksackContents,
}

impl AuthenticatorElf {
    fn new() -> Self {
        AuthenticatorElf {
            rucksack_one: RucksackContents::new(),
            rucksack_two: RucksackContents::new(),
        }
    }

    fn inspect_rucksack_one(&mut self, rucksack: &str) {
        // Reset from any previous searches
        self.rucksack_one.reset();
        for item in rucksack.chars() {
            let priority = get_priority(item).expect("invalid rucksack item!");
            self.rucksack_one.add(&priority);
        }
    }
    fn inspect_rucksack_two(&mut self, rucksack: &str) {
        // Reset from any previous searches
        self.rucksack_two.reset();
        for item in rucksack.chars() {
            let priority = get_priority(item).expect("invalid rucksack item!");
            self.rucksack_two.add(&priority);
        }
    }
    fn inspect_rucksack_three(&mut self, rucksack: &str) -> Option<Priority> {
        rucksack.chars()
            .map(|item| get_priority(item).expect("invalid rucksack item!"))
            .filter(|priority| self.rucksack_one.has(priority))
            .find(|priority| self.rucksack_two.has(priority))
    }
}

#[cfg(test)]
mod tests_authenticator_elf {
    use super::get_priority;
    use super::AuthenticatorElf;

    macro_rules! should_authenticate {
        ($elf: ident, $rucksack_one: literal, $rucksack_two: literal, $rucksack_three: literal, $item: literal) => {
            let expected_priority = get_priority($item).unwrap();
            $elf.inspect_rucksack_one($rucksack_one);
            $elf.inspect_rucksack_two($rucksack_two);
            assert_eq!($elf.inspect_rucksack_three($rucksack_three), Some(expected_priority), "authentication priority for {} / {} / {} should be {} (item {})", $rucksack_one, $rucksack_two, $rucksack_three, expected_priority, $item);
        }
    }

    #[test]
    fn it_does_the_examples() {
        let mut elf = AuthenticatorElf::new();
        should_authenticate!(elf,
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            'r'
        );
        should_authenticate!(elf,
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
            'Z'
        );
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

    let mut sum_of_search_priorities: u32 = 0;
    let mut sum_of_auth_priorities: u32 = 0;
    let mut search_elf = RucksackSearchingElf::new();
    let mut auther_elf = AuthenticatorElf::new();
    for (index, line) in lines.enumerate() {
        let rucksack = line.expect("unable to read rucksack contents");
        let priority = search_elf.search_rucksack(&rucksack)
            .expect("all rucksacks should have a repeated item");
        sum_of_search_priorities += priority as u32;

        match index % 3 {
            0 => auther_elf.inspect_rucksack_one(&rucksack),
            1 => auther_elf.inspect_rucksack_two(&rucksack),
            2 => {
                let priority = auther_elf.inspect_rucksack_three(&rucksack)
                    .expect("all groups should have a common authentication badge");
                sum_of_auth_priorities += priority as u32;
            }
            _ => unreachable!("modulo arithmetic is broken"),
        }
    }
    dbg!(sum_of_search_priorities);
    dbg!(sum_of_auth_priorities);
}
