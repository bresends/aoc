use std::collections::HashMap;
/*
First Challenge

Opponent Plays
A Rock
B Paper
C Scissors

My plays
X Rock
Y Paper
Z Scissors
*/

pub fn process_part1(input: &str) -> u32 {
    input.lines().map(calculate_firt_duel_score).sum()
}

/*
Second Challenge

Opponent Plays
A Rock
B Paper
C Scissors

Outcome
X Lose
Y Draw
C Win
*/

pub fn process_part2(input: &str) -> u32 {
    input.lines().map(calculate_second_duel_score).sum()
}

fn calculate_firt_duel_score(duel: &str) -> u32 {
    let mut moves = duel.split_whitespace();

    let opponent_move = moves.next().unwrap();
    let my_move = moves.next().unwrap();

    let my_move_values = HashMap::from([("X", 3), ("Y", 2), ("Z", 1)]);

    let game_values = HashMap::from([
        ("A", HashMap::from([("X", 3), ("Y", 6), ("Z", 0)])),
        ("B", HashMap::from([("X", 0), ("Y", 3), ("Z", 6)])),
        ("C", HashMap::from([("X", 6), ("Y", 0), ("Z", 3)])),
    ]);

    let my_move_score = my_move_values[my_move];
    let game_score = game_values[opponent_move][my_move];

    my_move_score + game_score
}

fn calculate_second_duel_score(duel: &str) -> u32 {
    let mut moves = duel.split_whitespace();

    let opponent_move = moves.next().unwrap();
    let game_outcome = moves.next().unwrap();

    let my_move_values = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let game_outcome_values = HashMap::from([("X", 0), ("Y", 3), ("Z", 6)]);

    let required_move = HashMap::from([
        ("A", HashMap::from([("X", "C"), ("Y", "A"), ("Z", "B")])),
        ("B", HashMap::from([("X", "A"), ("Y", "B"), ("Z", "C")])),
        ("C", HashMap::from([("X", "B"), ("Y", "C"), ("Z", "A")])),
    ]);

    let my_move = required_move[opponent_move][game_outcome];
    let game_score = game_outcome_values[game_outcome];
    let my_move_score = my_move_values[my_move];

    my_move_score + game_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
    B X
    C Z";

    #[test]
    fn part_1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 15);
    }

    #[test]
    fn part_2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 12);
    }
}
