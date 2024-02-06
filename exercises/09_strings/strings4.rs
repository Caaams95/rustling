/// Write up 
/// 
/// La principale différence entre &str et String sont :
/// 
/// String :
///     - String est une chaîne de caractères possédée, ce qui signifie qu'elle possède ses propres données et peut être modifiée.
///     - String est mutable on peut modifier le contenu d'une String après sa création.
///     - String se crée avec String::from("...") ou avec la méthode to_string().
/// 
/// &str :
///     - &str est une référence à une séquence de caractères en mémoire, sans possession des données.
///     - &str ne crée pas de nouvelle chaîne, mais fait référence à une partie d'une chaîne existante.
///     - &str est immuable, on ne ne pas modifier la séquence de caractères qu'il référence.
///     - &str est souvent passé en tant que référence ou résultat de certaines opérations, telles que &String ou des tranches (&str[1..3]).
/// 
/// 
/// Explication des choix:
/// 
/// string_slice("blue");
/// "blue" est de type &str, et string_slice prend un &str en tant que paramètre.
///
/// string("red".to_string());
/// La méthode to_string() crée une nouvelle String, et string prend un String en tant que paramètre.
///
/// string(String::from("hi"));
/// La fonction String::from("hi") crée une nouvelle String, et string prend un String en tant que paramètre.
///
/// string("rust is fun!".to_owned());
/// La méthode to_owned() crée une nouvelle String, et string prend un String en tant que paramètre.
///
/// string("nice weather".into());
/// La méthode into() convertit la chaîne littérale "nice weather" en String, et string prend un String en tant que paramètre.
///
/// string(format!("Interpolation {}", "Station"));
/// La fonction format! crée une chaîne formatée qui est ensuite convertie en String par string.
///
/// string_slice(&String::from("abc")[0..1]);
/// La découpe ([0..1]) d'une String renvoie une référence &str, et string_slice prend un &str en tant que paramètre.
///
/// string_slice(" hello there ".trim());
/// La méthode trim() sur une chaîne littérale crée une référence &str, et string_slice prend un &str en tant que paramètre.
///
/// string("Happy Monday!".to_string().replace("Mon", "Tues"));
/// La chaîne est manipulée avec replace(), ce qui crée une nouvelle String, et string prend un String en tant que paramètre.
///
/// string("mY sHiFt KeY iS sTiCkY".to_lowercase());
/// La méthode to_lowercase() crée une nouvelle String, et string prend un String en tant que paramètre.
/// 
/// 
/// 
/// 
// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
