/// Write up
/// 
/// Le code commence par une création d'une structure appelée Point avec deux champs (x et y),
/// chacun de type entier 32 bits (i32).
/// 
/// =========
///     let y: Option<Point> = Some(Point { x: 100, y: 200 });
/// Ici, une variable y est déclarée et initialisée avec Some(Point { x: 100, y: 200 }). 
/// Option est un type enumératif qui peut soit être Some contenant une valeur, soit None indiquant l'absence de valeur.
/// Dans ce cas, y contient un Point avec des coordonnées (x=100, y=200).
/// 
/// =========
///     match y {
///         Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
///         _=> panic!("no match!"),
///     }
/// La déclaration match est utilisée pour gérer les différentes possibilités de valeurs de y.
/// Si y est Some, le bloc de code entre après le => est exécuté. 
/// Ici, ref p est une référence à la valeur de type Point contenue dans Some. 
/// Le bloc imprime les coordonnées x et y du point.
///
/// Si y est None, le bloc de code après le => du _ est exécuté, ce qui déclenche une panique avec le message "no match!".
///
/// Le mot-clé ref est utilisé pour créer une référence à la valeur pointée plutôt que de déplacer la valeur, 
/// ce qui est nécessaire car p est utilisé dans l'affichage sans déplacer la propriété.
/// 

/// // options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
