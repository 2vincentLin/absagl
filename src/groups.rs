pub mod modulo;
pub mod permutation;
pub mod dihedral;
pub mod factor;


use std::fmt;
use std::error::Error;

use crate::error::AbsaglError;

/// A trait representing a group element, it can be finite or cost.
pub trait GroupElement: Clone + PartialEq {
    type Error;
    /// The group operation (usually denoted as *)
    fn op(&self, other: &Self) -> Self;

    /// The inverse of the element
    fn inverse(&self) -> Self;

    fn safe_op(&self, other: &Self) -> Result<Self, Self::Error>
        where
            Self: Sized;
    
}


/// A trait to represent Group, that holds a finite number of elements
pub trait Group<T: GroupElement> {

    /// Applies the group operation to two elements
    fn operate(&self, a: &T, b: &T) -> T;
      
    /// Returns the inverse of an element
    fn inverse(&self, element: &T) -> T;

    /// Return identity of the Group, this will search through the whole group.
    /// If you want to know the specific identity like Permutation(n)'s,
    /// directly call from it
    fn identity(&self) -> T;

    /// Returns the order of the group, which is the number of elements in the group
    fn order(&self) -> usize;

    /// Checks if the group is closed under the group operation
    fn is_closed(&self) -> bool;

    /// Checks if the group is abelian
    fn is_abelian(&self) -> bool;

    // Checks if a subgroup of the group is normal in it
    // fn is_normal(&self, subgroup: &Group<T>) -> bool;
}


// A generic group struct holding elements of type T
#[derive(Debug, Clone)]
pub struct FiniteGroup<T: GroupElement> {
    pub elements: Vec<T>,
}

impl<T: GroupElement> Group<T> for FiniteGroup<T> {

      

    /// Applies the group operation to two elements
    fn operate(&self, a: &T, b: &T) -> T {
        a.op(b)
    }

    /// Returns the identity element of the group by looping through whole group,
    /// where e.op(x)==x and x.op(e)==x.
    /// if you need identity element from GroupElement, call element.identity()
    fn identity(&self) -> T {
        // Find the element e such that for all x, e.op(x) == x and x.op(e) == x
        self.elements.iter().find(|e| {
            self.elements.iter().all(|x| e.op(x) == *x && x.op(e) == *x)
        }).cloned().expect("No identity element found")
    }

    /// Returns the inverse of an element
    fn inverse(&self, element: &T) -> T {
        element.inverse()
    }

    /// Returns the order of the group, which is the number of elements in the group
    fn order(&self) -> usize {
        // todo: return the order of the group
        self.elements.len()
        
    }

    /// Checks if the group is closed under the group operation
    fn is_closed(&self) -> bool {
        for i in &self.elements {
            for j in &self.elements {
                let result = self.operate(i, j);
                if !self.elements.contains(&result) {
                    return false;
                }
            }
        }
        true
    }

    /// check if the group is abelian
    fn is_abelian(&self) -> bool {
        for i in &self.elements {
            for j in &self.elements {
                if i.op(j) != j.op(i) {
                    return false;
                }
            }
        }
        true
    }

    



}

impl<T: GroupElement> FiniteGroup<T> {
    /// Creates a new group with the given elements
    pub fn new(elements: Vec<T>) -> Self {
        FiniteGroup { elements }
    }

    ///check if a subgroup of the group is normal in it
    pub fn is_normal(&self, subgroup: &FiniteGroup<T>) -> bool {
        for g in &self.elements {
            for h in &subgroup.elements {
                let conjugate = g.op(h).op(&g.inverse());
                if !self.elements.contains(&conjugate) {
                    return false;
                }
            }
        }
        true
    }
}


#[derive(Debug, PartialEq)]
pub enum GroupError {
    NotClosed,
    NotAbelian, // this is for abelian group
    // some operation error
}

impl fmt::Display for GroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupError::NotClosed => write!(f, "This group is not closed"),
            GroupError::NotAbelian => write!(f, "This group is not abelian"),
            
        }
    }
}

impl Error for GroupError {}

/// a collection of group generators
/// This struct is used to generate groups of different types
pub struct GroupGenerators;

impl GroupGenerators {
    /// Generates a dihedral group
    pub fn generate_modulo_group(n: usize) -> Result<FiniteGroup<modulo::Modulo>, AbsaglError> {
        // This function can be used to generate a modulo group
        // Example: Modulo::generate_group(3);
        // You can implement this in the modulo module
        let elements = modulo::Modulo::generate_group(n as u64)?;
        Ok(FiniteGroup::new(elements))
    }
    /// Generates permutation groups
    pub fn generate_permutation_group(n: usize) -> Result<FiniteGroup<permutation::Permutation>, AbsaglError> {
        // This function can be used to generate a permutation group
        // Example: Permutation::generate_group(3);
        // You can implement this in the permutation module
        let elements = permutation::Permutation::generate_group(n)?;
        Ok(FiniteGroup::new(elements))
    }
    /// Generates alternating groups
    pub fn generate_alternating_group(n: usize) -> Result<FiniteGroup<permutation::AlternatingGroupElement>, AbsaglError> {
        // This function can be used to generate an alternating group
        // Example: AlternatingGroupElement::generate_group(3);
        // You can implement this in the permutation module
        let elements = permutation::AlternatingGroupElement::generate_group(n)?;
        Ok(FiniteGroup::new(elements))
    }
    /// Generates dihedral groups
    pub fn generate_dihedral_group(n: usize) -> Result<FiniteGroup<dihedral::DihedralElement>, dihedral::DihedralError> {
        // This function can be used to generate a dihedral group
        // Example: DihedralElement::generate(3);
        // You can implement this in the dihedral module
        let elements = dihedral::DihedralElement::generate(n)?;
        Ok(FiniteGroup::new(elements))
        
    }
}