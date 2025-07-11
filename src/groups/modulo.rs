use crate::groups::{Additive, GroupElement, Multiplicative};
use crate::utils;
use crate::error::AbsaglError;


use std::fmt;
use std::error::Error;
use std::marker::PhantomData;



#[derive(Debug)]
pub enum ModuloError {
    SizeNotMatch,
    ZeroModulus,
    ElementNotInGroup { value: u64, modulus: u64 }, // for Modulo multiplicative group, gcd(x,n)=1
    // Add more as needed
}

impl fmt::Display for ModuloError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuloError::SizeNotMatch => write!(f, "Size mismatch error"),
            ModuloError::ZeroModulus => write!(f, "Zero modulus error"),
            ModuloError::ElementNotInGroup { value: v, modulus: n } => write!(f, "{} is not in Modulus({}) when op is mul", v, n),
            // Handle other errors as needed
        }
    }
}

impl Error for ModuloError {}


/// Modulo struct for add/mul, Op can be Additive, Multiplicative,
/// call it with Modulo::<Additive>::method()
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Modulo<Op> {
    value: u64,
    modulus: u64,
    _marker: PhantomData<Op>,
}

/// Defines properties associated with a group operation.
pub trait ModuloOperation : Sized where Modulo<Self>: GroupElement {
    /// The identity element for the operation (e.g., 0 for addition, 1 for multiplication).
    fn identity() -> u64;
    /// Check if a element is a valid member of the modulus group, 
    /// this is used for multiplicative modulus group, where gcd(x,n)=1,
    /// for additive, it's just true.
    fn is_valid(value: u64, modulus: u64) -> bool {
        let _ = value;
        let _ = modulus;
        true
    }
    // generate the whole group based on the operation
    fn generate_group(modulus: u64) -> Result<Vec<Modulo<Self>>, AbsaglError>;
    
}

impl ModuloOperation for Additive {
    fn identity() -> u64 {
        0
    }
    fn generate_group(modulus: u64) -> Result<Vec<Modulo<Self>>, AbsaglError> {
        (0..modulus).map(|i| Modulo::new(i, modulus)).collect()
    }
}

impl ModuloOperation for Multiplicative {
    fn identity() -> u64 {
        1
    }
    /// Override: A value is valid for multiplication if gcd(value, modulus) == 1.
    fn is_valid(value: u64, modulus: u64) -> bool {
        utils::gcd(value as usize, modulus as usize) == 1
    }
    fn generate_group(modulus: u64) -> Result<Vec<Modulo<Self>>, AbsaglError> {
        (1..modulus)
            .filter(|&k| utils::gcd(k as usize, modulus as usize) == 1)
            .map(|k| Modulo::new(k, modulus))
            .collect()
    }
}




impl GroupElement for Modulo<Additive> {
    type Error = AbsaglError;
    fn op(&self, other: &Self) -> Self {
        assert_eq!(self.modulus, other.modulus, "Modulus must match for operation");
        Modulo {
            value: (self.value + other.value) % self.modulus,
            modulus: self.modulus,
            _marker: PhantomData,
        }
    }
    
    fn inverse(&self) -> Self {
        Modulo {
            value: (self.modulus - self.value) % self.modulus,
            modulus: self.modulus,
            _marker: PhantomData,
        }
    }
    fn safe_op(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.modulus != other.modulus {
            log::error!("Size mismatch: {} != {}", self.modulus, other.modulus);
            Err(ModuloError::SizeNotMatch)?
        } else {
            Ok(self.op(other))
        }
    }
}



impl GroupElement for Modulo<Multiplicative> {
    type Error = AbsaglError;

    fn op(&self, other: &Self) -> Self {
        assert_eq!(self.modulus, other.modulus, "Modulus must match");
        Modulo {
            value: (self.value * other.value) % self.modulus,
            modulus: self.modulus,
            _marker: PhantomData,
        }
    }

    fn inverse(&self) -> Self {
        let inverse_value = utils::modular_inverse(self.value as i64, self.modulus as i64)
            .expect("Inverse does not exist");
            
        Modulo {
            value: inverse_value as u64,
            modulus: self.modulus,
            _marker: PhantomData,
        }
    }

    fn safe_op(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.modulus != other.modulus {
            log::error!("Size mismatch: {} != {}", self.modulus, other.modulus);
            Err(ModuloError::SizeNotMatch)?
        } else {
            Ok(self.op(other))
        }
    }
}


impl<Op> Modulo<Op> where Modulo<Op>: GroupElement {

