use crate::error::Result;
use crate::rucksack_searcher::{RuckSack, RuckSackData};

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rucksacks_from_string() {
        let data = "aa\nbaba\nNJnjmc\n";

        let string_data = StringData::new(data);

        let expected: Vec<RuckSack> = vec![
            RuckSack::new("a", "a"),
            RuckSack::new("ba", "ba"),
            RuckSack::new("NJn", "jmc"),
        ];
        let actual = string_data.get_rucksacks().unwrap();

        assert_eq!(expected, actual);
    }
}
