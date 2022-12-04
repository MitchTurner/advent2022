use crate::error::Result;
use crate::rucksack_searcher::{Group, RuckSack, RuckSackData};

pub struct StringData {
    inner: String,
}

impl StringData {
    pub fn new(inner: &str) -> Self {
        StringData {
            inner: inner.to_string(),
        }
    }

    fn rucksack_from_line(line: &str) -> Result<RuckSack> {
        let len = line.len();
        if len % 2 == 0 {
            let half = len / 2;
            let one = &line[..half];
            let two = &line[half..];
            let rucksack = RuckSack::new(one, two);
            Ok(rucksack)
        } else {
            Err("Odd amount in rucksack compartment!".to_string())
        }
    }
}

impl RuckSackData for StringData {
    fn get_rucksacks(&self) -> Result<Vec<RuckSack>> {
        self.inner.lines().map(Self::rucksack_from_line).collect()
    }

    fn get_groups(&self) -> Result<Vec<Group>> {
        self.inner
            .lines()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(group_from_three_chunk)
            .collect()
    }
}

fn group_from_three_chunk(chunk: &[&str]) -> Result<Group> {
    if chunk.len() == 3 {
        let one = chunk[0];
        let two = chunk[1];
        let three = chunk[2];
        let group = Group::new(one, two, three);
        Ok(group)
    } else {
        Err("Groups are made from 3 rucksacks!".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rucksacks_from_string() {
        let data = "aa\nbaba\nNJnjmc\n";

        let string_data = StringData::new(data);

        let expected = vec![
            RuckSack::new("a", "a"),
            RuckSack::new("ba", "ba"),
            RuckSack::new("NJn", "jmc"),
        ];
        let actual = string_data.get_rucksacks().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn groups_from_string() {
        let data = "aa\nbaba\nNJnjmc\nsjklf\nWEKNVBe\nEKJFej";

        let string_data = StringData::new(data);

        let expected = vec![
            Group::new("aa", "baba", "NJnjmc"),
            Group::new("sjklf", "WEKNVBe", "EKJFej"),
        ];
        let actual = string_data.get_groups().unwrap();

        assert_eq!(expected, actual);
    }
}
