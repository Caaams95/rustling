/// Write up
///
/// La structure Book a deux champs :
/// author et title, chacun étant une référence (&str) à une chaîne de caractères.
/// 'a est une annotation de durée de vie (lifetimes) indiquant
/// que les références contenues dans Book doivent vivre au moins
/// aussi longtemps que la référence de durée de vie 'a.
/// 
/// 
/// Dans la fonction main :
/// Deux variables sont créées, name et title, qui sont des String (chaînes de caractères dynamiques en Rust).
/// Une instance de la structure Book est créée avec les références aux chaînes de caractères 
/// name et title en tant qu'auteur et titre.
/// La dernière ligne imprime les détails du livre en utilisant les champs de la structure Book.
///
/// L'utilisation de références (&str) au lieu de String pour les champs author et title permet 
/// à la structure Book de partager les données existantes plutôt que de posséder sa propre copie.
/// Cela peut être plus efficace en termes de mémoire et de performance.
///
/// La notation 'a dans la structure indique que la durée de vie des références dans Book est liée à celle des variables name et title. Cela garantit que les références restent valides tant que les chaînes de caractères d'origine (name et title) sont toujours valides.
/// 
/// 

// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
