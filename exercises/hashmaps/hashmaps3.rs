// hashmaps3.rs

// A list of scores (one per line) of a soccer match is given. Each line
// is of the form :
// <team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>
// Example: England,France,4,2 (England scored 4 goals, France 2).

// You have to build a scores table containing the name of the team, goals
// the team scored, and goals the team conceded. One approach to build
// the scores table is to use a Hashmap. The solution is partially
// written to use a Hashmap, complete it to pass the test.

// Make me pass the tests!

// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a hint.


use std::collections::HashMap;

// A structure to store team name and its goal details.
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String)  {
   
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn get_results() -> String {
    //     let results = "".to_string()
    //         + "England,France,4,2\n"
    //         + "France,Italy,3,1\n"
    //         + "Poland,Spain,2,0\n"
    //         + "Germany,England,2,1\n";
    //     results
    // }

    // #[test]
    // fn build_scores() {
    //     let scores = build_scores_table(get_results());

    //     let mut keys: Vec<&String> = scores.keys().collect();
    //     keys.sort();
    //     assert_eq!(
    //         keys,
    //         vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
    //     );
    // }

    // #[test]
    // fn validate_team_score_1() {
    //     let scores = build_scores_table(get_results());
    //     let team = scores.get("England").unwrap();
    //     assert_eq!(team.goals_scored, 5);
    //     assert_eq!(team.goals_conceded, 4);
    // }

    // #[test]
    // fn validate_team_score_2() {
    //     let scores = build_scores_table(get_results());
    //     let team = scores.get("Spain").unwrap();
    //     assert_eq!(team.goals_scored, 0);
    //     assert_eq!(team.goals_conceded, 2);
    // }
}
