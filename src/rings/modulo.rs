use crate::error::AbsaglError;
use crate::groups::{GroupElement, CanonicalRepr, CheckedOp};
use crate::groups::modulo::ModuloError;
use crate::rings::{RingElement, CheckedRingOp};
use crate::utils;
use std::fmt;


/// Represents an element in a modulo ring.
/// This struct encapsulates a value and its modulus, providing methods for addition,
/// negation, and multiplication, as well as checked operations that ensure
/// the validity of the operations (e.g., checking for zero divisors).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModuloElement {
    value: u64,
    modulus: u64,
}

impl ModuloElement {
    /// Creates a new `ModuloElement` with the given value and modulus.
    /// If the value is greater than or equal to the modulus, it will be reduced modulo the modulus.
    /// this is a unchecked constructor.
    pub fn new(value: u64, modulus: u64) -> Self {

        Self { value: value % modulus, modulus }
    }
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn modulus(&self) -> u64 {
        self.modulus
    }
    /// Attempts to create a new `ModuloElement` with the given value and modulus.
    /// Returns an error if the modulus is zero.
    pub fn try_new(value: u64, modulus: u64) -> Result<Self, AbsaglError> {
        if modulus == 0 {
            log::error!("Modulus cannot be zero");
            return Err(AbsaglError::from(ModuloError::ZeroModulus));
        }
        let value = value % modulus; // Ensure value is within bounds

        Ok(Self { value, modulus })
    }

    /// Generates a vector of `ModuloElement` instances representing the group of integers modulo `modulus`.
    /// This function creates elements from 0 to `modulus - 1`, each with the specified modulus.
    /// Returns an empty vector if the modulus is zero.
    pub fn generate_modulo_group(modulus: u64) -> Vec<Self> {
        if modulus == 0 {
            log::error!("Cannot generate modulo group with zero modulus");
            return vec![];
        }
        (0..modulus).map(|i| Self::new(i, modulus)).collect()
    }
}

// Implementation for the RING properties
impl RingElement for ModuloElement {
    fn add(&self, other: &Self) -> Self {
        if self.modulus != other.modulus {
            panic!("Cannot add elements with different moduli");
        }
        Self {value:(self.value + other.value) % self.modulus, modulus: self.modulus}
    }

    fn negate(&self) -> Self {
        Self {value:(self.modulus - self.value) % self.modulus, modulus: self.modulus}
    }

    fn mul(&self, other: &Self) -> Self {
        if self.modulus != other.modulus {
            panic!("Cannot multiply elements with different moduli");
        }
        Self {value:(self.value * other.value) % self.modulus, modulus: self.modulus}
    }
}


impl CheckedRingOp for ModuloElement {
    type Error = ModuloError;

    fn checked_add(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.modulus != other.modulus {
            Err(ModuloError::DifferentModuli)
        } else {
            Ok(self.add(other))
        }
    }

    fn checked_mul(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.modulus != other.modulus {
            Err(ModuloError::DifferentModuli)
        } else {
            Ok(self.mul(other))
        }
    }
}


// Implementation for the primary GROUP properties (always additive for rings)
impl GroupElement for ModuloElement {
    fn op(&self, other: &Self) -> Self {
        self.add(other) // op is addition
    }

    fn inverse(&self) -> Self {
        self.negate() // inverse is additive inverse
    }
}

impl CheckedOp for ModuloElement {
    type Error = ModuloError;
    /// Performs a checked operation, ensuring both elements have the same modulus.
    /// for `ModuloElement`, this is addition.
    fn checked_op(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.modulus != other.modulus {
            log::error!("Cannot perform operation on elements with different moduli");
            return Err(ModuloError::DifferentModuli);
        }
        Ok(self.add(other))
    }
}

// Boilerplate implementations
impl CanonicalRepr for ModuloElement {
    fn to_canonical_bytes(&self) -> Vec<u8> {
        [self.value.to_be_bytes(), self.modulus.to_be_bytes()].concat()
    }
}


impl fmt::Display for ModuloElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (mod {})", self.value, self.modulus)
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_modulo_element_creation_unchecked() {
        let elem = ModuloElement::new(5, 12);
        assert_eq!(elem.value(), 5);
        assert_eq!(elem.modulus(), 12);
    }

    #[test]
    fn test_modulo_element_creation_checked() {
        let elem = ModuloElement::try_new(5, 12);
        assert!(elem.is_ok());
        let elem = ModuloElement::try_new(5, 0);
        assert!(elem.is_err());
        match elem {
            Err(AbsaglError::Modulo(ModuloError::ZeroModulus)) => (),
            _ => panic!("Expected ZeroModulus error"),
        }
    }

    #[test]
    fn test_modulo_element_addition() {
        let elem1 = ModuloElement::new(5, 12);
        let elem2 = ModuloElement::new(7, 12);
        let result = elem1.add(&elem2);
        assert_eq!(result.value(), 0); // (5 + 7) % 12 = 0
    }

    #[test]
    fn test_modulo_element_negation() {
        let elem = ModuloElement::new(5, 12);
        let negated = elem.negate();
        assert_eq!(negated.value(), 7); // (12 - 5) % 12 = 7
    }

    #[test]
    fn test_modulo_element_multiplication() {
        let elem1 = ModuloElement::new(5, 12);
        let elem2 = ModuloElement::new(3, 12);
        let result = elem1.mul(&elem2);
        assert_eq!(result.value(), 3); // (5 * 3) % 12 = 3
    }

    #[test]
    fn test_modulo_element_checked_addition() {
        let elem1 = ModuloElement::new(5, 12);
        let elem2 = ModuloElement::new(7, 12);
        let result = elem1.checked_add(&elem2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), 0); // (5 + 7) % 12 = 0

        let elem3 = ModuloElement::new(5, 12);
        let elem4 = ModuloElement::new(7, 13); // Different modulus
        let result = elem3.checked_add(&elem4);
        assert!(result.is_err());
        match result {
            Err(ModuloError::DifferentModuli) => (),
            _ => panic!("Expected DifferentModuli error"),
        }
    }

    #[test]
    fn test_modulo_element_checked_multiplication() {
        let elem1 = ModuloElement::new(5, 12);
        let elem2 = ModuloElement::new(3, 12);
        let result = elem1.checked_mul(&elem2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), 3); // (5 * 3) % 12 = 3

        let elem3 = ModuloElement::new(5, 12);
        let elem4 = ModuloElement::new(7, 13); // Different modulus
        let result = elem3.checked_mul(&elem4);
        assert!(result.is_err());
        match result {
            Err(ModuloError::DifferentModuli) => (),
            _ => panic!("Expected DifferentModuli error"),
        }
    }

    #[test]
    fn test_modulo_element_generate_group() {
        let group = ModuloElement::generate_modulo_group(5);
        assert_eq!(group.len(), 5);
        assert_eq!(group[0].value(), 0);
        assert_eq!(group[1].value(), 1);
        assert_eq!(group[2].value(), 2);
        assert_eq!(group[3].value(), 3);
        assert_eq!(group[4].value(), 4);
    }

    #[test]
    fn test_modulo_element_display() {
        let elem = ModuloElement::new(5, 12);
        assert_eq!(format!("{}", elem), "5 (mod 12)");
    }
}