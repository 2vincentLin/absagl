pub mod modulo;
pub mod permutation;
pub mod dihedral;
pub mod factor;
pub mod directproduct;

use std::fmt::{self, Debug};
use std::error::Error;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use crate::error::AbsaglError;
use crate::utils;
use crate::groups::directproduct::DirectProductElement;

use rayon::prelude::*;


#[derive(Debug)]
pub enum GroupError {
    NotClosed,
    NotAbelian, // this is for abelian group
    NotSubgroup,
    NotNormalSubgroup,
    NotFound, // this is for identity not found
    
    // some operation error
}

impl fmt::Display for GroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupError::NotClosed => write!(f, "This group is not closed"),
            GroupError::NotAbelian => write!(f, "This group is not abelian"),
            GroupError::NotSubgroup => write!(f, "The subgroup equal to whole group"),
            GroupError::NotNormalSubgroup => write!(f, "The subgroup is not normal subgroup in whole group"),
            GroupError::NotFound => write!(f, "Identity element not found in the group"),
            
        }
    }
}

impl Error for GroupError {}


/// A marker for additive group operations.
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Additive;

/// A marker for multiplicative group operations.
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Multiplicative;


/// A trait representing a group element, it can be finite or coset.
/// it must derive Clone, PartialEq, Eq, Hash, and Sync.
/// for PartialEq, Eq, Hash, it is used to compare two elements in the group.
/// for Sync, it is used to allow the element to be used in parallel computations.
pub trait GroupElement: Clone + PartialEq + Eq + Hash + Sync {
    /// The group operation (usually denoted as *)
    fn op(&self, other: &Self) -> Self;

    /// The inverse of the element
    fn inverse(&self) -> Self;
    
}


/// An optional trait for group elements that support a fallible, checked operation.
/// the Error should derive Debug, std::error::Error, Sync, Send + 'static, 
/// because in Coset checked_op, we need to wrap the underlying element T error 
pub trait CheckedOp: GroupElement {
    /// The specific error type for this element's operation.
    type Error: Error + Send + Sync + 'static;

    /// A fallible version of the group operation.
    fn checked_op(&self, other: &Self) -> Result<Self, Self::Error>;
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

/// this trait is mainly used to return byte representation of the GroupElement.
/// the reason we need this is because we need to put FiniteGroup<T> to HashMap.
/// this requires the underlying Vec<T> to return orderless unique hash value,
/// which means we need to sort the Vec<T> 1st, but we can't use sort() on the Vec<T>,
/// because this requires the GroupElement derive Ord trait, but for Permutation, the order is meaningless.
pub trait CanonicalRepr {
    /// Returns a unique, stable byte representation of the element.
    fn to_canonical_bytes(&self) -> Vec<u8>;
}


/// A generic group struct holding elements of type T
#[derive(Debug, Clone)]
pub struct FiniteGroup<T: GroupElement> {
    elements: Vec<T>,
}

impl<T: GroupElement> Group<T> for FiniteGroup<T> {

      

    /// Applies the group operation to two elements
    fn operate(&self, a: &T, b: &T) -> T {
        a.op(b)
    }

    /// Returns the identity element of the group by looping through whole group,
    /// where `e.op(x)==x` and `x.op(e)==x`.
    /// if you need identity element from GroupElement, call `element.identity()`
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
        
        self.elements.len()
        
    }

    /// Checks if the group is closed under the group operation
    /// A group is closed if for all elements i and j in the group, i.op(j) is also in the group.
    /// this is a single-threaded implementation, if you want to use parallel computing, use `is_closed_parallel()`
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
    

    /// check if the group is abelian, a group is abelian if for all elements i and j in the group, i.op(j) == j.op(i)
    /// this is a single-threaded implementation, if you want to use parallel computing, use `is_abelian_parallel()`
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

   
    /// returns a reference to the elements of the group
    pub fn elements(&self) -> &[T] {
        &self.elements
    }

    /// Creates a new group with the given elements, this is unchecked constructor.
    pub fn new(elements: Vec<T>) -> Self {
        FiniteGroup { elements }
    }

    /// Creates a new group with the given elements, this will check if given `Vec<T>` is closed
    pub fn try_new(elements: Vec<T>) -> Result<Self, AbsaglError> {

        let group = FiniteGroup { elements };
        if !group.is_closed() {
            return Err(GroupError::NotClosed)?;
        }

        Ok(group)
    }

