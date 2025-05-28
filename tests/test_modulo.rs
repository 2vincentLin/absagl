
use absagl::groups::modulo::Modulo;
use absagl::groups::GroupElement;


mod tests {
    use super::*;

    #[test]
    fn test_modulo_operations() {
        let a = Modulo { value: 3, modulus: 5 };
        let b = Modulo { value: 4, modulus: 5 };
        let c = a.op(&b);
        assert_eq!(c.value, 2); // (3 + 4) mod 5 = 2
    }

    #[test]
    fn test_identity_element() {
        let identity = Modulo::identity();
        assert_eq!(identity.value, 0); // Identity for addition mod n is 0
    }

    #[test]
    fn test_inverse_element() {
        let a = Modulo { value: 3, modulus: 5 };
        let inverse = a.inverse();
        assert_eq!(inverse.value, 2); // Inverse of 3 mod 5 is 2
    }
}