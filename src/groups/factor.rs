
use crate::error::AbsaglError;

use crate::groups::{CanonicalRepr, CheckedOp, FiniteGroup, Group, GroupElement, GroupError};


use std::fmt::{self};
use std::marker::PhantomData;
use std::hash::{Hash,Hasher};
use std::error::Error;


#[derive(Debug)]
pub enum CosetError {
    /// The provided subgroup is invalid.
    InvalidSubgroup(GroupError),
    MixLeftAndRightCoset,
    DifferentSubgroup,
    Element(Box<dyn Error + Send + Sync + 'static>),
    // You could add other coset-specific errors here later, e.g.:
    // RepresentativeNotInParentGroup,
}

impl fmt::Display for CosetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CosetError::InvalidSubgroup(e) => write!(f, "Invalid subgroup: {}", e),
            CosetError::MixLeftAndRightCoset => write!(f, "Cannot mix left/right coset"),
            CosetError::DifferentSubgroup => write!(f, "cannot operation cosets from different subgroup"),
            CosetError::Element(e) => write!(f, "Underlying element error: {}", e),
            // Add more cases as you add more error variants
        }
    }
}

impl Error for CosetError {}


/// a enum to represent left/right coset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CosetSide {
    Left,
    Right,
}

/// Represents a coset gH/Hg of a subgroup H.
#[derive(Debug, Clone)]
pub struct Coset<'a, T: GroupElement> {
    representative: T,
    subgroup: &'a FiniteGroup<T>,
    side: CosetSide,
    _marker: PhantomData<&'a T>,
}

/// Equality for cosets: aH == bH if and only if a⁻¹b ∈ H.
impl<'a, T: GroupElement> PartialEq for Coset<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        // A left coset can never equal a right coset
        if self.side != other.side {
            return false;
        }
        // check if the underlying subgroup equal 
        if self.subgroup != other.subgroup {
            return false;
        }
        let a_inv = self.representative.inverse();
        let a_inv_b = a_inv.op(&other.representative);
        self.subgroup.elements.contains(&a_inv_b)
    }
}
impl<'a, T: GroupElement> Eq for Coset<'a, T> {}

impl<'a, T: GroupElement> Hash for Coset<'a, T> 
where 
    T: CanonicalRepr,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Find the element in the coset with the "smallest" canonical form.
        let canonical_rep = self.subgroup.elements.iter()
            .map(|h| match self.side {
                CosetSide::Left => self.representative.op(h),  // g * h
                CosetSide::Right => h.op(&self.representative), // h * g
            }) 
            .min_by(|a, b| a.to_canonical_bytes().cmp(&b.to_canonical_bytes()))
            .unwrap(); // Assumes coset is not empty

        // Hash the canonical representative's bytes.
        canonical_rep.to_canonical_bytes().hash(state);

        // Also hash the subgroup itself.
        self.subgroup.hash(state);
    }
}

// Now, implement the core group operations for the Coset.
// This is where the magic happens! The logic is generic.
impl<'a, T: GroupElement + CanonicalRepr> GroupElement for Coset<'a, T> {

    /// Operation for cosets: (aN)(bN) = (ab)N, will panic if left coset op right coset, and vice versa
    fn op(&self, other: &Self) -> Self {
        assert_eq!(self.side, other.side, "cannot mix operation with left/right coset");
        Coset {
            representative: self.representative.op(&other.representative),
            subgroup: self.subgroup,
            side: self.side,
            _marker: PhantomData,
        }
    }

    

    /// Inverse of a coset: (gN)⁻¹ = g⁻¹N
    fn inverse(&self) -> Self {
        Coset {
            representative: self.representative.inverse(),
            subgroup: self.subgroup,
            side: self.side,
            _marker: PhantomData,
        }
    }
    
    
}


impl<'a, T: GroupElement + CheckedOp + CanonicalRepr> CheckedOp for Coset<'a, T> {
    type Error = CosetError;

    /// A fallible version of the group operation for cosets.
    fn checked_op(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.side != other.side {
            log::error!("cannot mix left/right coset for operation");
            return Err(CosetError::MixLeftAndRightCoset)?;
        }

        if self.subgroup != other.subgroup {
            log::error!("Cosets must be from the same subgroup");
            return Err(CosetError::DifferentSubgroup)?;
        }

        // Use the checked operation of the underlying group element
        let representative = self.representative.checked_op(&other.representative)
            .map_err(|e| CosetError::Element(Box::new(e)))?;

        Ok(Coset {
            representative,
            subgroup: self.subgroup,
            side: self.side,
            _marker: PhantomData,
        })
    }
}


