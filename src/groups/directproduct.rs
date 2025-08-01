use crate::groups::GroupElement;
use crate::groups::modulo::{Modulo, ModuloError};
use crate::groups::CheckedOp;
use crate::groups::Additive;
use std::fmt;
use std::error::Error;

/// Represents an element in a direct product of cyclic groups.
/// Each component is an element of one of the factor groups.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DirectProductElement {
    pub components: Vec<Modulo<Additive>>,
}

impl GroupElement for DirectProductElement {
    /// The group operation is performed component-wise.
    fn op(&self, other: &Self) -> Self {
        assert_eq!(self.components.len(), other.components.len(), "Direct product elements must have the same number of components");
        let new_components = self.components.iter()
            .zip(other.components.iter())
            .map(|(c1, c2)| c1.op(c2))
            .collect();
        DirectProductElement { components: new_components }
    }

    /// The inverse is also found component-wise.
    fn inverse(&self) -> Self {
        let new_components = self.components.iter()
            .map(|c| c.inverse())
            .collect();
        DirectProductElement { components: new_components }
    }
}

#[derive(Debug)]
pub enum DirectProductError {
    /// The operation failed because the elements have different numbers of components.
    DifferentComponentCount,
    /// An error occurred in one ofthe underlying modulo operations.
    Modulo(ModuloError),
}

impl fmt::Display for DirectProductError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DirectProductError::DifferentComponentCount => write!(f, "Direct product elements have different numbers of components"),
            DirectProductError::Modulo(e) => write!(f, "A component-wise operation failed: {}", e),
        }
    }
}

impl Error for DirectProductError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DirectProductError::Modulo(e) => Some(e),
            _ => None,
        }
    }
}

// This allows the `?` operator to work seamlessly
impl From<ModuloError> for DirectProductError {
    fn from(err: ModuloError) -> Self {
        DirectProductError::Modulo(err)
    }
}

impl CheckedOp for DirectProductElement {
    type Error = DirectProductError; // Use the new error type

    fn checked_op(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.components.len() != other.components.len() {
            log::error!("Direct product elements must have the same number of components");
            return Err(DirectProductError::DifferentComponentCount);
        }

        let new_components = self.components.iter()
            .zip(other.components.iter())
            .map(|(c1, c2)| c1.checked_op(c2))
            .collect::<Result<Vec<_>, _>>()?; // This now converts ModuloError into DirectProductError

        Ok(DirectProductElement { components: new_components })
    }
}


#[cfg(test)]
mod test_direct_product {
    use super::*;

    #[test]
    fn test_direct_product_element_op() {
        let a = DirectProductElement {
            components: vec![Modulo::<Additive>::try_new(1, 3).unwrap(), Modulo::<Additive>::try_new(2, 5).unwrap()],
        };
        let b = DirectProductElement {
            components: vec![Modulo::<Additive>::try_new(2, 3).unwrap(), Modulo::<Additive>::try_new(3, 5).unwrap()],
        };
        let c = a.op(&b);

        assert_eq!(c.components[0].value(), 0); // (1 + 2) % 3
        assert_eq!(c.components[1].value(), 0); // (2 + 3) % 5
    }

    #[test]
    fn test_direct_product_element_inverse() {
        let a = DirectProductElement {
            components: vec![Modulo::<Additive>::try_new(1, 3).unwrap(), Modulo::<Additive>::try_new(2, 5).unwrap()],
        };
        let inverse = a.inverse();
        
        assert_eq!(inverse.components[0].value(), 2); // (3 - 1) % 3
        assert_eq!(inverse.components[1].value(), 3); // (5 - 2) % 5
    }

    #[test]
    fn test_direct_product_element_checked_op() {
        let a = DirectProductElement {
            components: vec![Modulo::<Additive>::try_new(1, 3).unwrap(), Modulo::<Additive>::try_new(2, 5).unwrap()],
        };
        let b = DirectProductElement {
            components: vec![Modulo::<Additive>::try_new(2, 3).unwrap(), Modulo::<Additive>::try_new(3, 5).unwrap()],
        };
        let c = a.checked_op(&b).expect("should succeed");

        assert_eq!(c.components[0].value(), 0); // (1 + 2) % 3
        assert_eq!(c.components[1].value(), 0); // (2 + 3) % 5
    }
    #[test]
    fn test_direct_product_element_different_component_count() {
        let a = DirectProductElement {
            components: vec![Modulo::<Additive>::try_new(1, 3).unwrap()],
        };
        let b = DirectProductElement {
            components: vec![Modulo::<Additive>::try_new(2, 5).unwrap(), Modulo::<Additive>::try_new(3, 7).unwrap()],
        };
        
        let result = a.checked_op(&b);
        assert!(result.is_err());
        // assert_eq!(result.unwrap_err(), DirectProductError::DifferentComponentCount);
        match result {
            Err(DirectProductError::DifferentComponentCount) => (), // This works
            _ => panic!("Expected DifferentComponentCount error"),
        }

    }

    #[test]
    fn test_direct_product_element_op_with_different_moduli() {
        let a = DirectProductElement {
            components: vec![Modulo::<Additive>::try_new(1, 3).unwrap(), Modulo::<Additive>::try_new(2, 5).unwrap()],
        };
        let b = DirectProductElement {
            components: vec![Modulo::<Additive>::try_new(2, 4).unwrap(), Modulo::<Additive>::try_new(3, 6).unwrap()],
        };
        
        let result = a.checked_op(&b);
        assert!(result.is_err());
        // assert_eq!(result.unwrap_err(), DirectProductError::Modulo(ModuloError::DifferentModuli));
        match result {
            Err(DirectProductError::Modulo(ModuloError::DifferentModuli)) => (), // This works
            _ => panic!("Expected DifferentModuli error"),
        }
    }
}