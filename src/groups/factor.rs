
use crate::error::AbsaglError;

use super::Group;
use super::GroupError;
use super::FiniteGroup;
use super::GroupElement;




use std::fmt;
use std::marker::PhantomData;


#[derive(Debug, PartialEq)]
pub enum CosetError {
    /// The provided subgroup is invalid.
    InvalidSubgroup(GroupError),
    // You could add other coset-specific errors here later, e.g.:
    // RepresentativeNotInParentGroup,
}

impl fmt::Display for CosetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CosetError::InvalidSubgroup(e) => write!(f, "Invalid subgroup: {}", e),
            // Add more cases as you add more error variants
        }
    }
}



/// Represents a coset gN of a normal subgroup N.
#[derive(Debug, Clone)]
pub struct Coset<'a, T: GroupElement> {
    pub representative: T,
    pub normal_subgroup: &'a FiniteGroup<T>,
    _marker: PhantomData<&'a T>,
}

/// Equality for cosets: aN == bN if and only if a⁻¹b ∈ N.
impl<'a, T: GroupElement> PartialEq for Coset<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        let a_inv = self.representative.inverse();
        let a_inv_b = a_inv.op(&other.representative);
        self.normal_subgroup.elements.contains(&a_inv_b)
    }
}
impl<'a, T: GroupElement> Eq for Coset<'a, T> {}

// Now, implement the core group operations for the Coset.
// This is where the magic happens! The logic is generic.
impl<'a, T: GroupElement> GroupElement for Coset<'a, T> {
    type Error = T::Error; // Or a new error type

    /// Operation for cosets: (aN)(bN) = (ab)N
    fn op(&self, other: &Self) -> Self {
        Coset {
            representative: self.representative.op(&other.representative),
            normal_subgroup: self.normal_subgroup,
            _marker: PhantomData,
        }
    }

    

    /// Inverse of a coset: (gN)⁻¹ = g⁻¹N
    fn inverse(&self) -> Self {
        Coset {
            representative: self.representative.inverse(),
            normal_subgroup: self.normal_subgroup,
            _marker: PhantomData,
        }
    }
    
    /// safe operation for cosets: (aN)(bN)=(ab)N
    fn safe_op(&self, other: &Self) -> Result<Self, Self::Error> {
        
        let representative = self.representative.safe_op(&other.representative)?;
        Ok(Coset {
            representative: representative,
            normal_subgroup: self.normal_subgroup,
            _marker: PhantomData,
        })
        
    }
}


impl<'a, T: GroupElement> Coset<'a, T> {


    /// create a coset, will check if the normal subgroup is abelian
    pub fn new(representative:T, subgroup: &'a FiniteGroup<T>) -> Result<Coset<'a, T>, AbsaglError> {
        if !subgroup.is_closed() {
            return Err(CosetError::InvalidSubgroup(GroupError::NotClosed))?;
        }

        if !subgroup.is_abelian() {
            return Err(CosetError::InvalidSubgroup(GroupError::NotAbelian))?;
        }

        Ok(Coset {
            representative: representative,
            normal_subgroup: subgroup,
            _marker: PhantomData,
        })

    }

    /// enumerate a full coset based on representative
    pub fn enumerate_coset(&self)->FiniteGroup<T> {
        if self.normal_subgroup.elements.contains(&self.representative) {
            return self.normal_subgroup.clone();
        }
        // let mut elements = Vec::new();
        // for g in &self.normal_subgroup.elements {
        //     let new_element = g.op(&self.representative);
        //     if !elements.contains(&new_element) {
        //         elements.push(new_element);
        //     }
        // }
        // FiniteGroup::new(elements)

        // Use iterators for an efficient O(n) implementation.
        let elements: Vec<T> = self.normal_subgroup
            .elements
            .iter()
            .map(|n| self.representative.op(n)) // Follows left coset convention (g*n)
            .collect();

        FiniteGroup::new(elements)
    }

}



/// FactorGroup struct. It borrows the groups it's built from.
pub struct FactorGroup<'a, T: GroupElement> {
    group: &'a FiniteGroup<T>,
    normal_subgroup: &'a FiniteGroup<T>,
}

impl<'a, T: GroupElement> Group<Coset<'a, T>> for FactorGroup<'a, T> {
    /// The group operation for cosets is (aN)(bN) = (ab)N.
    fn operate(&self, a: &Coset<'a, T>, b: &Coset<'a, T>) -> Coset<'a, T> {
        Coset {
            representative: a.representative.op(&b.representative),
            normal_subgroup: self.normal_subgroup,
            _marker: PhantomData,
        }
    }

    /// The inverse of a coset (gN)⁻¹ is g⁻¹N.
    fn inverse(&self, element: &Coset<'a, T>) -> Coset<'a, T> {
        Coset {
            representative: element.representative.inverse(),
            normal_subgroup: self.normal_subgroup,
            _marker: PhantomData,
        }
    }

    /// The identity element of the factor group is the coset eN.
    fn identity(&self) -> Coset<'a, T> {
        Coset {
            // Assumes FiniteGroup has an identity() method
            representative: self.group.identity(),
            normal_subgroup: self.normal_subgroup,
            _marker: PhantomData,
        }
    }

    /// The order of G/N is |G| / |N| by Lagrange's Theorem.
    fn order(&self) -> usize {
        if self.normal_subgroup.order() == 0 {
            0
        } else {
            self.group.order() / self.normal_subgroup.order()
        }
    }

    /// A factor group is always closed by definition.
    fn is_closed(&self) -> bool {
        true
    }

    /// Checks if the factor group is abelian.
    fn is_abelian(&self) -> bool {
        // We can check this by generating all cosets and comparing them.
        // First, get the unique cosets.
        let mut unique_cosets: Vec<Coset<T>> = Vec::new();
        for g in &self.group.elements {
            let coset = Coset {
                representative: g.clone(),
                normal_subgroup: self.normal_subgroup,
                _marker: PhantomData,
            };
            if !unique_cosets.contains(&coset) {
                unique_cosets.push(coset);
            }
        }

        // Now, check the abelian property for all pairs.
        for a in &unique_cosets {
            for b in &unique_cosets {
                if self.operate(a, b) != self.operate(b, a) {
                    return false;
                }
            }
        }
        true
    }
}


