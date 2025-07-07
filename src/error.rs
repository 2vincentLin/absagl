// In src/error.rs
use std::fmt;

#[derive(Debug)]
pub enum AbsaglError {
    Modulo(crate::groups::modulo::ModuloError),
    Permutation(crate::groups::permutation::PermutationError),
    Dihedral(crate::groups::dihedral::DihedralError),
    Group(crate::groups::GroupError),
    Coset(crate::groups::factor::CosetError),
    // ... add other sub-errors
    Other(String),
}

impl fmt::Display for AbsaglError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbsaglError::Modulo(e) => write!(f, "Modulo error: {}", e),
            AbsaglError::Permutation(e) => write!(f, "Permutation error: {}", e),
            AbsaglError::Dihedral(e) => write!(f, "Diherdral error: {}", e),
            AbsaglError::Group(e) => write!(f, "Group error: {}", e),
            AbsaglError::Coset(e) => write!(f, "Coset error: {}", e),
            AbsaglError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl std::error::Error for AbsaglError {}


impl From<crate::groups::factor::CosetError> for AbsaglError {
    fn from(e: crate::groups::factor::CosetError) -> Self {
        AbsaglError::Coset(e)
    }
}

impl From<crate::groups::GroupError> for AbsaglError {
    fn from(e: crate::groups::GroupError) -> Self {
        AbsaglError::Group(e)
    }
}

impl From<crate::groups::modulo::ModuloError> for AbsaglError {
    fn from(e: crate::groups::modulo::ModuloError) -> Self {
        AbsaglError::Modulo(e)
    }
}

impl From<crate::groups::permutation::PermutationError> for AbsaglError {
    fn from(e: crate::groups::permutation::PermutationError) -> Self {
        AbsaglError::Permutation(e)
    }
}

impl From<crate::groups::dihedral::DihedralError> for AbsaglError {
    fn from(e: crate::groups::dihedral::DihedralError) -> Self {
        AbsaglError::Dihedral(e)
    }
}
