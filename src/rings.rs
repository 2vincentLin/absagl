pub mod modulo;


use std::fmt;
use std::fmt::Debug;
use std::error::Error;
use std::hash::Hash;
use crate::groups::{Additive, GroupElement, Multiplicative};

use crate::error::AbsaglError;
use crate::groups::{FiniteGroup, Group, GroupError};
use crate::groups::modulo::ModuloError;
use crate::rings::modulo::ModuloElement;
use std::collections::HashSet;


/// Defines errors specific to ring validation.
#[derive(Debug)]
pub enum RingError {
    /// The underlying additive group is not abelian.
    AdditiveGroupNotAbelian,
    /// The set is not closed under multiplication.
    MultiplicationNotClosed,
    /// Multiplication is not associative.
    MultiplicationNotAssociative,
    /// The distributive property does not hold.
    DistributivityFailed,
    /// An error occurred in the underlying group structure.
    GroupError(GroupError),
    /// An error propagated from an element's operation.
    ElementError(Box<dyn Error + Send + Sync + 'static>),
}

impl fmt::Display for RingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RingError::AdditiveGroupNotAbelian => write!(f, "The set under addition is not an abelian group"),
            RingError::MultiplicationNotClosed => write!(f, "Multiplication is not closed"),
            RingError::MultiplicationNotAssociative => write!(f, "Multiplication is not associative"),
            RingError::DistributivityFailed => write!(f, "The distributive property does not hold"),
            RingError::GroupError(e) => write!(f, "Group error: {}", e),
            RingError::ElementError(e) => write!(f, "Ring element operation error: {}", e),
        }
    }
}