impl<'a, T: GroupElement + CanonicalRepr + CheckedOp> Coset<'a, T> {

    
    /// create a coset, will check if the subgroup is closed
    pub fn new(representative:T, subgroup: &'a FiniteGroup<T>, side: CosetSide) -> Result<Coset<'a, T>, AbsaglError> {
        if !subgroup.is_closed() {
            return Err(CosetError::InvalidSubgroup(GroupError::NotClosed))?;
        }

        Ok(Coset {
            representative: representative,
            subgroup,
            side: side,
            _marker: PhantomData,
        })

    }

    /// enumerate a full coset based on representative
    pub fn enumerate_coset(&self) -> Vec<T> {
        if self.subgroup.elements.contains(&self.representative) {
            return self.subgroup.elements.clone();
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
        let elements: Vec<T> = self.subgroup
            .elements
            .iter()
            .map(|h| match self.side {
                CosetSide::Left => self.representative.op(h),  // g * h
                CosetSide::Right => h.op(&self.representative), // h * g
            }) // Follows left coset convention (g*n)
            .collect();
        // FiniteGroup {elements}

        elements

        // FiniteGroup::new(elements)
    }

    /// Finds the canonical representative of the coset.
    /// This is the element in the coset with the lexicographically smallest
    /// canonical byte representation.
    pub fn get_canonical_representative(&self) -> T {
        self.subgroup
            .elements
            .iter()
            // Generate all elements of the coset: { self.representative * h }
            .map(|h| match self.side {
                CosetSide::Left => self.representative.op(h),  // g * h
                CosetSide::Right => h.op(&self.representative), // h * g
            }) 
            // Find the one with the "smallest" canonical representation
            .min_by(|a, b| a.to_canonical_bytes().cmp(&b.to_canonical_bytes()))
            .unwrap() // A coset is never empty, so this is safe
    }



}


#[derive(Debug)]
/// FactorGroup struct. It borrows the groups it's built from.
pub struct FactorGroup<'a, T: GroupElement> {
    group: &'a FiniteGroup<T>,
    normal_subgroup: &'a FiniteGroup<T>,
}

impl<'a, T: GroupElement + CanonicalRepr> Group<Coset<'a, T>> for FactorGroup<'a, T> {
    /// The group operation for cosets is (aN)(bN) = (ab)N.
    fn operate(&self, a: &Coset<'a, T>, b: &Coset<'a, T>) -> Coset<'a, T> {
        Coset {
            representative: a.representative.op(&b.representative),
            subgroup: self.normal_subgroup,
            side: CosetSide::Left, // for factor group, use left coset
            _marker: PhantomData,
        }
    }

    /// The inverse of a coset (gN)⁻¹ is g⁻¹N.
    fn inverse(&self, element: &Coset<'a, T>) -> Coset<'a, T> {
        Coset {
            representative: element.representative.inverse(),
            subgroup: self.normal_subgroup,
            side: CosetSide::Left, // for factor group, use left coset
            _marker: PhantomData,
        }
    }

    /// The identity element of the factor group is the coset eN.
    fn identity(&self) -> Coset<'a, T> {
        Coset {
            representative: self.group.identity(),
            subgroup: self.normal_subgroup,
            side: CosetSide::Left, // for factor group, use left coset
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
                subgroup: self.normal_subgroup,
                side: CosetSide::Left, // for factor group, use left coset
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


impl<'a,T: GroupElement + CanonicalRepr> FactorGroup<'a,T> {
    /// given two FiniteGroup<T>, it'll return a FactorGroup<'a,T> if the both groups are closed.
    /// subgroup is normal in group and both group order is different.
    pub fn new(group: &'a FiniteGroup<T>, subgroup: &'a FiniteGroup<T>) -> Result<FactorGroup<'a, T>, AbsaglError> {
        if !group.is_closed() || !subgroup.is_closed() {
            log::error!("one of the group/subgroup is not closed");
            return Err(GroupError::NotClosed)?;
        }

        if !group.is_normal(subgroup) {
            log::error!("subgroup is not normal in group");
            return Err(GroupError::NotNormalSubgroup)?;
        }

        if group.order() == subgroup.order() {
            log::error!("the order of group and its subgroup is equal");
            return Err(GroupError::NotSubgroup)?;
        }

        Ok(FactorGroup { group: group, normal_subgroup: subgroup })
    }
}


#[cfg(test)]
mod test_coset{

    use crate::groups::{modulo::Modulo, Additive, Multiplicative};
    use crate::groups::permutation::{Permutation, PermutationError};
    use crate::groups::GroupGenerators;
    use super::*;


    #[test]
    fn test_coset_create_err() {
        let a = Modulo::<Additive>::new(0, 3).expect("Failed to create Modulo element");
        let b = Modulo::<Additive>::new(1, 3).expect("Failed to create Modulo element");

        // let group = FiniteGroup::new(vec![a, b]).expect("should create a FiniteGroup");
        let group = FiniteGroup { elements: vec![a,b] };

        let result = Coset::new(b, &group, CosetSide::Left);

        match result {
            Err(AbsaglError::Coset(CosetError::InvalidSubgroup(GroupError::NotClosed))) => {
                //
            },
            _ => panic!("Expected Err(AbsaglError::Coset(CosetError::InvalidSubgroup(GroupError::NotClosed))), but got {:?}", result),
        }

    }

    
    #[test]
    fn test_coset_eq_success() {
        let a = Modulo::<Additive>::new(2, 6).expect("should create element");

        let e  = Modulo::<Additive>::new(0, 6).expect("should create element");
        let b = Modulo::<Additive>::new(2, 6).expect("should create element");
        let c = Modulo::<Additive>::new(4, 6).expect("should create element");

        let group = FiniteGroup::new(vec![e,b,c]).expect("should create a FiniteGroup");

        let coset1 = Coset {
            representative: a,
            subgroup: &group,
            side: CosetSide::Left,
            _marker: PhantomData,
        };

        let coset2 = Coset {
            representative: c,
            subgroup: &group,
            side: CosetSide::Left,
            _marker: PhantomData,
        };

        assert!(coset1==coset2, "different representative should be equal");

    }

    #[test]
    fn test_coset_eq_fail() {

        let e = Modulo::<Additive>::new(0, 8).expect("should create element");
        let a = Modulo::<Additive>::new(2, 8).expect("should create element");
        let b = Modulo::<Additive>::new(4, 8).expect("should create element");
        let c = Modulo::<Additive>::new(6, 8).expect("should create element");


        let group1 = FiniteGroup::new(vec![e,a,b,c]).expect("should create a FiniteGroup");
        let group2 = FiniteGroup::new(vec![e,b]).expect("should create a FiniteGroup");

        let coset1 = Coset {
            representative: b,
            subgroup: &group1,
            side: CosetSide::Left,
            _marker: PhantomData,
        };

        let coset2 = Coset {
            representative: b,
            subgroup: &group2,
            side: CosetSide::Left,
            _marker: PhantomData,
        };

        assert!(coset1!=coset2, "different normal subgroup should not equal");
    }

    #[test]
    fn test_coset_op() {
        let a = Modulo::new(2, 6).expect("should create element");

        let e  = Modulo::<Additive>::new(0, 6).expect("should create element");
        let b = Modulo::<Additive>::new(2, 6).expect("should create element");
        let c = Modulo::<Additive>::new(4, 6).expect("should create element");

        let group = FiniteGroup::new(vec![e,b,c]).expect("should create a FiniteGroup");
        let coset1 = Coset {
            representative: a,
            subgroup: &group,
            side: CosetSide::Left,
            _marker: PhantomData,
        };

        let expected = Coset {
            representative: c,
            subgroup: &group,
            side: CosetSide::Left,
            _marker: PhantomData,
        };
   
        let coset_result = coset1.op(&coset1); 

        assert!(coset_result==expected, "they should equal")
    }


    #[test]
    fn test_coset_checked_op_fail_box_error() {
        let a = Permutation::new(vec![0,1,2]).unwrap();
        let b = Permutation::new(vec![0,1]).unwrap();
        let s3 = GroupGenerators::generate_permutation_group(3).unwrap();
        println!("s3: {:?}", s3);

        let coset1 = Coset::new(a, &s3, CosetSide::Left).unwrap();
        let coset2 = Coset::new(b, &s3, CosetSide::Left).unwrap();

        match coset1.checked_op(&coset2) {
            Ok(_) => panic!("should not success"),
            Err(CosetError::Element(e)) => {
                println!("e: {:?}", e);
                // Attempt to downcast the boxed error to a PermutationError
                if let Some(perm_error) = e.downcast_ref::<PermutationError>() {
                    // Success! It's the error type we expected.
                    println!("downcasted to PermutationError: {:?}", perm_error);
                    
                    match perm_error {
                        PermutationError::SizeNotMatch => {
                            // pass
                        }
                        _ => panic!("got other error: {:?}", perm_error)
                    }

                } else {
                    // The error inside the box was not a PermutationError
                    panic!("Expected a PermutationError, but got a different boxed error: {:?}", e);
                }
            },
            Err(e) => panic!("Expect Err(CosetError::Element(e)), but got {:?}", e)
        }
    }

    #[test]
    fn test_coset_inverse() {
        let a = Modulo::new(2, 6).expect("should create element");

        let e  = Modulo::<Additive>::new(0, 6).expect("should create element");
        let b = Modulo::<Additive>::new(2, 6).expect("should create element");
        let c = Modulo::<Additive>::new(4, 6).expect("should create element");

        let group = FiniteGroup::new(vec![e,b,c]).expect("should create a FiniteGroup");

        let coset1 = Coset {
            representative: a,
            subgroup: &group,
            side: CosetSide::Left,
            _marker: PhantomData,
        };

        let coset1_inv = coset1.inverse();
        assert!(coset1==coset1_inv, "in coset, inverse should equal to itself");
    }

    #[test]
    fn test_enumerate_coset(){
        let a = Modulo::<Additive>::new(2, 6).expect("should create element");

        let e  = Modulo::<Additive>::new(0, 6).expect("should create element");
        let b = Modulo::<Additive>::new(2, 6).expect("should create element");
        let c = Modulo::<Additive>::new(4, 6).expect("should create element");

        let group = FiniteGroup::new(vec![e,b,c]).expect("should create a FiniteGroup");

        let coset1 = Coset {
            representative: a,
            subgroup: &group,
            side: CosetSide::Left,
            _marker: PhantomData,
        };

        let mut coset1_set = coset1.enumerate_coset();
        coset1_set.sort_unstable();

        eprintln!("element in coset1 is {:?}", coset1_set);

        assert!(coset1_set==group.elements, "should be true");

        let d1 = Modulo::new(1,6).expect("should create an element");
        let d3 = Modulo::new(3,6).expect("should create an element");
        let d5 = Modulo::new(5,6).expect("should create an element");

        
        
        let coset2 = Coset {
            representative: d1,
            subgroup: &group,
            side: CosetSide::Left,
            _marker: PhantomData,
        };

        let coset2_set = coset2.enumerate_coset();
        eprintln!("coset2_set is {:?}", coset2_set);


        assert!(coset2_set==vec![d1,d3,d5], "should be true")


    }

    #[test]
    fn test_coset_get_canonical_representative() {
        let e = Modulo::<Additive>::new(0, 8).expect("should create element");
        let a = Modulo::<Additive>::new(2, 8).expect("should create element");
        let b = Modulo::<Additive>::new(4, 8).expect("should create element");
        let c = Modulo::<Additive>::new(6, 8).expect("should create element");

        let group = FiniteGroup::new(vec![e,a,b,c]).expect("should create a FiniteGroup");
        let coset1 = Coset::new(a, &group, CosetSide::Left).unwrap();

        assert_eq!(e, coset1.get_canonical_representative());

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

        let normal_subgroup = FiniteGroup::new(vec![e,b,c]).expect("should create a FiniteGroup");
        let group = GroupGenerators::generate_modulo_group_add(6).expect("should generate group");

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

        let normal_subgroup = FiniteGroup::new(vec![e,b,c]).expect("should create a FiniteGroup");
        let group = GroupGenerators::generate_modulo_group_add(6).expect("should generate group");

        let factor_group = FactorGroup {
            group: &group,
            normal_subgroup: &normal_subgroup,
        };

        assert!(factor_group.is_abelian(), "should be true");

    }





}