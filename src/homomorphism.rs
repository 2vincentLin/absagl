use std::marker::PhantomData;
use std::collections::HashSet;
use std::fmt;
use crate::{error::AbsaglError, groups::{CheckedOp, FiniteGroup, Group, GroupElement}};


/// Defines errors that can occur when creating a homomorphism.
#[derive(Debug, PartialEq, Eq)]
pub enum HomomorphismError {
    /// The provided mapping does not satisfy the homomorphism property
    /// f(a * b) = f(a) * f(b) for some a, b.
    PropertyNotHeld,
}

impl fmt::Display for HomomorphismError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HomomorphismError::PropertyNotHeld => write!(f, "The mapping does not satisfy the homomorphism property"),
        }
    }
}

impl std::error::Error for HomomorphismError {}



/// Represents a homomorphism `f: G -> H` between two groups.
/// The mapping logic is provided by a closure of type F.
pub struct Homomorphism<G, H, F>
where
    G: GroupElement,
    H: GroupElement,
    F: Fn(&G) -> H,
{
    mapping: F,
    // The description is optional, describe what the mapping is.
    description: Option<String>,
    // Using PhantomData to mark that this struct "acts on" G and H,
    // which is good practice for generic structs.
    _source_marker: PhantomData<G>,
    _target_marker: PhantomData<H>,
}

// We manually implement Debug for Homomorphism because the contained
// closure `F` does not implement the Debug trait itself. This allows
// a Homomorphism to be debug-printed (e.g., inside other structs or
// during testing), which is crucial for developer experience.
impl<G, H, F> fmt::Debug for Homomorphism<G, H, F>
where
    G: GroupElement,
    H: GroupElement,
    F: Fn(&G) -> H,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Choose the string to display based on whether a description exists.
        let mapping_display = match &self.description {
            Some(desc) => desc.as_str(),
            None => "<closure>",
        };

        f.debug_struct("Homomorphism")
         .field("mapping", &mapping_display)
         .finish()
    }
}

