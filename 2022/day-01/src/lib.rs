fn get_group_sum(input: &str) -> u32 {
    input
        .split("\n")
        .fold(0, |acc, cur| acc + cur.parse::<u32>().unwrap())
}
pub fn process_part1(input: &str) -> u32 {
    input.split("\n\n").map(get_group_sum).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let result = process_part1(input);
        assert_eq!(result, 24000);
    }
}
