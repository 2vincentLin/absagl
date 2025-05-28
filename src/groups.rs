pub mod modulo;
pub mod permutation;
pub mod dihedral;


pub trait GroupElement: Clone + PartialEq {
    type Error;
    /// The group operation (usually denoted as *)
    fn op(&self, other: &Self) -> Self;
    /// The identity element, define as a static method (associated function do not take `self`)
    fn identity() -> Self;
    /// The inverse of the element
    fn inverse(&self) -> Self;

    fn safe_op(&self, other: &Self) -> Result<Self, Self::Error>
        where
            Self: Sized;
    
}

// A generic group struct holding elements of type T
pub struct Group<T: GroupElement> {
    pub elements: Vec<T>,
}

impl<T: GroupElement> Group<T> {
    /// Creates a new group with the given elements
    pub fn new(elements: Vec<T>) -> Self {
        Group { elements }
    }

    /// Applies the group operation to two elements
    pub fn operate(&self, a: &T, b: &T) -> T {
        a.op(b)
    }

    // Returns the identity element of the group
    pub fn identity(&self) -> T {
        T::identity()
    }

    /// Returns the inverse of an element
    pub fn inverse(&self, element: &T) -> T {
        element.inverse()
    }

    pub fn order(&self) -> usize {
        // todo: return the order of the group
        self.elements.len()
        
    }

    pub fn is_closed(&self) -> bool {
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
}


#[derive(Debug)]
pub enum GroupError {
    // some operation error
}


pub struct GroupGenerators;

impl GroupGenerators {
    // call fn from each group module
    pub fn generate_modulo_group(n: usize) -> Group<modulo::Modulo> {
        // This function can be used to generate a modulo group
        // Example: Modulo::generate_group(3);
        // You can implement this in the modulo module
        let elements = modulo::Modulo::generate_group(n as u32);
        Group::new(elements)
    }
}