


use absagl::groups::modulo::Modulo;
use absagl::groups::Group;

#[cfg(test)]
mod tests {

    // Import the necessary modules and traits
    use super::*;

    #[test]
    fn test_is_closed_true() {
        let a = Modulo { value: 0, modulus: 3 };
        let b = Modulo { value: 1, modulus: 3 };
        let c = Modulo { value: 2, modulus: 3 };

        let group = Group::new(vec![a, b, c]);

        assert!(group.is_closed());
    }

    #[test]
    fn test_is_closed_false() {
        let a = Modulo { value: 0, modulus: 3 };
        let b = Modulo { value: 1, modulus: 3 };

        let group = Group::new(vec![a, b]);
        assert!(!group.is_closed());
    }
}