use crate::groups::{Additive, CanonicalRepr, CheckedOp, GroupElement, Multiplicative};
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
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Modulo<Op> {
    value: u64,
    modulus: u64,
    _marker: PhantomData<Op>,
}

/// Defines properties associated with a modulo group operation.
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
    /// return order of the element
    fn order(value: u64, modulus: u64) -> u64;
    /// return symbol of the operation, e.g. "+" for additive, "×" for multiplicative
    fn symbol() -> &'static str;
    /// generate the whole group based on the operation
    fn generate_group(modulus: u64) -> Result<Vec<Modulo<Self>>, AbsaglError>;
    
}

impl ModuloOperation for Additive {
    fn identity() -> u64 {
        0
    }
    /// return order of the element
    fn order(value: u64, modulus: u64) -> u64 {
        
        // The order of an element in Z_n is n / gcd(n, value)
        modulus / utils::gcd(modulus as usize, value as usize) as u64
    }
    fn symbol() -> &'static str {
        "+"
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
    fn order(value: u64, modulus: u64) -> u64 {
        let mut k = 1;
        let mut acc = value % modulus;
        while acc != 1 {
            acc = (acc * value) % modulus;
            k += 1;
        }
        k
    }
    fn symbol() -> &'static str {
        "×" // Use the multiplication sign for clarity
    }
    fn generate_group(modulus: u64) -> Result<Vec<Modulo<Self>>, AbsaglError> {
        (1..modulus)
            .filter(|&k| utils::gcd(k as usize, modulus as usize) == 1)
            .map(|k| Modulo::new(k, modulus))
            .collect()
    }
}




impl GroupElement for Modulo<Additive> {
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
    
}



impl GroupElement for Modulo<Multiplicative> {

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

    
}


impl<T> CanonicalRepr for Modulo<T> {
    fn to_canonical_bytes(&self) -> Vec<u8> {
        // Use a consistent endianness, like big-endian (to_be_bytes),
        // which is a common convention for canonical forms.
        let value_bytes = self.value.to_be_bytes();
        let modulus_bytes = self.modulus.to_be_bytes();

        // Concatenate the byte arrays into a single, flat Vec<u8>.
        [value_bytes, modulus_bytes].concat()
    }
}

impl<Op> CheckedOp for Modulo<Op> 
where 
    Op: ModuloOperation,
    Modulo<Op>: GroupElement,
{
    type Error = ModuloError;

    fn checked_op(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.modulus != other.modulus {
            log::error!("Size mismatch: {} != {}", self.modulus, other.modulus);
            Err(ModuloError::SizeNotMatch)
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

    /// return value of the element
    pub fn value(&self) -> u64 {
        self.value
    }

    /// return modulus of the element
    pub fn modulus(&self) -> u64 {
        self.modulus
    }

    /// return order of the element
    pub fn order(&self) -> u64 
    where 
        Op: ModuloOperation,
    {
        Op::order(self.value, self.modulus)
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

impl<Op> fmt::Display for Modulo<Op>
where
    Op: ModuloOperation, Modulo<Op>: GroupElement
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Option 1: Standard Mathematical Notation (Recommended)
        // write!(f, "{} (mod {})", self.value, self.modulus)

        // Option 2: Explicit Notation (Good for debugging)
        write!(
            f,
            "{} (mod {}){}",
            self.value,
            self.modulus,
            Op::symbol()
        )
    }
}


#[cfg(test)]
mod test_modulos {

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
    fn test_modulo_mul_create_fail_not_member() {
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
    fn test_modulo_order_add() {
        let a = Modulo::<Additive>::new(3, 5).unwrap();
        assert_eq!(a.order(), 5);
    }

    #[test]
    fn test_modulo_order_add_identity() {
        let a = Modulo::<Additive>::new(0, 5).unwrap();
        // The order of the identity element should be 1
        assert_eq!(a.order(), 1);
    }

    #[test]
    fn test_modulo_order_mul() {
        let a = Modulo::<Multiplicative>::new(3, 7).unwrap();
        assert_eq!(a.order(), 6);
    }

    #[test]
    fn test_modulo_order_mul_identity() {
        let a = Modulo::<Multiplicative>::new(1, 7).unwrap();
        // The order of the identity element should be 1
        assert_eq!(a.order(), 1);
    }

    #[test]
    fn test_modulo_checked_op_size_mismatch_add() {
        let a = Modulo::<Additive>::new(1, 5).unwrap();
        let b = Modulo::<Additive>::new(2, 6).unwrap();
        let result = a.checked_op(&b);
        match result {
            // you can use Err(AbsaglError::Modulo(_)) too
            Err(ModuloError::SizeNotMatch) => {
                //
            },
            _ => panic!("Expected Err(AbsaglError::Modulo(ModuloError::SizeNotMatch)), but got {:?}", result),
        }
    }

    #[test]
    fn test_go_canonical_bytes() {
        let a = Modulo::<Additive>::new(2, 5).expect("should create permutation");
        println!("canonical form: {:?}", a.to_canonical_bytes());
        let b : Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 5];
        assert_eq!(a.to_canonical_bytes(), b);
    }


    #[test]
    fn test_display_add() {
        let a = Modulo::<Additive>::new(2, 5).expect("should create permutation");
        assert_eq!(format!("{}", a), "2 (mod 5)+");
    }

    #[test]
    fn test_display_mul() {
        let a = Modulo::<Multiplicative>::new(2, 5).expect("should create permutation");
        assert_eq!(format!("{}", a), "2 (mod 5)×");
    }

    
}