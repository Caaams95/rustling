/// Write up
/// Ici, nous souhaitons retourner le resultat de la fonction dans une variable.
/// Nous devons donc retourner le resultat dans la fonctions square()
/// Pour cela il ne faut pas mettre de ; a la fin de num*num afin de retourner cette variable

// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
