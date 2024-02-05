/// Write up:
/// Ici on souhaite calculer les frais de port d'un envoi de coli.
/// Dans notre cas, le package (coli) est composé de 3 criteres : pays d'expedition/ reception et le poids,
/// On fait donc une structure qui ferra office d'objet contenant ses 3 variables.
/// Notre structure sera definit comme ceci:
///     - sender_country: String, ==> sender_country sera donc une chaine de caractère representant le pays d'expedition
///     - recipient_country: String, ==> recipient_country sera donc une chaine de caractère representant le pays de reception
///     - weight_in_grams: u32, ==> ici u32 représente un entier non signé de 32 bits qui sera le poids du package

/// fn is_international(&self) -> bool {
///     self.sender_country != self.recipient_country
/// }
/// 
/// Pour commencer, -> bool signifie que la fonction retournera un booléen car ici on veux savoir si Oui ou Non
/// l'expedition se fera a l'etranger
/// Ici, &self indique que la méthode is_international prend une référence (&) à self,
/// qui est une convention en Rust pour représenter l'instance sur laquelle la méthode est appelée,
/// la fonction etant appeler comme ceci par exemple:
///     package.is_international()
/// cela signifie donc que self correspond a "package"
/// 
/// On compare donc si le sender_country et le recipient_country du package sont differents
/// et on retournera donc sous forme de booléen (True ou False) si les pays sont différents 
/// PS : pas de ; pour retourner une variable
/// 
/// 
/// 
/// Suite à ça nous avons la fonction:
/// 
/// fn get_fees(&self, cents_per_gram: u32) -> u32 {
///     self.weight_in_grams * cents_per_gram
/// }
/// 
/// celle ci retournera un nombre u32 car ici on souhaite savoir le prix des frais
/// on a donc comme parametre encore une fois self ce qui nous permettra de recuperer le poids du package 
/// et cents_per_gram: u32 qui est un nombre.
/// Multiplier ces 2 nombres nous donnera donc le prix des frais s'il doit en avoir ( depend de is_international())
/// 
/// 

// structs3.rs
//
// Structs contain data, but can also have logic. In this exercise we have
// defined the Package struct and we want to test some logic attached to it.
// Make the code compile and the tests pass!
//
// Execute `rustlings hint structs3` or use the `hint` watch subcommand for a
// hint. 


#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Package {
        if weight_in_grams < 10 {
            // This is not how you should handle errors in Rust,
            // but we will learn about error handling later.
            panic!("Can not ship a package with weight below 10 grams.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    fn is_international(&self) -> bool {
        // Something goes here...
        self.sender_country != self.recipient_country
    }

    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        // Something goes here...
        self.weight_in_grams * cents_per_gram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
