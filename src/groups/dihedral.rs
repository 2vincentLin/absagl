use std::fmt;
use std::error::Error;

use crate::error::AbsaglError;
use crate::groups::{CanonicalRepr, CheckedOp, GroupElement};
use crate::utils;



#[derive(Debug)]
pub enum DihedralError {
    SizeCannotBeZero,
    SizeNotMatch
    // Add more as needed
}   

impl fmt::Display for DihedralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DihedralError::SizeCannotBeZero => write!(f, "Size cannot be zero"),
            DihedralError::SizeNotMatch => write!(f, "Size mismatch error"),
            // Handle other errors as needed
        }
    }
}

impl Error for DihedralError {}






#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
pub struct DihedralElement {
    rotation: usize, // Number of rotations
    reflection: bool, // Whether the element is a reflection
    n: usize, // Number of sides of the polygon
}

impl GroupElement for DihedralElement {

    fn op(&self, other: &Self) -> Self {
        if self.n != other.n {
            panic!("Cannot operate on elements with different n values");
        }
        
        let new_rotation = (self.rotation + other.rotation) % self.n;
        let new_reflection = self.reflection ^ other.reflection; // XOR for reflection

        DihedralElement {
            rotation: new_rotation,
            reflection: new_reflection,
            n: self.n,
        }
    }

   

    fn inverse(&self) -> Self {
        DihedralElement {
            rotation: (self.n - self.rotation) % self.n,
            reflection: self.reflection, // Inverse of reflection is itself
            n: self.n,
        }
    }

    
}

impl CheckedOp for DihedralElement {
    type Error = DihedralError;

    fn checked_op(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.n != other.n {
            log::error!("Size mismatch: {} != {}", self.n, other.n);
            Err(DihedralError::SizeNotMatch)
        } else {
            Ok(self.op(other))
        }
    }
}


impl DihedralElement {

    /// Creates a new DihedralElement with the given rotation and reflection.
    /// This will not check if the size is zero.
    pub fn new(rotation: usize, reflection: bool, n: usize) -> Self {
        DihedralElement {
            rotation,
            reflection,
            n,
        }
    }

    /// Creates a new DihedralElement with the given rotation and reflection, 
    /// this will not check if the size is zero
    pub fn try_new(rotation: usize, reflection: bool, n: usize) -> Result<Self, AbsaglError> {
        if n == 0 {
            log::error!("Size cannot be zero");
        }
        Ok(DihedralElement {
            rotation,
            reflection,
            n,
        })
    }

    pub fn identity(n: usize) -> Self {
        DihedralElement {
            rotation: 0,
            reflection: false,
            n,
        }
    }

    /// Returns the number of sides of the polygon
    pub fn n(&self) -> usize {
        self.n
    }

    /// Returns the rotation of the dihedral element
    pub fn rotation(&self) -> usize {
        self.rotation
    }
    /// Returns whether the dihedral element is a reflection
    pub fn is_reflection(&self) -> bool {
        self.reflection
    }


    /// Returns the order of the dihedral element
    pub fn order(&self) -> usize {
        // Case 1: The element is a reflection.
        if self.reflection {
            return 2;
        }

        // Case 2: The element is a pure rotation.
        if self.rotation == 0 {
            // This is the identity element.
            1
        } else {
            // Use the formula n / gcd(n, j).
            self.n / utils::gcd(self.n, self.rotation)
        }
    }

    /// Generate a whole dihedral group .
    pub fn generate_group(n: usize) -> Result<Vec<Self>, AbsaglError> {
        if n == 0 {
            log::error!("Size cannot be zero");
            return Err(DihedralError::SizeCannotBeZero)?;
        }

        let mut elements = Vec::new();

        // Add rotations
        for i in 0..n {
            elements.push(DihedralElement {
                rotation: i,
                reflection: false,
                n,
            });
        }

        // Add reflections
        for i in 0..n {
            elements.push(DihedralElement {
                rotation: i,
                reflection: true,
                n,
            });
        }

        Ok(elements)
    }

}


