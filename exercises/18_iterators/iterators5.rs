/// Write UP
///     map.iter().fold(0, |acc, (_, v)| { if v == &value {acc + 1} else {acc}})
/// Cette ligne itère sur les valeurs de la HashMap (map), comptant combien d'entre 
/// elles sont égales à la valeur spécifiée. Le résultat est le décompte total des 
/// occurrences.
///  
/// - map.iter(): crée un itérateur sur les paires clé-valeur de la HashMap (map).
/// - .fold(0, |acc, (_, v)| { if v == &value {acc + 1} else {acc}}): La méthode fold 
/// est utilisée pour accumuler une valeur sur l'itérateur. Elle prend une valeur initiale 
/// pour l'accumulateur (0 dans ce cas) et une fermeture (fonction anonyme) en argument.
/// - |acc, (_, v)|: C'est la liste de paramètres de la fermeture. acc est 
/// l'accumulateur, et (_, v) est un modèle de déstructuration qui représente 
/// chaque paire clé-valeur dans l'itération. Nous ignorons la clé (_) et utilisons 
/// seulement la valeur (v).
/// - { if v == &value {acc + 1} else {acc}}: C'est le corps de la fermeture. 
/// Il vérifie si la valeur actuelle (v) est égale à la valeur cible (&value). 
/// Si c'est le cas, il incrémente l'accumulateur (acc + 1) ; sinon, il laisse 
/// l'accumulateur inchangé (acc).
/// 
/// ===
/// 
///     collection.iter().fold(0, |acc, x| acc + count_iterator(x, value))
/// Cette ligne itère sur la collection de HashMaps, utilisant la fonction 
/// count_iterator pour compter les occurrences de la valeur spécifiée dans chaque 
/// HashMap. Les résultats sont accumulés, et la somme finale est renvoyée comme 
/// résultat.
/// 
/// collection.iter().fold(0, |acc, x| acc + count_iterator(x, value))

/// - collection.iter(): Cela crée un itérateur sur les éléments de la collection (collection), 
/// qui sont des HashMaps dans ce cas.
/// - .fold(0, |acc, x| acc + count_iterator(x, value)): Similaire à la ligne précédente, 
/// elle utilise la méthode fold pour accumuler une valeur sur l'itérateur.
/// |acc, x|: Cette fermeture prend deux paramètres, où acc est l'accumulateur, 
/// et x représente chaque élément dans l'itération (chaque HashMap dans ce cas).
/// - { acc + count_iterator(x, value) }: C'est le corps de la fermeture. 
/// Il incrémente l'accumulateur (acc) du résultat de l'appel de la fonction
/// count_iterator sur l'élément actuel (x).
/// 

// iterators5.rs
//
// Let's define a simple model to track Rustlings exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try not to use imperative loops (for, while).
// Only the two iterator methods (count_iterator and count_collection_iterator)
// need to be modified.
//
// Execute `rustlings hint iterators5` or use the `hint` watch subcommand for a
// hint.


use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // map is a hashmap with String keys and Progress values.
    // map = { "variables1": Complete, "from_str": None, ... }
    //todo!();
    map.iter().fold(0, |acc, (_, v)| { if v == &value {acc + 1} else {acc}})

}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // collection is a slice of hashmaps.
    // collection = [{ "variables1": Complete, "from_str": None, ... },
    //     { "variables2": Complete, ... }, ... ]
    //todo!();
    collection.iter().fold(0, |acc, x| acc + count_iterator(x, value))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(3, count_iterator(&map, Progress::Complete));
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(1, count_iterator(&map, Progress::Some));
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(2, count_iterator(&map, Progress::None));
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator(&map, progress_state)
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            6,
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(1, count_collection_iterator(&collection, Progress::Some));
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(4, count_collection_iterator(&collection, Progress::None));
    }

    #[test]
    fn count_collection_equals_for() {
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        let collection = get_vec_map();

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state)
            );
        }
    }

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }
}