    /// check if a given subgroup is normal in the group
    pub fn is_normal(&self, subgroup: &FiniteGroup<T>) -> bool {
        for g in &self.elements {
            for h in &subgroup.elements {
                let conjugate = g.op(h).op(&g.inverse());
                if !subgroup.elements.contains(&conjugate) {
                    return false;
                }
            }
        }
        true
    }
    /// Checks if the group is closed in parallel, this is useful for parallel computing.
    /// It checks if for all elements i and j in the group, the result of the
    /// group operation is also in the group.
    pub fn is_closed_parallel(&self) -> bool {
        self.elements.par_iter().all(|i|
            self.elements.par_iter().all(|j|
                self.elements.contains(&self.operate(i, j))
            )
        )
    }

    /// Checks if the group is abelian in parallel, this is useful for parallel computing.
    pub fn is_abelian_parallel(&self) -> bool {
        self.elements.par_iter().all(|i| 
            self.elements.par_iter().all(|j| 
                i.op(j) == j.op(i)
            )
        )
    }



    /// generate a normal subgroup given Vec<T>, it'll return error if the generete subgroup is equal to the whole group, 
    /// or the subgroup is not a normal subgroup in the whole group.
    /// for permutation group, the given Vec<T> should come from element in altenative group (even permutationh);
    /// for modulo<Additive>, the normal subgroup is gcd(x,n)>1
    pub fn generate_normal_subgroup(&self, generators: Vec<T>) -> Result<FiniteGroup<T>, AbsaglError> {
        // 1. Initialize
        let mut subgroup_elements = HashSet::new();
        subgroup_elements.insert(self.identity()); // Start with the identity

        let mut queue: Vec<T> = Vec::new();

        // Add initial generators
        for g in generators {
            if subgroup_elements.insert(g.clone()) {
                queue.push(g);
            }
        }

        // 2. Loop and Close
        while let Some(g) = queue.pop() {
            // Use a copy of the elements to avoid borrowing issues
            let current_elements: Vec<T> = subgroup_elements.iter().cloned().collect();

            for h in current_elements {
                let product = self.operate(&g, &h); 

                // 3. If a new element is found, add it to the set and the queue
                if subgroup_elements.insert(product.clone()) {
                    queue.push(product);
                }
            }
        }

        let subgroup_elements: Vec<T> = subgroup_elements.into_iter().collect();

     
        if subgroup_elements.len() == self.elements.len() {
            log::error!("The generated subgroup is the whole group");
            return Err(GroupError::NotSubgroup)?
        }

        let subgroup = FiniteGroup::try_new(subgroup_elements)?;

        if !self.is_normal(&subgroup) {
            log::error!("The generated subgroup is not normal in whole group");
            return Err(GroupError::NotNormalSubgroup)?
        }

        // 4. Return the new group from the final set of elements
        Ok(subgroup)

    }

    /// If the group is abelian, computes its decomposition into a direct product
    /// of cyclic groups of prime-power orders.
    pub fn abelian_decomposition(&self) -> Result<AbelianDecomposition, GroupError> {
        if !self.is_abelian() {
            log::error!("The group is not abelian, cannot compute decomposition");
            return Err(GroupError::NotAbelian);
        }

        let order = self.order() as u64;
        if order == 0 {
            // Or handle as another error type
            log::error!("The group order is zero");
            return Err(GroupError::NotFound);
        }
        
        // You'll need a prime factorization function.
        // Let's assume it's in `src/utils.rs`.
        let prime_factors = utils::prime_factorization(order);

        Ok(AbelianDecomposition {
            prime_power_orders: prime_factors,
        })
    }


}

impl<T: GroupElement> PartialEq for FiniteGroup<T> {
    fn eq(&self, other: &Self) -> bool {
        // Two groups are equal if they have the same number of elements
        // and the same set of elements. Using HashSets is a great way
        // to compare the elements while ignoring order.

        if self.elements.len() != other.elements.len() {
            return false;
        }

        let self_set: HashSet<_> = self.elements.iter().cloned().collect();
        let other_set: HashSet<_> = other.elements.iter().cloned().collect();

        self_set == other_set
    }
}

/// Don't forget to add this boilerplate impl for Eq
impl<T: GroupElement> Eq for FiniteGroup<T> {}

/// to impl Hash for FiniteGroup<T>, because for some GroupElement like Permutation, doesn't have meaning of ordering.
/// so simply derive Ord is a bad design, you don't know when it'll create a computation bug.
// impl<T: GroupElement + Ord> Hash for FiniteGroup<T> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         // To create a consistent hash, we must sort the elements first.
//         // This requires T to implement the `Ord` trait.
//         let mut sorted_elements = self.elements.clone();
//         sorted_elements.sort();
//         sorted_elements.hash(state);
//     }
// }

