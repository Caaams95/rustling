/// Write up:
/// vec_loop:
/// Dans vec_loop nous voulons modifier la valeur de chaque element 
/// du vecteur v en la multipliant par 2.
/// Afin de parcourir de chaque element,
/// nous allons utiliser l'itérateur iter_mut() qui permet de parcourir 
/// la collection tout en autorisant la modification des éléments pendant l'itération.
/// 
/// Pour modifier le contenu de "element" qui lui devient chaque contenu du vecteur v,
/// nous devons utiliser * .
/// L'opérateur * est utilisé pour déréférencer un pointeur. 
/// Dans ce contexte, "element" est un pointeur vers l'élément actuel dans la collection. 
/// *element donne accès à la valeur pointée par cet élément.
/// 
/// Le ; à la fin de *element *= 2; signifie que l'on souhaite pas retourner 
/// mais bien modifier le contenu d'element.
/// *element *= 2; <==> *element = *element * 2;
/// 
/// vec_map:
/// Dans vec_map nous souhaitons juste retourner la valeur multiplier par 2.
/// Afin de parcourir en lecture v, nous faisons donc un iter().
/// La méthode iter() est appelée sur la référence du vecteur v. 
/// Elle renvoie un itérateur sur les éléments du vecteur, permettant de parcourir chaque élément de manière immuable.
/// Chaque element sera stocké dans la variable entre ||, ici "element".
/// .collect() est utilisée pour collecter les résultats de la transformation
///  effectuée par map et les stocker dans un nouveau vecteur. => "ans" dans notre cas.
///     let ans = vec_map(&v);
/// ans stockera dont les meme donnée et v mais tout multiplié par 2

 
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
