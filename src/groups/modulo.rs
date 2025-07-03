use crate::groups::{GroupElement};
use crate::utils;
use std::f32::consts::E;
use std::fmt;
use std::error::Error;

#[derive(Clone, PartialEq, Debug)]
pub struct Modulo {
    value: u64,
    modulus: u64,
}

#[derive(Debug)]
pub enum ModuloError {
    SizeNotMatch,
    ZeroModulus,
    // Add more as needed
}

impl fmt::Display for ModuloError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuloError::SizeNotMatch => write!(f, "Size mismatch error"),
            ModuloError::ZeroModulus => write!(f, "Zero modulus error"),
            // Handle other errors as needed
        }
    }
}

impl Error for ModuloError {}



impl GroupElement for Modulo {
    type Error = ModuloError;
    fn op(&self, other: &Self) -> Self {
        assert_eq!(self.modulus, other.modulus, "Modulus must match for operation");
        Modulo {
            value: (self.value + other.value) % self.modulus,
            modulus: self.modulus,
        }
    }
    fn identity() -> Self {
        Modulo { value: 0, modulus: 1 } // You may want to pass modulus as parameter
    }
    fn inverse(&self) -> Self {
        Modulo {
            value: (self.modulus - self.value) % self.modulus,
            modulus: self.modulus,
        }
    }
    fn safe_op(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.modulus != other.modulus {
            log::error!("Size mismatch: {} != {}", self.modulus, other.modulus);
            Err(ModuloError::SizeNotMatch)
        } else {
            Ok(self.op(other))
        }
    }
}

impl Modulo {

    /// Create a new Modulo element
    pub fn new(value: u64, modulus: u64) -> Result<Self, ModuloError> {
        if modulus == 0 {
            log::error!("Modulus cannot be zero");
            return Err(ModuloError::ZeroModulus);
        }
        let value = value % modulus; // Ensure value is within bounds
        Ok(Modulo { value, modulus })
    }

    /// identity element for Modulo group
    pub fn identity(modulus: u64) -> Self {
        Modulo { value: 0, modulus }
    }

    /// return order of the element
    pub fn order(&self) -> u64 {
        
        // The order of an element in Z_n is n / gcd(n, value)
        self.modulus / utils::gcd(self.modulus as usize, self.value as usize) as u64
    }

    /// Generate Z_n group elements
    pub fn generate_group(n: u64) -> Result<Vec<Self>, ModuloError> {
        if n == 0 {
            log::error!("Cannot generate group with modulus zero");
            return Err(ModuloError::ZeroModulus);
        }
        Ok((0..n).map(|i| Modulo { value: i, modulus: n }).collect())
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modulo_op() {
        let a = Modulo::new(1, 5).unwrap();
        let b = Modulo::new(3, 5).unwrap();
        let c = a.op(&b);
        assert_eq!(c.value, 4);
    }

    #[test]
    fn test_modulo_identity() {
        let id = Modulo::identity(5);
        assert_eq!(id.value, 0);
    }

    #[test]
    fn test_modulo_inverse() {
        let a = Modulo::new(3, 5).unwrap();
        let inverse = a.inverse();
        assert_eq!(inverse.value, 2);
    }

    #[test]
    fn test_modulo_safe_op_size_mismatch() {
        let a = Modulo::new(1, 5).unwrap();
        let b = Modulo::new(2, 6).unwrap();
        let result = a.safe_op(&b);
        assert!(result.is_err(), "safe_op should return Err on size mismatch");
    }
}