impl<T: GroupElement + CanonicalRepr> Hash for FiniteGroup<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Create a list of the canonical byte representations.
        let mut canonical_forms: Vec<Vec<u8>> = self
            .elements
            .iter()
            .map(|elem| elem.to_canonical_bytes())
            .collect();

        // Sort these byte vectors lexicographically. This is a safe, arbitrary order.
        canonical_forms.sort();

        // Hash the sorted list of canonical forms.
        canonical_forms.hash(state);
    }
}


/// Represents the decomposition of a finite abelian group
/// into a direct product of cyclic groups of prime-power order.
#[derive(Debug, PartialEq, Eq)]
pub struct AbelianDecomposition {
    pub prime_power_orders: Vec<(u64, u32)>, // Vec of (prime, exponent)
}

impl AbelianDecomposition {
    /// Returns the order of the group represented by this decomposition.
    pub fn order(&self) -> u64 {
        self.prime_power_orders
            .iter()
            .map(|(p, k)| p.pow(*k))
            .product()
    }
}







/// Represents the direct product group structure itself.
#[derive(Debug, Clone)]
pub struct DirectProductGroup {
    /// The list of cyclic groups (Z_p^k) that form the direct product.
    pub factors: Vec<FiniteGroup<modulo::Modulo<Additive>>>,
}

impl DirectProductGroup {
    /// Creates a new direct product group from an abelian decomposition.
    pub fn from_decomposition(decomposition: &AbelianDecomposition) -> Result<Self, AbsaglError> {
        let mut factors = Vec::new();
        for (p, k) in &decomposition.prime_power_orders {
            let order = p.pow(*k);
            // Use your existing generator for Z_n
            let cyclic_group = GroupGenerators::generate_modulo_group_add(order as usize)?;
            factors.push(cyclic_group);
        }
        Ok(DirectProductGroup { factors })
    }

    /// Returns the identity element of the direct product group.
    pub fn identity(&self) -> DirectProductElement {
        let components = self.factors.iter()
            .map(|group| group.identity())
            .collect();
        DirectProductElement { components }
    }
}

/// a collection of group generators
/// This struct is used to generate groups of different types
pub struct GroupGenerators;

impl GroupGenerators {
    /// Generates a modulo group with additive operation
    pub fn generate_modulo_group_add(n: usize) -> Result<FiniteGroup<modulo::Modulo<Additive>>, AbsaglError> {
        let elements = modulo::Modulo::<Additive>::generate_group(n as u64)?;
        Ok(FiniteGroup::try_new(elements)?)
    }
    /// Generates a modulo group with Multiplicative operation
    pub fn generate_modulo_group_mul(n: usize) -> Result<FiniteGroup<modulo::Modulo<Multiplicative>>, AbsaglError> {
        let elements = modulo::Modulo::<Multiplicative>::generate_group(n as u64)?;
        Ok(FiniteGroup::try_new(elements)?)
    }
    /// Generates permutation groups
    pub fn generate_permutation_group(n: usize) -> Result<FiniteGroup<permutation::Permutation>, AbsaglError> {
        let elements = permutation::Permutation::generate_group(n)?;
        Ok(FiniteGroup::try_new(elements)?)
    }
    /// Generates alternating groups
    pub fn generate_alternating_group(n: usize) -> Result<FiniteGroup<permutation::Permutation>, AbsaglError> {
        let elements = permutation::Permutation::generate_alternative_group(n)?;
        Ok(FiniteGroup::try_new(elements)?)
    }
    /// Generates dihedral groups
    pub fn generate_dihedral_group(n: usize) -> Result<FiniteGroup<dihedral::DihedralElement>, AbsaglError> {
        let elements = dihedral::DihedralElement::generate_group(n)?;
        Ok(FiniteGroup::try_new(elements)?)
        
    }
}




#[cfg(test)]
mod test_finite_group {

    // Import the necessary modules and traits
    use super::*;
    use crate::groups::modulo::Modulo;
    use crate::groups::permutation::Permutation;

    #[test]
    fn test_is_closed_true() {
        let a = Modulo::<Additive>::try_new(0, 3).expect("Failed to create Modulo element");
        let b = Modulo::<Additive>::try_new(1, 3).expect("Failed to create Modulo element");
        let c = Modulo::<Additive>::try_new(2, 3).expect("Failed to create Modulo element");

        let group = FiniteGroup::try_new(vec![a, b, c]).expect("should create a FiniteGroup");

        assert!(group.is_closed());
    }

