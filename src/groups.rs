pub mod modulo;
pub mod permutation;
pub mod dihedral;
pub mod factor;


use std::fmt;
use std::error::Error;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use crate::error::AbsaglError;


/// A marker for additive group operations.
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Additive;

/// A marker for multiplicative group operations.
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Multiplicative;







/// A trait representing a group element, it can be finite or cost.
pub trait GroupElement: Clone + PartialEq + Eq + Hash {
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


/// A generic group struct holding elements of type T
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
                if !subgroup.elements.contains(&conjugate) {
                    return false;
                }
            }
        }
        true
    }


    /// generate a normal subgroup given Vec<T>, it'll return error if the generete subgroup is equal to the whole group, 
    /// or the subgroup is not a normal subgroup in the whole group
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
                let product = self.operate(&g, &h); // Assuming self.op performs the group operation

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

        let subgroup = FiniteGroup::new(subgroup_elements);

        if !self.is_normal(&subgroup) {
            log::error!("The generated subgroup is not normal in whole group");
            return Err(GroupError::NotAbelian)?
        }




        // 4. Return the new group from the final set of elements
        Ok(subgroup)

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

// Don't forget to add this boilerplate impl for Eq
impl<T: GroupElement> Eq for FiniteGroup<T> {}


impl<T: GroupElement + Ord> Hash for FiniteGroup<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // To create a consistent hash, we must sort the elements first.
        // This requires T to implement the `Ord` trait.
        let mut sorted_elements = self.elements.clone();
        sorted_elements.sort();
        sorted_elements.hash(state);
    }
}


#[derive(Debug, PartialEq)]
pub enum GroupError {
    NotClosed,
    NotAbelian, // this is for abelian group
    NotSubgroup,
    
    // some operation error
}

impl fmt::Display for GroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupError::NotClosed => write!(f, "This group is not closed"),
            GroupError::NotAbelian => write!(f, "This group is not abelian"),
            GroupError::NotSubgroup => write!(f, "The generated subgroup equal to whole group"),
            
        }
    }
}

impl Error for GroupError {}

/// a collection of group generators
/// This struct is used to generate groups of different types
pub struct GroupGenerators;

impl GroupGenerators {
    /// Generates a modulo group with additive operation
    pub fn generate_modulo_group_add(n: usize) -> Result<FiniteGroup<modulo::Modulo<Additive>>, AbsaglError> {
        // This function can be used to generate a modulo group
        // Example: Modulo::generate_group(3);
        // You can implement this in the modulo module
        let elements = modulo::Modulo::<Additive>::generate_group(n as u64)?;
        Ok(FiniteGroup::new(elements))
    }
    /// Generates a modulo group with Multiplicative operation
    pub fn generate_modulo_group_mul(n: usize) -> Result<FiniteGroup<modulo::Modulo<Multiplicative>>, AbsaglError> {
        // This function can be used to generate a modulo group
        // Example: Modulo::generate_group(3);
        // You can implement this in the modulo module
        let elements = modulo::Modulo::<Multiplicative>::generate_group(n as u64)?;
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
    pub fn generate_dihedral_group(n: usize) -> Result<FiniteGroup<dihedral::DihedralElement>, AbsaglError> {
        // This function can be used to generate a dihedral group
        // Example: DihedralElement::generate(3);
        // You can implement this in the dihedral module
        let elements = dihedral::DihedralElement::generate_group(n)?;
        Ok(FiniteGroup::new(elements))
        
    }
}




#[cfg(test)]
mod tests {

    // Import the necessary modules and traits
    use super::*;
    use crate::groups::modulo::Modulo;
    use crate::groups::permutation::Permutation;

    #[test]
    fn test_is_closed_true() {
        let a = Modulo::<Additive>::new(0, 3).expect("Failed to create Modulo element");
        let b = Modulo::<Additive>::new(1, 3).expect("Failed to create Modulo element");
        let c = Modulo::<Additive>::new(2, 3).expect("Failed to create Modulo element");

        let group = FiniteGroup::new(vec![a, b, c]);

        assert!(group.is_closed());
    }

    #[test]
    fn test_is_closed_false() {
        let a = Modulo::<Additive>::new(0, 3).expect("Failed to create Modulo element");
        let b = Modulo::<Additive>::new(1, 3).expect("Failed to create Modulo element");

        let group = FiniteGroup::new(vec![a, b]);
        assert!(!group.is_closed());
    }

    #[test]
    fn test_is_abelian_true() {
        let a = Modulo::<Additive>::new(0, 3).expect("Failed to create Modulo element");
        let b = Modulo::<Additive>::new(1, 3).expect("Failed to create Modulo element");
        let c = Modulo::<Additive>::new(2, 3).expect("Failed to create Modulo element");

        let group = FiniteGroup::new(vec![a, b, c]);

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
        let g1 = Modulo::new(1,5).unwrap();

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
    fn test_generate_normal_subgroup_success() {
        let group = GroupGenerators::generate_modulo_group_add(6).unwrap();
        let g2 = Modulo::new(2,6).unwrap();

        let result = group.generate_normal_subgroup(vec![g2]);

        // println!("result is: {:?}", &result.unwrap());



        match result {
            Ok(group) => {
                let g0 = Modulo::new(0,6).unwrap();
                let g2 = Modulo::new(2,6).unwrap();
                let g4 = Modulo::new(4,6).unwrap();
                let expected = FiniteGroup::new(vec![g0,g2,g4]);
                assert_eq!(group, expected);
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
            Err(AbsaglError::Group(GroupError::NotAbelian)) => {
                // pass
            }
            _ => panic!("Expected Err(AbsaglError::Group(GroupError::NotAbelian)), but got {:?}", result),

        }
        
    }


    #[test]
    fn test_modulo_group_equal_success() {
        let a = Modulo::<Additive>::new(0, 3).expect("Failed to create Modulo element");
        let b = Modulo::<Additive>::new(1, 3).expect("Failed to create Modulo element");

        let group1 = FiniteGroup::new(vec![a, b]);
        let group2= FiniteGroup::new(vec![b, a]);
        assert_eq!(group1, group2);
    }

    #[test]
    fn test_modulo_group_equal_fail() {
        let a = Modulo::<Additive>::new(0, 3).expect("Failed to create Modulo element");
        let b = Modulo::<Additive>::new(1, 3).expect("Failed to create Modulo element");

        let group1 = FiniteGroup::new(vec![a, b]);
        let group2= FiniteGroup::new(vec![b]);
        assert_ne!(group1, group2);
    }

    #[test]
    fn test_permutation_group_equal_success() {
        let a = Permutation::from_cycles(&vec![vec![0,2], vec![1,3]], 4).unwrap();
        let b = Permutation::from_cycles(&vec![vec![1,3], vec![0,2]], 4).unwrap();

        let group1 = FiniteGroup::new(vec![a.clone(), b.clone()]);
        let group2= FiniteGroup::new(vec![b, a]);
        assert_eq!(group1, group2);
    }

     #[test]
    fn test_permutation_group_equal_fail() {
        let a = Permutation::from_cycles(&vec![vec![0,2], vec![1,3]], 4).unwrap();
        let b = Permutation::from_cycles(&vec![vec![1,3], vec![0,2]], 4).unwrap();

        let group1 = FiniteGroup::new(vec![a.clone(), b.clone()]);
        let group2= FiniteGroup::new(vec![b]);
        assert_ne!(group1, group2);
    }

}