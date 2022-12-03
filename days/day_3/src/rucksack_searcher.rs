use crate::error::Result;
use std::collections::HashMap;

pub trait RuckSackData {
    fn get_rucksacks(&self) -> Result<Vec<RuckSack>>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuckSack {
    compartment_one: String,
    compartment_two: String,
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
            .collect::<Result<Vec<u32>>>()?
            .iter()
            .fold(0, |acc, priority| acc + priority);
        Ok(sum)
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

    fn convert_to_priorities(item: char) -> Result<u32> {
        match item {
            'a' => Ok(1),
            'b' => Ok(2),
            'c' => Ok(3),
            'd' => Ok(4),
            'e' => Ok(5),
            'f' => Ok(6),
            'g' => Ok(7),
            'h' => Ok(8),
            'i' => Ok(9),
            'j' => Ok(10),
            'k' => Ok(11),
            'l' => Ok(12),
            'm' => Ok(13),
            'n' => Ok(14),
            'o' => Ok(15),
            'p' => Ok(16),
            'q' => Ok(17),
            'r' => Ok(18),
            's' => Ok(19),
            't' => Ok(20),
            'u' => Ok(21),
            'v' => Ok(22),
            'w' => Ok(23),
            'x' => Ok(24),
            'y' => Ok(25),
            'z' => Ok(26),
            'A' => Ok(27),
            'B' => Ok(28),
            'C' => Ok(29),
            'D' => Ok(30),
            'E' => Ok(31),
            'F' => Ok(32),
            'G' => Ok(33),
            'H' => Ok(34),
            'I' => Ok(35),
            'J' => Ok(36),
            'K' => Ok(37),
            'L' => Ok(38),
            'M' => Ok(39),
            'N' => Ok(40),
            'O' => Ok(41),
            'P' => Ok(42),
            'Q' => Ok(43),
            'R' => Ok(44),
            'S' => Ok(45),
            'T' => Ok(46),
            'U' => Ok(47),
            'V' => Ok(48),
            'W' => Ok(49),
            'X' => Ok(50),
            'Y' => Ok(51),
            'Z' => Ok(52),
            _ => Err(format!("{} does not have a priority", item)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestData {
        data: Vec<RuckSack>,
    }

    impl From<Vec<(&str, &str)>> for TestData {
        fn from(rucksacks: Vec<(&str, &str)>) -> Self {
            let data = rucksacks
                .iter()
                .map(|(compartment_one, compartment_two)| {
                    RuckSack::new(compartment_one, compartment_two)
                })
                .collect();
            TestData { data }
        }
    }

    impl RuckSackData for TestData {
        fn get_rucksacks(&self) -> Result<Vec<RuckSack>> {
            Ok(self.data.clone())
        }
    }

    #[test]
    fn can_find_shared() {
        let test_data: TestData = vec![
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
}