    #[test]
    fn test_is_closed_false() {
        let a = Modulo::<Additive>::try_new(0, 3).expect("Failed to create Modulo element");
        let b = Modulo::<Additive>::try_new(1, 3).expect("Failed to create Modulo element");

        let result = FiniteGroup::try_new(vec![a, b]);
        match result {
            Err(AbsaglError::Group(GroupError::NotClosed)) => {
                // pass
            }
            _ => panic!("Expect Err(AbsaglError::Group(GroupError::NotClosed)), but got {:?}", result)
        }
        // assert!(!group.is_closed());
    }

    #[test]
    fn test_is_closed_parallel() {
        let s6 = Permutation::generate_group(6).expect("Failed to generate S6 group");
        let s6_group = FiniteGroup::new(s6);
        assert!(s6_group.is_closed_parallel());

        let s6 = Permutation::generate_group(6).expect("Failed to generate S6 group");
        let mut s6_missing = s6.clone();
        s6_missing.pop(); // Remove one element
        let s6_group_missing = FiniteGroup::new(s6_missing);
        assert_eq!(s6_group_missing.is_closed_parallel(), false);
    }

    #[test]
    fn test_is_abelian_parallel() {
        let z100 = Modulo::<Additive>::generate_group(100).expect("Failed to generate Z100 group");
        let z100_group = FiniteGroup::new(z100);
        assert!(z100_group.is_abelian_parallel());

        let s6 = Permutation::generate_group(6).expect("Failed to generate S6 group");
        let s6_group = FiniteGroup::new(s6);
        assert!(!s6_group.is_abelian_parallel());

    }

    #[test]
    fn test_is_abelian_true() {
        let a = Modulo::<Additive>::try_new(0, 3).expect("Failed to create Modulo element");
        let b = Modulo::<Additive>::try_new(1, 3).expect("Failed to create Modulo element");
        let c = Modulo::<Additive>::try_new(2, 3).expect("Failed to create Modulo element");

        let group = FiniteGroup::try_new(vec![a, b, c]).expect("should create a FiniteGroup");

        assert!(group.is_abelian());
    }

    #[test]
    fn test_is_abelian_false() {
        let s3 = GroupGenerators::generate_permutation_group(3).expect("Failed to generate S3 group");
        assert!(!s3.is_abelian());
    }

    #[test]
    fn test_generate_normal_subgroup_fail_not_subgroup() {
        let group = GroupGenerators::generate_modulo_group_add(5).unwrap();
        let g1 = Modulo::<Additive>::try_new(1,5).unwrap();

        let result = group.generate_normal_subgroup(vec![g1]);

        // println!("result is: {:?}", &result.unwrap());

        match result {
            Err(AbsaglError::Group(GroupError::NotSubgroup)) => {
                // pass
            }
            _ => panic!("Expected Err(AbsaglError::Group(GroupError::NotSubgroup)), but got {:?}", result),

        }
        
    }

    #[test]
    fn test_generate_normal_subgroup_fail_not_normal() {
        let group = GroupGenerators::generate_permutation_group(3).unwrap();
        let g1 = Permutation::from_cycles(&vec![vec![0,1]], 3).unwrap();

        let result = group.generate_normal_subgroup(vec![g1]);

        
        match result {
            Err(AbsaglError::Group(GroupError::NotNormalSubgroup)) => {
                // pass
            }
            _ => panic!("Expected Err(AbsaglError::Group(GroupError::NotNormalSubgroup)), but got {:?}", result),

        }
        
    }


    #[test]
    fn test_generate_normal_subgroup_success() {
        let group = GroupGenerators::generate_modulo_group_add(6).unwrap();
        let g2 = Modulo::<Additive>::try_new(2,6).unwrap();

        let result = group.generate_normal_subgroup(vec![g2]);

        // println!("result is: {:?}", &result.unwrap());



        match result {
            Ok(group) => {
                let g0 = Modulo::try_new(0,6).unwrap();
                let g2 = Modulo::try_new(2,6).unwrap();
                let g4 = Modulo::try_new(4,6).unwrap();
                let expected = FiniteGroup::try_new(vec![g0,g2,g4]).expect("should create a FiniteGroup");
                assert_eq!(group, expected);
                // pass
            }
            _ => panic!("Expected Err(AbsaglError::Group(GroupError::NotSubgroup)), but got {:?}", result),

        }
        
    }

    