impl Error for RingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RingError::GroupError(e) => Some(e),
            RingError::ElementError(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

/// A trait representing an element of a ring.
/// It must support both an additive and a multiplicative structure.
pub trait RingElement: GroupElement + Clone + Debug + Eq + Hash {
    /// The additive operation (+).
    fn add(&self, other: &Self) -> Self;

    /// The additive inverse (-a).
    fn negate(&self) -> Self;

    /// The multiplicative operation (·).
    fn mul(&self, other: &Self) -> Self;

}

/// An abstract trait representing a Ring.
pub trait Ring<T: RingElement> {
    /// Returns the additive identity (zero) of the ring.
    /// This is the element that, when added to any element, returns that element.
    fn zero(&self) -> &T;
    /// Returns the multiplicative identity (one) of the ring, if it exists.
    /// This is the element that, when multiplied by any element, returns that element.
    fn one(&self) -> Option<&T>;
    /// Checks if the ring is commutative under multiplication.
    /// This means that for any two elements a and b, a * b == b *
    fn is_commutative(&self) -> bool;
}

/// A trait for checked operations on ring elements.
/// This trait extends the `RingElement` trait to include checked operations that ensure
/// the validity of the operation (e.g., checking for zero divisors).
pub trait CheckedRingOp: RingElement {
    type Error: std::error::Error + Send + Sync + 'static;

    fn checked_add(&self, other: &Self) -> Result<Self, Self::Error>;
    fn checked_mul(&self, other: &Self) -> Result<Self, Self::Error>;
}



/// Represents a finite ring.
#[derive(Debug, Clone)]
pub struct FiniteRing<T: RingElement> {
    elements: Vec<T>,
    // additive_group: FiniteGroup<T>,
    zero: T,
    one: Option<T>,
}

impl<T: RingElement> Ring<T> for FiniteRing<T> {
    fn zero(&self) -> &T {
        &self.zero
    }

    fn one(&self) -> Option<&T> {
        self.one.as_ref()
    }

    fn is_commutative(&self) -> bool {
        for a in &self.elements {
            for b in &self.elements {
                if a.mul(b) != b.mul(a) {
                    return false;
                }
            }
        }
        true
    }
}

impl<T: RingElement> FiniteRing<T> {

    /// Creates a new finite ring with the given elements, zero, and one.
    /// The `one` parameter is optional, as some rings may not have a multiplicative identity.
    /// this is unchecked constructor.
    pub fn new(elements: Vec<T>, zero: T, one: Option<T>) -> Self {
        FiniteRing { elements, zero, one }
    }


    /// Creates a new ring from a set of elements, automatically discovering identities.
    /// This constructor verifies all ring axioms, but can be slow due to O(n^2) identity searches.
    pub fn try_new(elements: Vec<T>) -> Result<Self, AbsaglError> {
        let additive_group = FiniteGroup::try_new(elements.clone())?;
        
        // Find the additive identity (zero) by searching.
        let zero = additive_group.identity();

        // Find the multiplicative identity (one) by searching.
        let one = elements.iter().find(|o| {
            elements.iter().all(|a| a.mul(o) == *a && (*o).mul(a) == *a)
        }).cloned();

        // Now, call the efficient verifying constructor with the discovered identities.
        Self::try_new_with_identities(elements, zero, one)
    }

    /// Creates a new ring from a set of elements and identity candidates.
    /// This is the most efficient **checked** constructor. It verifies that the provided
    /// `zero` and `one` are correct identities and that all ring axioms hold.
    pub fn try_new_with_identities(elements: Vec<T>, zero: T, one: Option<T>) -> Result<Self, AbsaglError> {
        // 1. Verify the additive group properties.
        let additive_group = FiniteGroup::try_new(elements.clone())?;
        if !additive_group.is_abelian() {
            log::error!("Additive group is not abelian");
            return Err(AbsaglError::Ring(RingError::AdditiveGroupNotAbelian));
        }

        // 2. Verify the provided additive identity (zero).
        // Check that `zero` is the identity and is present in the elements.
        if !elements.iter().all(|a| zero.add(a) == *a) || !elements.contains(&zero) {
             log::error!("Provided zero element is not the additive identity.");
             // You could add a new RingError variant for this.
             return Err(AbsaglError::Ring(RingError::GroupError(GroupError::NotFound)));
        }

        // 3. Verify the provided multiplicative identity (one), if it exists.
        if let Some(ref one_candidate) = one {
            if !elements.iter().all(|a| a.mul(one_candidate) == *a) || !elements.contains(one_candidate) {
                log::error!("Provided one element is not the multiplicative identity.");
                // You could add a new RingError variant for this.
                return Err(AbsaglError::Ring(RingError::GroupError(GroupError::NotFound)));
            }
        }

        let element_set: HashSet<_> = elements.iter().collect();

        // 4. Verify multiplicative closure and associativity.
        for a in &elements {
            for b in &elements {
                let product = a.mul(b);
                if !element_set.contains(&product) {
                    log::error!("Multiplication not closed for elements: {:?} and {:?}", a, b);
                    return Err(AbsaglError::Ring(RingError::MultiplicationNotClosed));
                }

                for c in &elements {
                    let lhs = a.mul(&b.mul(c));
                    let rhs = (a.mul(b)).mul(c);
                    if lhs != rhs {
                        log::error!("Multiplication not associative for elements: {:?}, {:?}, {:?}", a, b, c);
                        return Err(AbsaglError::Ring(RingError::MultiplicationNotAssociative));
                    }
                }
            }
        }

        // 5. Verify distributivity.
        for a in &elements {
            for b in &elements {
                for c in &elements {
                    let lhs = a.mul(&b.add(c));
                    let rhs = a.mul(b).add(&a.mul(c));
                    if lhs != rhs {
                        log::error!("Distributivity failed for elements: {:?}, {:?}, {:?}", a, b, c);
                        return Err(AbsaglError::Ring(RingError::DistributivityFailed));
                    }
                }
            }
        }

        Ok(FiniteRing { elements, zero, one })
    }

    pub fn order(&self) -> usize {
        self.elements.len()
    }
}


/// A collection of ring generators.
pub struct RingGenerators;

impl RingGenerators {
    /// Generates the finite ring of integers modulo n, Z_n.
    pub fn zn(n: u64) -> Result<FiniteRing<modulo::ModuloElement>, AbsaglError> {
        if n == 0 {
            // Or return a specific error
            log::error!("Cannot create a ring with zero modulus.");
            return Err(AbsaglError::from(ModuloError::ZeroModulus));
        }
        let elements = ModuloElement::generate_modulo_group(n);
        let zero = ModuloElement::new(0, n);
        let one = if n > 1 { Some(ModuloElement::new(1, n)) } else { None };

        // Use the efficient, checked constructor
        FiniteRing::try_new_with_identities(elements, zero, one)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::rings::modulo::ModuloElement;

    #[test]
    fn test_ring_creation() {
        let elements = ModuloElement::generate_modulo_group( 5);
        let ring = FiniteRing::try_new(elements).unwrap();
        assert_eq!(ring.zero().value(), 0);
        assert!(ring.one().is_some());
        assert_eq!(ring.one().unwrap().value(), 1);
    }

    #[test]
    fn test_ring_creation_with_invalid_zero() {
        let elements = ModuloElement::generate_modulo_group(5);
        let ring = FiniteRing::try_new_with_identities(elements, ModuloElement::try_new(1, 5).unwrap(), None);
        assert!(ring.is_err());
        match ring {
            Err(AbsaglError::Ring(RingError::GroupError(GroupError::NotFound))) => (),
            _ => panic!("Expected NotFound error for invalid zero"),
        }
    }

    #[test]
    fn test_ring_creation_with_invalid_one() {
        let elements = ModuloElement::generate_modulo_group(5);
        let ring = FiniteRing::try_new_with_identities(elements, ModuloElement::try_new(0, 5).unwrap(), Some(ModuloElement::try_new(2, 5).unwrap()));
        assert!(ring.is_err());
        match ring {
            Err(AbsaglError::Ring(RingError::GroupError(GroupError::NotFound))) => (),
            _ => panic!("Expected NotFound error for invalid one"),
        }
    }

    #[test]
    fn test_ring_creation_with_generator() {
        let ring = RingGenerators::zn(5).unwrap();
        assert_eq!(ring.order(), 5);
        assert_eq!(ring.zero().value(), 0);
        assert!(ring.one().is_some());
        assert_eq!(ring.one().unwrap().value(), 1);
        assert!(ring.is_commutative());
    }

    // --- Tests for Axiom Failures ---

        // A dummy element that is intentionally non-associative under multiplication.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        struct NonAssociativeElement(u8);

        impl RingElement for NonAssociativeElement {
            fn add(&self, other: &Self) -> Self { NonAssociativeElement((self.0 + other.0) % 4) }
            fn negate(&self) -> Self { NonAssociativeElement((4 - self.0) % 4) }
            fn mul(&self, other: &Self) -> Self {
                // Special case to break associativity: (2*2)*1 != 2*(2*1)
                // if self.0 == 2 && other.0 == 2 { return NonAssociativeElement(2); } // (2*2) = 2
                if self.0 == 2 && other.0 == 1 { return NonAssociativeElement(1); } // (2*1) = 1
                // Default multiplication
                NonAssociativeElement((self.0 * other.0) % 4)
            }
        }
        impl GroupElement for NonAssociativeElement {
            fn op(&self, other: &Self) -> Self { self.add(other) }
            fn inverse(&self) -> Self { self.negate() }
        }

    #[test]
    fn test_fails_on_non_associative_multiplication() {
        let elements = vec![
            NonAssociativeElement(0),
            NonAssociativeElement(1),
            NonAssociativeElement(2),
            NonAssociativeElement(3),
        ];
        let result = FiniteRing::try_new(elements);
        assert!(result.is_err());

        match result {
            Err(AbsaglError::Ring(RingError::MultiplicationNotAssociative)) => (), // Success!
            _ => panic!("Expected MultiplicationNotAssociative error, got {:?}", result),
        }
    }

        // A dummy element that is intentionally non-distributive.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        struct NonDistributiveElement(u8);

        impl RingElement for NonDistributiveElement {
            fn add(&self, other: &Self) -> Self { NonDistributiveElement((self.0 + other.0) % 4) }
            fn negate(&self) -> Self { NonDistributiveElement((4 - self.0) % 4) }
            // Multiplication is defined as `a * b = a`. This is associative.
            fn mul(&self, _other: &Self) -> Self {
                *self
            }
        }
        impl GroupElement for NonDistributiveElement {
            fn op(&self, other: &Self) -> Self { self.add(other) }
            fn inverse(&self) -> Self { self.negate() }
        }

    #[test]
    fn test_fails_on_non_distributive_property() {
        let elements = vec![
            NonDistributiveElement(0),
            NonDistributiveElement(1),
            NonDistributiveElement(2),
            NonDistributiveElement(3),
        ];
        let result = FiniteRing::try_new(elements);
        assert!(result.is_err());
        // Check: 1*(1+1) = 1*2 = 3.
        // 1*1 + 1*1 = 1 + 1 = 2.
        // 3 != 2, so it fails.
        match result {
            Err(AbsaglError::Ring(RingError::DistributivityFailed)) => (), // Success!
            _ => panic!("Expected DistributivityFailed error, got {:?}", result),
        }
    }
}