    /// Create a new Modulo element
    pub fn new(value: u64, modulus: u64) -> Result<Self, AbsaglError> 
    where 
        Op: ModuloOperation,
    {
        if modulus == 0 {
            log::error!("Modulus cannot be zero");
            return Err(ModuloError::ZeroModulus)?;
        }
        let value = value % modulus; // Ensure value is within bounds

        if !Op::is_valid(value, modulus) {
            log::error!("{} is not a valid element in this group mod {}", value, modulus);
            return Err(ModuloError::ElementNotInGroup { value, modulus })?;
        }

        Ok(Modulo { value, modulus, _marker: PhantomData })
    }

    /// identity element for Modulo group
    pub fn identity(modulus: u64) -> Self 
    where
        Op: ModuloOperation,
    {
        Modulo { value: Op::identity(), modulus, _marker: PhantomData }
    }

    /// return order of the element
    pub fn order(&self) -> u64 {
        
        // The order of an element in Z_n is n / gcd(n, value)
        self.modulus / utils::gcd(self.modulus as usize, self.value as usize) as u64
    }

    /// Generate Z_n group elements
    pub fn generate_group(n: u64) -> Result<Vec<Self>, AbsaglError> 
    where 
        Op: ModuloOperation
    {
        Op::generate_group(n)
    }
    // pub fn generate_group(n: u64) -> Result<Vec<Self>, AbsaglError> {
    //     if n == 0 {
    //         log::error!("Cannot generate group with modulus zero");
    //         return Err(ModuloError::ZeroModulus)?;
    //     }
    //     Ok((0..n).map(|i| Modulo { value: i, modulus: n, _marker: PhantomData}).collect())
    // }
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_modulo_add_create_fail_zero_modulus() {
        let result = Modulo::<Additive>::new(1, 0);
        match result {
            // you can use Err(AbsaglError::Modulo(_)) too
            Err(AbsaglError::Modulo(ModuloError::ZeroModulus)) => {
                //
            },
            _ => panic!("Expected Err(AbsaglError::Modulo(ModuloError::ZeroModulus)), but got {:?}", result),
        }
    }

    #[test]
    fn test_modulo_add_create_fail_not_member() {
        let result = Modulo::<Multiplicative>::new(2, 4);
        match result {
            // you can use Err(AbsaglError::Modulo(_)) too
            Err(AbsaglError::Modulo(ModuloError::ElementNotInGroup { value: 2, modulus: 4 })) => {
                //
            },
            _ => panic!("Expected Err(AbsaglError::Modulo(ModuloError::ElementNotInGroup)), but got {:?}", result),
        }
    }

    

    #[test]
    fn test_modulo_op_add() {
        let a = Modulo::<Additive>::new(1, 5).unwrap();
        let b = Modulo::<Additive>::new(3, 5).unwrap();
        let c = a.op(&b);
        assert_eq!(c.value, 4);
    }

    #[test]
    fn test_modulo_op_mul() {
        let a = Modulo::<Multiplicative>::new(2, 5).unwrap();
        let b = Modulo::<Multiplicative>::new(3, 5).unwrap();
        let c = a.op(&b);
        assert_eq!(c.value, 1);
    }

    #[test]
    fn test_modulo_identity_add() {
        let id = Modulo::<Additive>::identity(5);
        assert_eq!(id.value, 0);
    }

     #[test]
    fn test_modulo_identity_mul() {
        let id = Modulo::<Multiplicative>::identity(5);
        assert_eq!(id.value, 1);
    }

    #[test]
    fn test_modulo_inverse_add() {
        let a = Modulo::<Additive>::new(3, 5).unwrap();
        let inverse = a.inverse();
        assert_eq!(inverse.value, 2);
    }

    #[test]
    fn test_modulo_inverse_mul() {
        let a = Modulo::<Multiplicative>::new(17, 46).unwrap();
        let inverse = a.inverse();
        assert_eq!(inverse.value, 19);
    }

    #[test]
    fn test_modulo_safe_op_size_mismatch_add() {
        let a = Modulo::<Additive>::new(1, 5).unwrap();
        let b = Modulo::<Additive>::new(2, 6).unwrap();
        let result = a.safe_op(&b);
        match result {
            // you can use Err(AbsaglError::Modulo(_)) too
            Err(AbsaglError::Modulo(ModuloError::SizeNotMatch)) => {
                //
            },
            _ => panic!("Expected Err(AbsaglError::Modulo(ModuloError::SizeNotMatch)), but got {:?}", result),
        }
    }
}