use crate::common::Solution;

pub struct DayX;

impl Solution for DayX {
    type Parsed = Vec<String>;
    type PartOneOutput = u64;
    type PartTwoOutput = u64;

    fn parse(input: String) -> Self::Parsed {
        todo!()
    }

    fn part_one(data: &mut Self::Parsed) -> Self::PartOneOutput {
        todo!()
    }

    fn part_two(data: &mut Self::Parsed) -> Self::PartTwoOutput {
        todo!()
    }
}

impl DayX {
    fn util(data: <DayX as Solution>::Parsed) {}
}