// Implementation block for the Homomorphism struct
impl<G, H, F> Homomorphism<G, H, F>
where
    G: GroupElement,
    H: GroupElement,
    F: Fn(&G) -> H,
{
    /// Creates a new homomorphism from a mapping function.
    ///
    /// This is an "unchecked" constructor. It assumes the user has provided
    /// a function that correctly satisfies the homomorphism property.
    /// # Arguments
    /// * `mapping_fn`: A closure that maps elements of the source group G to the target group H.
    /// * `description`: An optional description of the homomorphism.
    /// # Returns
    /// A new `Homomorphism` instance.
    pub fn new(mapping_fn: F, description: Option<String>) -> Self {
        Self {
            mapping: mapping_fn,
            description: description,
            _source_marker: PhantomData,
            _target_marker: PhantomData,
        }
    }

    /// Applies the homomorphism to an element of the source group.
    pub fn apply(&self, g: &G) -> H {
        // This just calls the stored closure.
        (self.mapping)(g)
    }

    /// Attempts to create a new homomorphism by verifying the property
    /// `f(a * b) = f(a) * f(b)` for all elements in the source group.
    ///
    /// This is the "safe" constructor. It requires a reference to the source
    /// group to perform the check if the mapping is valid.
    ///
    /// # Arguments
    /// * `source_group`: The finite group from which elements `a` and `b` are drawn.
    /// * `mapping_fn`: The closure representing the potential homomorphism.
    ///
    /// # Returns
    /// A `Result` containing the `Homomorphism` on success, or a
    /// `HomomorphismError` on failure.
    pub fn try_new(
        source_group: &FiniteGroup<G>,
        mapping_fn: F,
        description: Option<String>,
    ) -> Result<Self, AbsaglError> 
    where
        G: GroupElement + CheckedOp, // G must have a fallible operation
        H: GroupElement + CheckedOp, // H must also have a fallible operation
        F: Fn(&G) -> H,
        AbsaglError: From<G::Error> + From<H::Error> // Crucial for `?`
    {
        // Iterate through all pairs of elements (a, b) from the source group.
        for a in source_group.elements() {
            for b in source_group.elements() {
                // Calculate f(a * b)
                let f_of_op = mapping_fn(&a.checked_op(b)?);

                // Calculate f(a) * f(b)
                let op_of_f = mapping_fn(a).checked_op(&mapping_fn(b))?;

                // If they are not equal, the property is violated.
                if f_of_op != op_of_f {
                    return Err(HomomorphismError::PropertyNotHeld)?;
                }
            }
        }

        // If the loop completes, the property holds for all elements.
        Ok(Self::new(mapping_fn, description))
    }

    /// Computes the kernel of the homomorphism.
    /// The kernel is the set {g in G | f(g) = id_H}.
    ///
    /// # Arguments
    /// * `source_group`: A reference to the source group G.
    /// * `identity_h`: The identity element of the target group H.
    /// # Returns
    /// A `Result` containing the kernel as a `FiniteGroup<G>` on success,
    pub fn kernel(&self, source_group: &FiniteGroup<G>, identity_h: &H) -> Result<FiniteGroup<G>, AbsaglError> {
        let kernel_elements: Vec<G> = source_group
            .elements()
            .iter()
            .filter(|g| self.apply(g) == *identity_h)
            .cloned()
            .collect();

        FiniteGroup::new(kernel_elements)
    }



    /// Computes the image of the homomorphism.
    /// The image is the set {f(g) | g in G}.
    ///
    /// # Arguments
    /// * `source_group`: A reference to the source group G.
    /// # Returns
    /// A `Result` containing the image as a `FiniteGroup<H>` on success,
    pub fn image(&self, source_group: &FiniteGroup<G>) -> Result<FiniteGroup<H>, AbsaglError> {
        // Use a HashSet to automatically handle duplicates
        let image_elements: HashSet<H> = source_group
            .elements()
            .iter()
            .map(|g| self.apply(g))
            .collect();

        FiniteGroup::new(image_elements.into_iter().collect())
    }


   
    /// Checks if the homomorphism is injective (a monomorphism).
    ///
    /// A homomorphism is injective if every distinct element in the source group
    /// maps to a distinct element in the target group. This implementation
    /// checks this directly and is more robust than a kernel-based check
    /// that relies on user-provided identity elements.
    ///
    /// # Arguments
    /// * `source_group`: A reference to the source group G.
    pub fn is_injective(&self, source_group: &FiniteGroup<G>) -> bool {
        let mut seen_images = HashSet::with_capacity(source_group.order());
        for g in source_group.elements() {
            // The `.insert()` method on a HashSet returns `false` if the
            // element was already present in the set.
            if !seen_images.insert(self.apply(g)) {
                // We found a collision! Two different elements from G have
                // mapped to the same element in H. Therefore, the function
                // is not injective, and we can stop early.
                return false;
            }
        }
        // If we successfully insert an image for every element from G without
        // any collisions, the function is injective.
        true
    }


    /// Checks if the homomorphism is surjective (an epimorphism).
    ///
    /// A homomorphism is surjective if its image is the entire target group.
    /// This implementation first performs a fast check on the order of the groups
    /// and then proceeds to a rigorous set comparison if the orders match.
    ///
    /// # Arguments
    /// * `source_group`: A reference to the source group G.
    /// * `target_group`: A reference to the target group H.
    pub fn is_surjective(&self, source_group: &FiniteGroup<G>, target_group: &FiniteGroup<H>) -> Result<bool, AbsaglError> {
        // 1. Compute the image of the homomorphism.
        let image = self.image(source_group)?;

        // 2. Fast Check (Optimization): If the sizes don't match, it's
        //    impossible for them to be the same set. Return false immediately.
        if image.order() != target_group.order() {
            return Ok(false)
        }

        // 3. Rigorous Check: The orders are equal, but the sets might still
        //    be different. We must compare the elements.
        let image_set: HashSet<_> = image.elements().iter().cloned().collect();
        let target_set: HashSet<_> = target_group.elements().iter().cloned().collect();

        Ok(image_set == target_set)
    }


}





#[cfg(test)]

mod test_homomorphism {
    use super::*;
    use crate::groups::{modulo::Modulo, Additive, GroupGenerators, Multiplicative};

    #[test]
    fn test_homomorphism_apply_modulo() {
        // Define a simple homomorphism from Z_6 to Z_2
        let valid_mapping = |m: &Modulo<Additive>| Modulo::<Additive>::try_new(m.value() % 2, 2).unwrap();
        let hom = Homomorphism::new(valid_mapping, None);

        let g = Modulo::<Additive>::try_new(3, 6).unwrap();
        let result = hom.apply(&g);
        println!("Result of applying homomorphism: {:?}", result);
        assert_eq!(result.value(), 1);

        let g = Modulo::<Additive>::try_new(4, 6).unwrap();
        let result = hom.apply(&g);
        println!("Result of applying homomorphism: {:?}", result);
        assert_eq!(result.value(), 0);
    }

