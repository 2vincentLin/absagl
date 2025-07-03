use crate::groups::{GroupElement};

#[derive(Clone, PartialEq, Debug)]
pub struct Modulo {
    pub value: u64,
    pub modulus: u64,
}

#[derive(Debug)]
pub enum ModuloError {
    SizeNotMatch,
    // Add more as needed
}

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
    pub fn new(value: u64, modulus: u64) -> Self {
        let value = value % modulus; // Ensure value is within bounds
        Modulo { value, modulus }
    }

    /// identity element for Modulo group
    pub fn identity(modulus: u64) -> Self {
        Modulo { value: 0, modulus }
    }

    /// Generate Z_n group elements
    pub fn generate_group(n: u64) -> Vec<Modulo> {
        (0..n).map(|i| Modulo { value: i, modulus: n }).collect()
    }
}