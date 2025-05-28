use crate::groups::{GroupElement};

#[derive(Clone, PartialEq, Debug)]
pub struct Modulo {
    pub value: u32,
    pub modulus: u32,
}

#[derive(Debug)]
pub enum ModuloError {
    SizeNotMatch,
    // Add more as needed
}

impl GroupElement for Modulo {
    type Error = ModuloError;
    fn op(&self, other: &Self) -> Self {
        assert_eq!(self.modulus, other.modulus);
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
            Err(ModuloError::SizeNotMatch)
        } else {
            Ok(self.op(other))
        }
    }
}

impl Modulo {
    // Generate Z_n group elements
    pub fn generate_group(n: u32) -> Vec<Modulo> {
        (0..n).map(|i| Modulo { value: i, modulus: n }).collect()
    }
}