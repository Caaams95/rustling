/// Write up
/// Ici nous utilisons des hashmap afin de stocker chaque données.
/// Une Hashmap est une structure de données dans laquelle des paires clé-valeur sont stockées
///  de manière à permettre un accès rapide à la valeur associée à une clé particulière.
/// 
/// Explication de code:
///     let team1 = scores.entry(team_1_name.clone()).or_insert(Team {
/// 
/// team_1_name.clone(): Clone le nom de l'équipe 1. 
/// Cela est nécessaire car team_1_name est de type String, 
/// et lorsqu'on insère une valeur dans une Hashmap, la propriété (ownership) est transférée. 
/// En clonant, on évite de transférer la propriété et on crée une nouvelle chaîne avec le même contenu.
///
/// scores.entry(team_1_name.clone()): Utilise la méthode entry sur la Hashmap scores pour 
/// récupérer une entrée correspondant à la clé team_1_name. 
/// Si l'entrée existe, elle est retournée. 
/// Sinon, une nouvelle entrée est créée avec la clé team_1_name et une valeur par défaut.
///
/// .or_insert(Team { goals_scored: 0, goals_conceded: 0 }): Si l'entrée correspondante n'existe pas,
/// cette méthode insère la valeur par défaut spécifiée (une nouvelle instance de la structure 
/// Team avec goals_scored et goals_conceded initialisés à 0) et renvoie une référence 
/// mutable à cette valeur.
/// Si l'entrée existait déjà, elle renvoie une référence mutable à la valeur existante.
/// 
/// Gestion des goals :
///     team1.goals_scored += team_1_score;
///     team1.goals_conceded += team_2_score;
/// On fait une simple addition de la variable goals_XXX de la team en question
/// 
/// 
// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.


use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded by team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.

        let team1 = scores.entry(team_1_name.clone()).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team1.goals_scored += team_1_score;
        team1.goals_conceded += team_2_score;

        let team2 = scores.entry(team_2_name.clone()).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team2.goals_conceded += team_1_score;
        team2.goals_scored += team_2_score;


        
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
