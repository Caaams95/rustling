/// Write up
/// Ici nous voulons importer 
///     std > time > SystemTime
///     std > time > UNIX_EPOCH
/// de la bibliotheque Rust.
/// Nous devons donc utiliser use afin de faire cette import
/// Voici les use a faire :
///     use std::time::SystemTime;
///     use std::time::UNIX_EPOCH;
/// On peut factoriser ceci comme ci dessous : 
///     use std::time::{SystemTime, UNIX_EPOCH};
/// 
/// Ici on a donc importé SystemTime et UNIX_EPOCH venant de time qui eux-memes viennent de std
/// Avec SystemTime qui est un module => SystemTime::now()....
/// Et UNIX_EPOCH qui est une variable => ...(UNIX_EPOCH)
/// 


// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.


// TODO: Complete this use statement
use std::time::{SystemTime, UNIX_EPOCH};


fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
