


use absagl::groups::modulo::Modulo;
use absagl::groups::Group;

#[cfg(test)]
mod tests {

    // Import the necessary modules and traits
    use super::*;

    #[test]
    fn test_is_closed_true() {
        let a = Modulo::new(0, 3).expect("Failed to create Modulo element");
        let b = Modulo::new(1, 3).expect("Failed to create Modulo element");
        let c = Modulo::new(2, 3).expect("Failed to create Modulo element");

        let group = Group::new(vec![a, b, c]);

        assert!(group.is_closed());
    }

    #[test]
    fn test_is_closed_false() {
        let a = Modulo::new(0, 3).expect("Failed to create Modulo element");
        let b = Modulo::new(1, 3).expect("Failed to create Modulo element");

        let group = Group::new(vec![a, b]);
        assert!(!group.is_closed());
    }
}