use crate::error::Result;
use std::collections::HashMap;

pub trait RuckSackData {
    fn get_rucksacks(&self) -> Result<Vec<RuckSack>>;
    fn get_groups(&self) -> Result<Vec<Group>>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuckSack {
    compartment_one: String,
    compartment_two: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    one: String,
    two: String,
    three: String,
}

impl Group {
    pub fn new(one: &str, two: &str, three: &str) -> Self {
        Group {
            one: one.to_string(),
            two: two.to_string(),
            three: three.to_string(),
        }
    }
    pub fn one(&self) -> &str {
        &self.one
    }
    pub fn two(&self) -> &str {
        &self.two
    }
    pub fn three(&self) -> &str {
        &self.three
    }
}

impl RuckSack {
    pub fn new(compartment_one: &str, compartment_two: &str) -> Self {
        RuckSack {
            compartment_one: compartment_one.to_string(),
            compartment_two: compartment_two.to_string(),
        }
    }

    pub fn one(&self) -> &str {
        &self.compartment_one
    }

    pub fn two(&self) -> &str {
        &self.compartment_two
    }

    pub fn all(&self) -> String {
        format!("{}{}", &self.compartment_one, &self.compartment_two)
    }
}

pub struct RucksackSearcher<D: RuckSackData> {
    data: D,
}

impl<D: RuckSackData> RucksackSearcher<D> {
    pub fn new(data: D) -> Self {
        RucksackSearcher { data }
    }

    pub fn get_sum_of_priorities(&self) -> Result<u32> {
        let sum = self
            .get_shared()?
            .into_iter()
            .map(Self::convert_to_priorities)
            .collect::<Option<Vec<u32>>>()
            .ok_or("Invalid char; does not have priority".to_string())?
            .iter()
            .fold(0, |acc, priority| acc + priority);
        Ok(sum)
    }

    pub fn get_sum_of_group_priorities(&self) -> Result<u32> {
        let sum = self
            .data
            .get_groups()?
            .iter()
            .map(Self::badge_from_group)
            .map(|item| item.and_then(Self::convert_to_priorities))
            .collect::<Option<Vec<u32>>>()
            .ok_or("No priority found for a group!".to_string())?
            .iter()
            .sum();
        Ok(sum)
    }

    fn badge_from_group(group: &Group) -> Option<char> {
        let mut badge = None;
        let mut one_hist = HashMap::new();
        let mut two_hist = HashMap::new();
        let one_chars = group.one().chars();
        let two_chars = group.two().chars();
        let three_chars = group.three().chars();
        one_chars.for_each(|item| {
            one_hist.insert(item, true);
        });
        two_chars.for_each(|item| {
            two_hist.insert(item, true);
        });
        three_chars.for_each(|item| {
            if one_hist.contains_key(&item) && two_hist.contains_key(&item) {
                badge = Some(item)
            }
        });
        badge
    }

    // NOTE: This assumes only one value is shared
    fn get_shared(&self) -> Result<Vec<char>> {
        let rucksacks = self.data.get_rucksacks()?;
        let shared = rucksacks
            .iter()
            .map(|rucksack| {
                let mut shared = None;
                let mut one_hist = HashMap::new();
                for char in rucksack.one().chars() {
                    one_hist.insert(char, true);
                }
                for char in rucksack.two().chars() {
                    if one_hist.contains_key(&char) {
                        shared = Some(char)
                    }
                }
                shared
            })
            .collect::<Option<_>>()
            .ok_or("One of the rucksacks didn't have a shared item".to_string())?;
        Ok(shared)
    }

    fn convert_to_priorities(item: char) -> Option<u32> {
        match item {
            'a' => Some(1),
            'b' => Some(2),
            'c' => Some(3),
            'd' => Some(4),
            'e' => Some(5),
            'f' => Some(6),
            'g' => Some(7),
            'h' => Some(8),
            'i' => Some(9),
            'j' => Some(10),
            'k' => Some(11),
            'l' => Some(12),
            'm' => Some(13),
            'n' => Some(14),
            'o' => Some(15),
            'p' => Some(16),
            'q' => Some(17),
            'r' => Some(18),
            's' => Some(19),
            't' => Some(20),
            'u' => Some(21),
            'v' => Some(22),
            'w' => Some(23),
            'x' => Some(24),
            'y' => Some(25),
            'z' => Some(26),
            'A' => Some(27),
            'B' => Some(28),
            'C' => Some(29),
            'D' => Some(30),
            'E' => Some(31),
            'F' => Some(32),
            'G' => Some(33),
            'H' => Some(34),
            'I' => Some(35),
            'J' => Some(36),
            'K' => Some(37),
            'L' => Some(38),
            'M' => Some(39),
            'N' => Some(40),
            'O' => Some(41),
            'P' => Some(42),
            'Q' => Some(43),
            'R' => Some(44),
            'S' => Some(45),
            'T' => Some(46),
            'U' => Some(47),
            'V' => Some(48),
            'W' => Some(49),
            'X' => Some(50),
            'Y' => Some(51),
            'Z' => Some(52),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestRucksackData {
        data: Vec<RuckSack>,
    }

    impl From<Vec<(&str, &str)>> for TestRucksackData {
        fn from(rucksacks: Vec<(&str, &str)>) -> Self {
            let data = rucksacks
                .iter()
                .map(|(compartment_one, compartment_two)| {
                    RuckSack::new(compartment_one, compartment_two)
                })
                .collect();
            TestRucksackData { data }
        }
    }

    impl RuckSackData for TestRucksackData {
        fn get_rucksacks(&self) -> Result<Vec<RuckSack>> {
            Ok(self.data.clone())
        }

        fn get_groups(&self) -> Result<Vec<Group>> {
            self.get_rucksacks()?
                .chunks(3)
                .map(group_from_three_chunk)
                .collect()
        }
    }

    fn group_from_three_chunk(chunk: &[RuckSack]) -> Result<Group> {
        if chunk.len() == 3 {
            let one = chunk[0].all();
            let two = chunk[1].all();
            let three = chunk[2].all();
            let group = Group { one, two, three };
            Ok(group)
        } else {
            Err("Groups are made from 3 rucksacks!".to_string())
        }
    }

    #[test]
    fn can_find_rucksack_shared() {
        let test_data: TestRucksackData = vec![
            ("a", "a"),
            ("ba", "bc"),
            ("Ba", "BC"),
            ("ABCDEFGHIJKLMNO", "QRSTUVXXabcdefA"),
        ]
        .into();
        let rucksack_searcher = RucksackSearcher::new(test_data);

        let expected = 1 + 2 + 28 + 27;
        let actual = rucksack_searcher.get_sum_of_priorities().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_find_group_shared() {
        let test_data: TestRucksackData = vec![
            ("a", "aR"),
            ("bRRR", "b"),
            ("c", "Rc"),
            ("d", "dx"),
            ("ex", "e"),
            ("xxxf", "f"),
        ]
        .into();
        let rucksack_searcher = RucksackSearcher::new(test_data);

        let expected = 44 + 24;
        let actual = rucksack_searcher.get_sum_of_group_priorities().unwrap();
        assert_eq!(expected, actual);
    }
}