// A nice display format.
impl fmt::Display for DihedralElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s_part = if self.reflection { "s" } else { "" };
        let r_part = if self.rotation > 0 {
            format!("r^{}", self.rotation)
        } else {
            "".to_string()
        };

        if !self.reflection && self.rotation == 0 {
            write!(f, "e") // Identity element
        } else {
            write!(f, "{}{}", s_part, r_part)
        }
    }
}

impl CanonicalRepr for DihedralElement {
    fn to_canonical_bytes(&self) -> Vec<u8> {
        // Use a consistent endianness, like big-endian (to_be_bytes),
        // which is a common convention for canonical forms.
        let rotation_bytes = self.rotation.to_be_bytes();
        let reflection_byte = if self.reflection { 1u8 } else { 0u8 };
        let n_bytes = self.n.to_be_bytes();

        // Concatenate the byte arrays into a single, flat Vec<u8>.
        // concat() will create a owned data so we are not return borrowed data
        [
            &rotation_bytes[..],
            &[reflection_byte],
            &n_bytes[..],
        ]
        .concat()
    }
}

#[cfg(test)]
mod test_dihedrals {
    use super::*;

    #[test]
    fn test_dihedral_element_creation() {
        let element = DihedralElement::try_new(1, true, 4).unwrap();
        assert_eq!(element.rotation, 1);
        assert!(element.reflection);
        assert_eq!(element.n, 4);
    }

    #[test]
    fn test_dihedral_element_identity() {
        let identity = DihedralElement::identity(5);
        assert_eq!(identity.rotation, 0);
        assert!(!identity.reflection);
        assert_eq!(identity.n, 5);
    }
    #[test]
    fn test_dihedral_element_order() {
        let element = DihedralElement::try_new(2, false, 4).unwrap();
        assert_eq!(element.order(), 2); // 4 / gcd(4, 2) = 2

        let reflection = DihedralElement::try_new(1, true, 4).unwrap();
        assert_eq!(reflection.order(), 2); // Reflections always have order 2

        let identity = DihedralElement::identity(4);
        assert_eq!(identity.order(), 1); // Identity has order 1
    }

    #[test]
    fn test_dihedral_element_inverse() {
        let element = DihedralElement::try_new(1, false, 4).unwrap();
        let inverse = element.inverse();
        assert_eq!(inverse.rotation, 3); // 4 - 1 = 3
        assert!(!inverse.reflection); // Inverse of a rotation is itself

        let reflection = DihedralElement::try_new(2, true, 4).unwrap();
        let reflection_inverse = reflection.inverse();
        assert_eq!(reflection_inverse.rotation, 2); // Reflection inverse is itself
        assert!(reflection_inverse.reflection);
    }

    #[test]
    fn test_dihedral_element_op() {
        let a = DihedralElement::try_new(1, false, 4).unwrap();
        let b = DihedralElement::try_new(2, true, 4).unwrap();
        let c = a.op(&b);
        assert_eq!(c.rotation, 3);
        assert!(c.reflection);
    }

    #[test]
    fn test_to_canonical_bytes() {
        let d1 = DihedralElement::try_new(1, false,9).unwrap();
        let expected: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 9];
        assert_eq!(d1.to_canonical_bytes(), expected);
    }

    #[test]
    fn test_dihedral_checked_op() {
        let a = DihedralElement::try_new(1, false, 4).unwrap();
        let b = DihedralElement::try_new(2, true, 4).unwrap();
        let result = a.checked_op(&b);
        assert!(result.is_ok());
        let c = result.unwrap();
        assert_eq!(c.rotation, 3);
        assert!(c.reflection);
        
        // Test size mismatch
        let d = DihedralElement::try_new(1, false, 5).unwrap();
        let result = a.checked_op(&d);
        match result {
            Err(DihedralError::SizeNotMatch) => assert!(true),
            _ => panic!("Expected size mismatch error"),
        }
    }
}