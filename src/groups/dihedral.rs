use std::fmt;
use std::error::Error;

use crate::error::AbsaglError;
use crate::groups::GroupElement;
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






#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub struct DihedralElement {
    rotation: usize, // Number of rotations
    reflection: bool, // Whether the element is a reflection
    n: usize, // Number of sides of the polygon
}

impl GroupElement for DihedralElement {
    type Error = AbsaglError;

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

    fn safe_op(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.n != other.n {
            log::error!("Size mismatch: {} != {}", self.n, other.n);
            return Err(DihedralError::SizeNotMatch)?;
        }
        Ok(self.op(other))
    }
}


impl DihedralElement {
    /// Creates a new DihedralElement with the given rotation and reflection
    pub fn new(rotation: usize, reflection: bool, n: usize) -> Result<Self, AbsaglError> {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dihedral_element_creation() {
        let element = DihedralElement::new(1, true, 4).unwrap();
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
        let element = DihedralElement::new(2, false, 4).unwrap();
        assert_eq!(element.order(), 2); // 4 / gcd(4, 2) = 2

        let reflection = DihedralElement::new(1, true, 4).unwrap();
        assert_eq!(reflection.order(), 2); // Reflections always have order 2

        let identity = DihedralElement::identity(4);
        assert_eq!(identity.order(), 1); // Identity has order 1
    }

    #[test]
    fn test_dihedral_element_inverse() {
        let element = DihedralElement::new(1, false, 4).unwrap();
        let inverse = element.inverse();
        assert_eq!(inverse.rotation, 3); // 4 - 1 = 3
        assert!(!inverse.reflection); // Inverse of a rotation is itself

        let reflection = DihedralElement::new(2, true, 4).unwrap();
        let reflection_inverse = reflection.inverse();
        assert_eq!(reflection_inverse.rotation, 2); // Reflection inverse is itself
        assert!(reflection_inverse.reflection);
    }

    #[test]
    fn test_dihedral_element_op() {
        let a = DihedralElement::new(1, false, 4).unwrap();
        let b = DihedralElement::new(2, true, 4).unwrap();
        let c = a.op(&b);
        assert_eq!(c.rotation, 3);
        assert!(c.reflection);
    }
}