    #[test]
    fn test_modulo_group_equal_success() {
        let a = Modulo::<Additive>::try_new(0, 3).expect("Failed to create Modulo element");
        let b = Modulo::<Additive>::try_new(1, 3).expect("Failed to create Modulo element");
        let c = Modulo::<Additive>::try_new(2, 3).expect("Failed to create Modulo element");

        let group1 = FiniteGroup::try_new(vec![a, b, c]).expect("should create a FiniteGroup");
        let group2 = FiniteGroup::try_new(vec![c, b, a]).expect("should create a FiniteGroup");
        println!("group1: {:?}", &group1);
        println!("group2: {:?}", &group2);
        assert_eq!(group1, group2);
    }

    #[test]
    fn test_modulo_group_equal_fail() {
        let g0 = Modulo::<Additive>::try_new(0, 4).expect("Failed to create Modulo element");
        let g1 = Modulo::<Additive>::try_new(1, 4).expect("Failed to create Modulo element");
        let g2 = Modulo::<Additive>::try_new(2, 4).expect("Failed to create Modulo element");
        let g3 = Modulo::<Additive>::try_new(3, 4).expect("Failed to create Modulo element");

        let group1 = FiniteGroup::try_new(vec![g0, g2]).expect("should create a FiniteGroup");
        let group2 = FiniteGroup::try_new(vec![g0,g1,g2,g3]).expect("should create a FiniteGroup");
        assert_ne!(group1, group2);
    }

    #[test]
    fn test_permutation_group_equal_success() {
        let e = Permutation::try_new(vec![0,1]).unwrap();
        let a = Permutation::try_new(vec![1,0]).unwrap();

        let group1 = FiniteGroup::try_new(vec![e.clone(),a.clone()]).expect("should create a FiniteGroup");
        let group2= FiniteGroup::try_new(vec![a.clone(),e.clone()]).expect("should create a FiniteGroup");
        assert_eq!(group1, group2);
    }

     #[test]
    fn test_permutation_group_equal_fail() {
        let e = Permutation::try_new(vec![0,1]).unwrap();
        let a = Permutation::try_new(vec![1,0]).unwrap();

        let group1 = FiniteGroup::try_new(vec![e.clone(),a.clone()]).expect("should create a FiniteGroup");
        let group2= GroupGenerators::generate_permutation_group(3).unwrap();
        if group1 != group2 {
            println!("not equal");
        }
        assert_ne!(group1, group2);
    }

    #[test]
    fn test_abelian_decomposition() {
        let group = GroupGenerators::generate_modulo_group_add(6).unwrap();
        let decomposition = group.abelian_decomposition().expect("should decompose");

        assert_eq!(decomposition.prime_power_orders, vec![(2, 1), (3, 1)]);
        assert_eq!(decomposition.order(), 6);
    }


    // #[test]
    // #[should_panic] // This test is expected to fail to compile, not panic at runtime
    // fn test_hash_for_permutation_group_fails_to_compile() {
    //     // This test will fail to compile because Permutation does not implement Ord,
    //     // which is required by the Hash implementation for FiniteGroup.
    //     use std::collections::HashSet;

    //     let group: FiniteGroup<Permutation> = GroupGenerators::generate_permutation_group(3).unwrap();
    //     let mut set = HashSet::new();

    //     // The line below will cause a compile-time error
    //     // because it requires FiniteGroup<Permutation> to be hashable.
    //     set.insert(group);
    // }

}



#[cfg(test)]
mod test_group_generators {

    use super::*;
    use crate::groups::GroupGenerators;

    #[test]
    fn test_generate_modulo_group_add() {
        let group = GroupGenerators::generate_modulo_group_add(5).expect("Failed to generate modulo group");
        assert_eq!(group.order(), 5);
    }

    #[test]
    fn test_generate_modulo_group_mul() {
        let group = GroupGenerators::generate_modulo_group_mul(5).expect("Failed to generate modulo group");
        assert_eq!(group.order(), 4); // 0 is not included in multiplicative group
    }

    #[test]
    fn test_generate_permutation_group() {
        let group = GroupGenerators::generate_permutation_group(3).expect("Failed to generate permutation group");
        assert_eq!(group.order(), 6); // S3 has 6 elements
    }

    #[test]
    fn test_generate_alternating_group() {
        let group = GroupGenerators::generate_alternating_group(3).expect("Failed to generate alternating group");
        assert_eq!(group.order(), 3); // A3 has 3 elements
    }

    #[test]
    fn test_generate_dihedral_group() {
        let group = GroupGenerators::generate_dihedral_group(3).expect("Failed to generate dihedral group");
        assert_eq!(group.order(), 6); // D3 has 6 elements
    }
}