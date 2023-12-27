fn get_group_sum(input: &str) -> u32 {
    input
        .lines()
        .fold(0, |acc, cur| acc + cur.parse::<u32>().unwrap())
}
pub fn process_part1(input: &str) -> u32 {
    input.split("\n\n").map(get_group_sum).max().unwrap()
}

// pub fn process_part2(input: &str) -> u32 {
//     let mut elves_calores = input.split("\n\n").map(get_group_sum).collect::<Vec<u32>>();
//
//     elves_calores.sort_by(|a, b| b.cmp(a));
//     elves_calores.iter().take(3).sum()
// }
//
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part_1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 24000);
    }

    //     #[test]
    //     fn part_2_works() {
    //         let result = process_part2(INPUT);
    //         assert_eq!(result, 45000);
    //     }
}
