advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| -> u32 {
                let dimensions: Vec<u32> =
                    line.split('x').map(|n| n.parse::<u32>().unwrap()).collect();
                let mut areas = [
                    dimensions[0] * dimensions[1],
                    dimensions[1] * dimensions[2],
                    dimensions[0] * dimensions[2],
                ];
                areas.sort_unstable();
                areas.iter().sum::<u32>() * 2 + areas[0]
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| -> u32 {
                let mut dimensions: Vec<u32> =
                    line.split('x').map(|n| n.parse::<u32>().unwrap()).collect();
                dimensions.sort_unstable();
                dimensions.iter().product::<u32>() + 2 * (dimensions[0] + dimensions[1])
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(58));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