    #[test]
    fn test_homomorphism_try_new_success() {
        let valid_mapping = |m: &Modulo<Additive>| Modulo::<Additive>::try_new(m.value() % 2, 2).unwrap();
        let z4 = GroupGenerators::generate_modulo_group_add(4).unwrap();
        let hom = Homomorphism::try_new(&z4, valid_mapping, None);

        assert!(hom.is_ok(), "Homomorphism should be valid");

   
    }

    #[test]
    fn test_homomorphism_try_new_failure() {
        // 1. Setup the groups
        let z7_mul = GroupGenerators::generate_modulo_group_mul(7).unwrap();
        
        // 2. Define the invalid mapping function
        let invalid_mapping = |m: &Modulo<Multiplicative>| {
            let val = m.value();
            let new_val = if val <= 3 { 1 } else { 2 };
            // The target group is Z_3 (mul)
            Modulo::<Multiplicative>::try_new(new_val, 3).unwrap()
        };

        // 3. Call try_new and assert that it fails
        let result = Homomorphism::try_new(&z7_mul, invalid_mapping, None);

        // 4. Check for the specific error
        match result {
            Err(AbsaglError::Homomorphism(HomomorphismError::PropertyNotHeld)) => {
                // pass
            },
            _ => panic!("Expected a PropertyNotHeld error, but got {:?}", result),
        }
    }

    #[test]
    fn test_homomorphism_kernel() {
        let valid_mapping = |m: &Modulo<Additive>| Modulo::<Additive>::try_new(m.value() % 2, 2).unwrap();
        let z6 = GroupGenerators::generate_modulo_group_add(6).unwrap();
        let hom = Homomorphism::new(valid_mapping, None);

        let identity_h = Modulo::<Additive>::try_new(0, 2).unwrap();

        let kernel = hom.kernel(&z6, &identity_h).unwrap();
        eprintln!("Kernel elements: {:?}", kernel.elements());
        assert_eq!(kernel.order(), 3, "Kernel should have order 3");
    }

    #[test]
    fn test_homomorphism_image() {
        let valid_mapping = |m: &Modulo<Additive>| Modulo::<Additive>::try_new(m.value() % 2, 2).unwrap();
        let z6 = GroupGenerators::generate_modulo_group_add(6).unwrap();
        let hom = Homomorphism::new(valid_mapping, None);

        let image = hom.image(&z6).unwrap();
        eprintln!("Image elements: {:?}", image.elements());
        assert_eq!(image.order(), 2, "Image should have order 2");
    }

    #[test]
    fn test_homomorphism_is_injective_success() {
        // trivial case, identity homomorphism
        let valid_mapping = |m: &Modulo<Additive>| Modulo::<Additive>::try_new(m.value() , 6).unwrap();
        let z6 = GroupGenerators::generate_modulo_group_add(6).unwrap();
        let hom = Homomorphism::new(valid_mapping, None);

        assert!(hom.is_injective(&z6), "Homomorphism should be injective");
    }

    #[test]
    fn test_homomorphism_is_injective_fail() {
        let valid_mapping = |m: &Modulo<Additive>| Modulo::<Additive>::try_new(m.value() % 2 , 2).unwrap();
        let z6 = GroupGenerators::generate_modulo_group_add(6).unwrap();
        let hom = Homomorphism::new(valid_mapping, None);

        assert!(!hom.is_injective(&z6), "Homomorphism should not be injective");
    }

    #[test]
    fn test_homomorphism_is_surjective_success() {
        let valid_mapping = |m: &Modulo<Additive>| Modulo::<Additive>::try_new(m.value() % 2, 2).unwrap();
        let z6 = GroupGenerators::generate_modulo_group_add(6).unwrap();
        let z2 = GroupGenerators::generate_modulo_group_add(2).unwrap();
        let hom = Homomorphism::new(valid_mapping, None);

        assert!(hom.is_surjective(&z6, &z2).unwrap(), "Homomorphism should be surjective");
    }

    #[test]
    fn test_homomorphism_is_surjective_fail() {
        let valid_mapping = |m: &Modulo<Additive>| Modulo::<Additive>::try_new(m.value() % 3, 3).unwrap();
        let z6 = GroupGenerators::generate_modulo_group_add(6).unwrap();
        let z2 = GroupGenerators::generate_modulo_group_add(2).unwrap();
        let hom = Homomorphism::new(valid_mapping, None);

        assert!(!hom.is_surjective(&z6, &z2).unwrap(), "Homomorphism should not be surjective");
    }






}