#[cfg(test)]
mod test_coset{

    use crate::groups::{modulo::Modulo};
    use super::*;


    #[test]
    fn test_coset_create_err() {
        let a = Modulo::new(0, 3).expect("Failed to create Modulo element");
        let b = Modulo::new(1, 3).expect("Failed to create Modulo element");

        let group = FiniteGroup::new(vec![a, b]);

        let result = Coset::new(b, &group);

        match result {
            Err(AbsaglError::Coset(CosetError::InvalidSubgroup(GroupError::NotClosed))) => {
                //
            },
            _ => panic!("Expected Err(AbsaglError::Coset(CosetError::InvalidSubgroup(GroupError::NotClosed))), but got {:?}", result),
        }

    }

    
    #[test]
    fn test_coset_eq() {
        let a = Modulo::new(2, 6).expect("should create element");

        let e  = Modulo::new(0, 6).expect("should create element");
        let b = Modulo::new(2, 6).expect("should create element");
        let c = Modulo::new(4, 6).expect("should create element");

        let group = FiniteGroup::new(vec![e,b,c]);

        let coset1 = Coset {
            representative: a,
            normal_subgroup: &group,
            _marker: PhantomData,
        };

        let coset2 = Coset {
            representative: c,
            normal_subgroup: &group,
            _marker: PhantomData,
        };

        assert!(coset1==coset2, "different representative should be equal");

    }

    #[test]
    fn test_coset_op() {
        let a = Modulo::new(2, 6).expect("should create element");

        let e  = Modulo::new(0, 6).expect("should create element");
        let b = Modulo::new(2, 6).expect("should create element");
        let c = Modulo::new(4, 6).expect("should create element");

        let group = FiniteGroup::new(vec![e,b,c]);
        let coset1 = Coset {
            representative: a,
            normal_subgroup: &group,
            _marker: PhantomData,
        };

        let expected = Coset {
            representative: c,
            normal_subgroup: &group,
            _marker: PhantomData,
        };
   
        let coset_result = coset1.op(&coset1); 

        assert!(coset_result==expected, "they should equal")
    }

    #[test]
    fn test_coset_inverse() {
        let a = Modulo::new(2, 6).expect("should create element");

        let e  = Modulo::new(0, 6).expect("should create element");
        let b = Modulo::new(2, 6).expect("should create element");
        let c = Modulo::new(4, 6).expect("should create element");

        let group = FiniteGroup::new(vec![e,b,c]);

        let coset1 = Coset {
            representative: a,
            normal_subgroup: &group,
            _marker: PhantomData,
        };

        let coset1_inv = coset1.inverse();
        assert!(coset1==coset1_inv, "in coset, inverse should equal to itself");
    }

    #[test]
    fn test_enumerate_coset(){
        let a = Modulo::new(2, 6).expect("should create element");

        let e  = Modulo::new(0, 6).expect("should create element");
        let b = Modulo::new(2, 6).expect("should create element");
        let c = Modulo::new(4, 6).expect("should create element");

        let group = FiniteGroup::new(vec![e,b,c]);

        let coset1 = Coset {
            representative: a,
            normal_subgroup: &group,
            _marker: PhantomData,
        };

        let mut coset1_subgroup = coset1.enumerate_coset();
        coset1_subgroup.elements.sort_unstable();

        eprintln!("element in coset1 is {:?}", coset1_subgroup.elements);

        assert!(coset1_subgroup.elements==group.elements, "should be true");

        let d1 = Modulo::new(1,6).expect("should create an element");
        let d3 = Modulo::new(3,6).expect("should create an element");
        let d5 = Modulo::new(5,6).expect("should create an element");

        
        
        let coset2 = Coset {
            representative: d1,
            normal_subgroup: &group,
            _marker: PhantomData,
        };

        let coset2_set = coset2.enumerate_coset();
        eprintln!("coset2_set is {:?}", coset2_set);


        assert!(coset2_set.elements==vec![d1,d3,d5], "should be true")


    }
}

#[cfg(test)]
mod test_factor_group {

    use crate::groups::{modulo::Modulo, GroupGenerators};
    use super::*;


    #[test]
    fn test_factor_group_order(){

        let e  = Modulo::new(0, 6).expect("should create element");
        let b = Modulo::new(2, 6).expect("should create element");
        let c = Modulo::new(4, 6).expect("should create element");

        let normal_subgroup = FiniteGroup::new(vec![e,b,c]);
        let group = GroupGenerators::generate_modulo_group(6).expect("should generate group");

        let factor_group = FactorGroup {
            group: &group,
            normal_subgroup: &normal_subgroup,
        };

        assert!(factor_group.order()==2, "the order should be 2");


    }

    #[test]
    fn test_factor_group_is_abelian() {
        let e  = Modulo::new(0, 6).expect("should create element");
        let b = Modulo::new(2, 6).expect("should create element");
        let c = Modulo::new(4, 6).expect("should create element");

        let normal_subgroup = FiniteGroup::new(vec![e,b,c]);
        let group = GroupGenerators::generate_modulo_group(6).expect("should generate group");

        let factor_group = FactorGroup {
            group: &group,
            normal_subgroup: &normal_subgroup,
        };

        assert!(factor_group.is_abelian(), "should be true");

    }





}