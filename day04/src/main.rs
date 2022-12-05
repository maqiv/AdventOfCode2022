use shared::read_input;

fn main() {
    let q0_result = quest0();
    println!("{q0_result} ranges in the assignment pairs contain the other");

    let q1_result = quest1();
    println!("{q1_result} ranges in the assignment pairs overlap");
}

fn quest1() -> i32 {
    let mut overlapping_pairs = 0;

    for sp in get_section_pairs() {
        if sp.first.lower <= sp.second.upper && sp.first.upper >= sp.second.lower
        {
            overlapping_pairs += 1;
        }
    }

    overlapping_pairs
}

fn quest0() -> i32 {
    let mut full_range_pairs_cnt = 0;

    for sp in get_section_pairs() {
        if (sp.first.lower >= sp.second.lower && sp.first.upper <= sp.second.upper)
            || (sp.second.lower >= sp.first.lower && sp.second.upper <= sp.first.upper)
        {
            full_range_pairs_cnt += 1;
        }
    }

    full_range_pairs_cnt
}

fn get_section_pairs() -> Vec<SectionPair> {
    let mut section_pairs: Vec<SectionPair> = Vec::new();
    let lines = read_input("./src/input");

    for l in lines {
        let sp = parse_section_pair(l);
        section_pairs.push(sp);
    }

    section_pairs
}

fn parse_section_pair(l: String) -> SectionPair {
    let parsed: Vec<&str> = l.split(",").collect();

    SectionPair {
        first: SectionRange::from(parsed[0]),
        second: SectionRange::from(parsed[1]),
    }
}

struct SectionPair {
    first: SectionRange,
    second: SectionRange,
}

struct SectionRange {
    lower: i32,
    upper: i32,
}

impl From<&str> for SectionRange {
    fn from(s: &str) -> Self {
        let s_ranges: Vec<&str> = s.split("-").collect();

        SectionRange {
            lower: s_ranges[0].parse().expect("This should be a number"),
            upper: s_ranges[1].parse().expect("This should be a number"),
        }
    }
}
