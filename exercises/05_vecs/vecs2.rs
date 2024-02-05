/// Write up:
/// Dans vec_loop nous voulons modifier la valeur d'element en la multipliant par 2.
/// Afin de modifier directement la valeur d'element, nous devons utiliser * .
/// * permet d'acceder directment et de modifier le contenu réel d'element.
/// Faire une boucle for permet de parcourir le contenu de v
/// Le ; a la fin de *element *= 2; signifie que l'on souhaite pas returner 
/// mais bien modifier le contenu d'element
/// 
/// Dans vec_map nous souhaitons juste retourner la valeur x 2
/// afin d'accelerer le code et de faciliter la lecture, nous faisons donc un iter().map...
/// celui ci nous permet de parcourir le contenu de v et de stocker chaque contenu 1 par 1
/// dans la variable entre ||, ici element
/// nous souhaitons juste retourner la valeur donc on ne doit pas mettre de ;
/// element stockant bien chaque element de v, nous pouvons le multiplié par 2 sans ;
/// afin de le retourner
/// .collect() permet de faire les traitements qu'on a mis dans le map()

 
// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.


fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element * 2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
