use crate::error::Result;

pub trait Reader {
    fn read_inventory(&self) -> Result<Vec<Vec<u32>>>;
}

pub struct InventoryCounter<R: Reader> {
    reader: R
}

impl<R: Reader> InventoryCounter<R> {
    pub fn new(reader: R) -> Self {
        InventoryCounter {
            reader
        }
    }

    pub fn highest_elf_calories_held(&self) -> Result<u32> {
        let inventory = self.reader.read_inventory()?;
        let mut highest = 0;

        for elf in inventory {
            let elf_cals = elf.iter().fold(0, |acc, cals| acc + cals );
            if elf_cals > highest {
                highest = elf_cals
            }
        }
        Ok(highest)
    }

    pub fn top_three_elf_calories_held(&self) -> Result<u32> {
        let inventory = self.reader.read_inventory()?;
        let mut one = 0;
        let mut two = 0;
        let mut three = 0;

        for elf in inventory {
            let elf_cals = elf.iter().fold(0, |acc, cals| acc + cals );
            if elf_cals > three {
                if elf_cals > two {
                    if elf_cals > one{
                        three = two;
                        two = one;
                        one = elf_cals;
                    } else {
                        three = two;
                        two = elf_cals;
                    }
                } else {
                    three = elf_cals
                }
            } else {
            }
        }
        Ok(one + two + three)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeReader {
        inventory: Vec<Vec<u32>>
    }

    impl FakeReader {
        fn new(inventory: Vec<Vec<u32>>) -> Self {
            FakeReader {
                inventory
            }
        }
    }

    impl Reader for FakeReader {
        fn read_inventory(&self) -> Result<Vec<Vec<u32>>> {
            Ok(self.inventory.clone())
        }
    }

    #[test]
    fn can_get_highest() {
        // given
        let inventory = vec![vec![1,2,3], vec![3,4,5], vec![1]];

        let reader = FakeReader::new(inventory);
        let inventory_counter = InventoryCounter::new(reader);

        // when
        let actual = inventory_counter.highest_elf_calories_held().unwrap();

        // then
        let expected: u32 = 3 + 4 + 5;
        assert_eq!(expected, actual)
    }

    #[test]
    fn can_get_top_three() {
        // given
        let inventory = vec![vec![1,2,3], vec![3,4,5], vec![1], vec![2,3,2]];

        let reader = FakeReader::new(inventory);
        let inventory_counter = InventoryCounter::new(reader);

        // when
        let actual = inventory_counter.top_three_elf_calories_held().unwrap();

        // then
        let expected: u32 = (3 + 4 + 5)+ (2 + 3 + 2) + (1 + 2 + 3);
        assert_eq!(expected, actual)
    }
}
