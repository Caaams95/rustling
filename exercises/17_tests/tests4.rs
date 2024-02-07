/// Write up
/// 
/// assert_eq!(rect.width, 10);: Utilise la macro assert_eq! pour vérifier
/// que la largeur (width) de l'instance rect est égale à 10. 
/// Si ce n'est pas le cas, le test échouera.
///
///  assert_eq!(rect.height, 20);: De même, vérifie que la hauteur (height)
///  de l'instance rect est égale à 20. Si ce n'est pas le cas, le test échouera.
///
/// L'attribut #[should_panic] est utilisé pour indiquer que les tests annotés 
/// doivent être considérés comme réussis s'ils provoquent une panique.
/// Dans ce code, deux tests portent cet attribut pour vérifier le comportement 
/// du programme lorsque des dimensions négatives sont utilisées pour créer 
/// une instance de la structure Rectangle. 

// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.


struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    #[test]
    #[should_panic]

    fn negative_width() {
        // This test should check if program panics when we try to create rectangle with negative width
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic]

    fn negative_height() {
        // This test should check if program panics when we try to create rectangle with negative height
        let _rect = Rectangle::new(10, -10);
    }
}
