/// Write up
/// fn some_func(item: impl SomeTrait + OtherTrait) -> bool 
/// indique que la fonction some_func prend un argument item qui 
/// doit être d'un type implémentant à la fois le trait SomeTrait 
/// et le trait OtherTrait. La fonction renvoie un booléen qui est 
/// le résultat de l'opération logique ET (&&) entre les résultats 
/// des fonctions some_function et other_function appelées sur 
/// l'objet item.



// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.




pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}
