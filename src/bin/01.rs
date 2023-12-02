advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    Some(input.chars().map(|c| match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }).sum::<i32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut counter = 0;
    let mut result = 0;
    for (i, n) in input.chars().map(|c| match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }).enumerate() {
        counter += n;
        if counter < 0 {
            result = i + 1;
            break;
        }
    }
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(-3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
