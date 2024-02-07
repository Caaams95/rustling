/// Write up 
/// struct Wrapper<T> {
///     value: T,
/// }
/// La première ligne déclare une structure appelée Wrapper.
/// La lettre T entre les chevrons < > indique que la structure est générique et 
/// peut être paramétrée par un type. 
/// Cela signifie que vous pouvez créer une instance de Wrapper en spécifiant
/// n'importe quel type à la place de T.
/// La structure a un seul champ appelé value qui est de type T.
/// 
/// impl<T> Wrapper<T> {
///     pub fn new(value: T) -> Self {
///         Wrapper { value }
///     }
/// }
/// 
/// Les lignes suivantes déclarent une implémentation pour la structure Wrapper 
/// avec le paramètre de type générique T. 
/// Cela signifie qu'on fournit une implémentation spécifique pour 
/// cette structure lorsqu'elle est utilisée avec un type particulier T.
/// 
/// new est une méthode associée qui est utilisée pour créer une nouvelle instance 
/// de la structure Wrapper.
/// Cette méthode prend un argument value de type T et renvoie une nouvelle 
/// instance de Wrapper avec ce value. 
/// La méthode utilise la syntaxe de la structuration pour initialiser la 
/// structure avec le champ value.
/// 
/// pub signifie que la méthode new est publique et peut être utilisée
/// en dehors du module actuel.
/// 
/// self est utilisé pour indiquer le type actuel, c'est-à-dire 
/// le type Wrapper<T> dans ce contexte.
/// 
/// 

// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.


struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
