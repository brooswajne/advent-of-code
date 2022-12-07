use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct SectionAssignment {
    first_section: u16,
    final_section: u16,
}

impl From<&str> for SectionAssignment {
    fn from(assignment: &str) -> Self {
        let mut sections = assignment.split('-');
        let first_section = sections
            .next()
            .expect("section assignment is missing a start section")
            .parse()
            .expect("section assignment's first section cannot be parsed as an integer");
        let final_section = sections
            .next()
            .expect("section assignment is missing a final section")
            .parse()
            .expect("section assignment's final section cannot be parsed as an integer");
        if sections.next().is_some() {
            panic!("section assignment has more than two sections");
        }
        assert!(first_section <= final_section, "section assignment is backwards");
        SectionAssignment { first_section, final_section }
    }
}

impl SectionAssignment {
    fn fully_contains(&self, other: &SectionAssignment) -> bool {
        self.first_section <= other.first_section
            && other.final_section <= self.final_section
    }
}

#[cfg(test)]
mod test {
    use super::SectionAssignment;

    #[test]
    fn test_full_containment_examples() {
        macro_rules! assert_containment {
            ($one: literal, $two: literal) => {
                let one: SectionAssignment = $one.into();
                let two: SectionAssignment = $two.into();
                assert!(one.fully_contains(&two), "{:?} should fully contain {:?}", one, two);
            }
        }
        assert_containment!("2-8", "3-7");
        assert_containment!("4-6", "6-6");
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

    let mut num_full_containments = 0;
    for line in lines {
        let line = line.expect("unable to read input line");
        let mut assignments = line.split(',');
        let assignment_one: SectionAssignment = assignments
            .next()
            .expect("line is missing first section assignment")
            .into();
        let assignment_two: SectionAssignment = assignments
            .next()
            .expect("line is missing second section assignment")
            .into();
        if assignments.next().is_some() {
            panic!("line has more than two section assignments");
        }
        if assignment_one.fully_contains(&assignment_two) || assignment_two.fully_contains(&assignment_one) {
            num_full_containments += 1;
        }
    }
    dbg!(num_full_containments);
}
