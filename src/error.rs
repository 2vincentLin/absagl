// In src/error.rs
use std::fmt;
use std::error::Error;


#[derive(Debug)]
pub enum AbsaglError {
    Modulo(crate::groups::modulo::ModuloError),
    Permutation(crate::groups::permutation::PermutationError),
    Dihedral(crate::groups::dihedral::DihedralError),
    Group(crate::groups::GroupError),
    Coset(crate::groups::factor::CosetError),
    Homomorphism(crate::homomorphism::HomomorphismError),
    // this new variant to hold the generic error from T
    Element(Box<dyn Error + Send + Sync + 'static>),
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
            AbsaglError::Homomorphism(e) => write!(f, "Homomorphism error: {}", e),
            AbsaglError::Element(e) => write!(f, "Underlying element error: {}", e),
            AbsaglError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl std::error::Error for AbsaglError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AbsaglError::Modulo(e) => Some(e),
            AbsaglError::Permutation(e) => Some(e),
            AbsaglError::Dihedral(e) => Some(e),
            AbsaglError::Group(e) => Some(e),
            AbsaglError::Coset(e) => Some(e),
            AbsaglError::Homomorphism(e) => Some(e),
            AbsaglError::Element(e) => Some(e.as_ref()),
            AbsaglError::Other(_) => None,
        }
    }
}


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

impl From<crate::homomorphism::HomomorphismError> for AbsaglError {
    fn from(e: crate::homomorphism::HomomorphismError) -> Self {
        AbsaglError::Homomorphism(e)
    }
}

impl From<String> for AbsaglError {
    fn from(e:String) -> Self {
        AbsaglError::Other(e)
    }
}
