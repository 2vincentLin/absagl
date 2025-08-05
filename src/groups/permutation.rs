use crate::groups::{CanonicalRepr, CheckedOp, GroupElement};
use crate::utils;
use crate::error::AbsaglError;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::ops::Mul;
use std::ops::Deref;
use std::error::Error;
use std::hash::{Hash, Hasher};


#[derive(Debug)]
pub enum PermutationError {
    SizeNotMatch,
    CycleIndexOutOfBounds,
    OrderIsTooLarge,
    NonDisjointCycles,
    NotEvenPermutation,
    // Add more as needed
}

impl fmt::Display for PermutationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PermutationError::SizeNotMatch => write!(f, "Size mismatch error"),
            PermutationError::CycleIndexOutOfBounds => write!(f, "Cycle index out of bounds"),
            PermutationError::OrderIsTooLarge => write!(f, "Order is too large for heap algorithm"),
            PermutationError::NonDisjointCycles => write!(f, "Non-disjoint cycles in permutation mapping"),
            PermutationError::NotEvenPermutation => write!(f, "Not an even permutation"),
        }
    }
}

impl Error for PermutationError {}


// region: implement permutation group using Vec (standard way in many computational group theory libraries)
/// note that unlike math symbol, in Vector representation, if we see Vector `[1, 2, 0]`,
/// it means 0 -> 1, 1 -> 2, 2 -> 0, it means index 0 map to 1, index 1 map to 2, index 2 map to 0
/// 

/// A standard way to represent permutation in many computational group theory libraries
/// it is a vector of indices, where the value at each index represents the image of that
#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct Permutation {
    mapping: Vec<usize>,
}

impl GroupElement for Permutation {
    // type Error = PermutationError;
    /// Perform the operation of two permutations
    /// this is not safe, it will panic if the sizes of the two permutations are not equal
    fn op(&self, other: &Self) -> Self {
        assert_eq!(self.mapping.len(), other.mapping.len(), "permutation op fail");
        let mapping = other.mapping.iter().map(|&i| self.mapping[i]).collect();
        Permutation { mapping }
    }
    
    /// Inverse of a permutation, which is the permutation that undoes the effect of the original permutation
    /// it simply swap the index and value in the mapping
    /// for example, if the mapping is [2, 0, 1], the inverse will be [1, 2, 0]
    fn inverse(&self) -> Self {
        let mut inv = vec![0; self.mapping.len()];
        for (i, &v) in self.mapping.iter().enumerate() {
            inv[v] = i;
        }
        Permutation { mapping: inv }
    }
    
}

impl CheckedOp for Permutation {
    type Error = PermutationError;

    /// A fallible version of the group operation.
    fn checked_op(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.mapping.len() != other.mapping.len() {
            log::error!("Size mismatch: {} != {}", self.mapping.len(), other.mapping.len());
            Err(PermutationError::SizeNotMatch)
        } else {
            Ok(self.op(other))
        }
    }
}

impl Permutation {

    /// Create a new permutation given a mapping, this will not check if the mapping is valid
    pub fn new(mapping: Vec<usize>) -> Self {
        Permutation { mapping }
    }
    
    /// Create a new permutation given a mapping, this will check if the mapping is valid
    pub fn try_new(mapping: Vec<usize>) -> Result<Self, AbsaglError> {
        if !utils::is_mapping_valid(&mapping) {
            log::error!("Invalid mapping: {:?}", mapping);
            return Err(PermutationError::NonDisjointCycles)?;
        }
        Ok(Permutation { mapping })
    }

    /// Get the mapping of the permutation
    pub fn mapping(&self) -> &Vec<usize> {
        &self.mapping
    }

    /// Return identity for the permutation 
    pub fn identity(size: usize) -> Self {
        Permutation { mapping: (0..size).collect() }
    }

    /// use cycle decomposition to determine if the permutation is even or odd
    /// in abstract algebra, a permutation is even if it can be expressed as a product of an even number of transpositions
    /// and we can break down k-length cycle into k-1 transpositions
    /// for example, (1,2,3) can be expressed as (1,3)(1,2), which is 2 transpositions, thus it is even
    pub fn is_even(&self) -> bool {
        let mut visited = vec![false; self.mapping.len()];
        let mut parity = 0;
        for i in 0..self.mapping.len() {
            if visited[i] || self.mapping[i] == i {
                continue;
            }
            let mut j = i;
            let mut cycle_len = 0;
            while !visited[j] {
                visited[j] = true;
                j = self.mapping[j];
                cycle_len += 1;
            }
            // Each cycle of length k contributes (k-1) transpositions
            parity += cycle_len - 1;
        }
        parity % 2 == 0
    }

    /// Construct a permutation from a list of cycles
    /// so you can pass cycles like (0,2,4) 0-based cycle to create a permutation
    /// it'll generate a mapping like `[2, 1, 4, 3, 0]` for size 5
    /// 
    /// ```rust
    /// # use absagl::groups::permutation::Permutation; // import the Permutation struct
    /// let cycles = vec![vec![0, 2, 4]];
    /// let size = 5;
    /// let perm = Permutation::from_cycles(&cycles, size).expect("Should construct permutation");
    ///
    /// // This is the crucial part: we assert that the output is correct.
    /// // If they are not equal, the test will panic and fail.
    /// let expected = vec![2, 1, 4, 3, 0];
    /// assert_eq!(perm.mapping(), &expected); // 
    /// ```
    pub fn from_cycles(cycles: &[Vec<usize>], n: usize) -> Result<Self, AbsaglError> {
        // Check for out-of-bounds indices
        for cycle in cycles {
            for &idx in cycle {
                if idx >= n {
                    log::error!("Cycle index {} is out of bounds for size {}", idx, n);
                    return Err(PermutationError::CycleIndexOutOfBounds)?;
                }
            }
        }

        let mut mapping: Vec<usize> = (0..n).collect();

        for cycle in cycles {
            if cycle.len() < 2 { continue; }
            for i in 0..cycle.len() {
                let from = cycle[i]; 
                let to = cycle[(i + 1) % cycle.len()]; 
                mapping[from] = to;
            }
        }

        if utils::is_mapping_valid(&mapping) {
            log::debug!("Permutation mapping is valid: {:?}", mapping);
            Ok(Permutation { mapping })
        } else {
            log::error!("Permutation mapping is not valid: {:?}", mapping);
            Err(PermutationError::NonDisjointCycles)?
        }


    }

    /// Calculates the order of the permutation.
    /// The order is the smallest positive integer k such that p^k is the identity.
    pub fn order(&self) -> usize {
        let mut visited = vec![false; self.mapping.len()];
        let mut overall_lcm = 1;

        for i in 0..self.mapping.len() {
            if visited[i] {
                continue;
            }

            // We've found the start of a new cycle. Let's find its length.
            let mut cycle_len = 0;
            let mut j = i;
            while !visited[j] {
                visited[j] = true;
                j = self.mapping[j];
                cycle_len += 1;
            }

            // Update the overall LCM with the new cycle's length.
            if cycle_len > 0 {
                overall_lcm = utils::lcm(overall_lcm, cycle_len);
            }
        }
        overall_lcm
    }

    /// Raises the permutation to the power of `exp` using exponentiation by squaring.
    /// this algorithm is efficient for computing powers of permutations because for any permutation p,
    /// we can express p^n as a series of squarings and multiplications.
    /// because for any n, we can express it as a sum of powers of 2, e.g.:
    /// 13 = 8 + 4 + 1, so we can compute it as p^8 * p^4 * p^1
    ///
    /// # Arguments
    /// * `exp` - The non-negative integer exponent.
    pub fn pow(&self, mut exp: u32) -> Self {
        // Start with the identity element for the group.
        let mut res = Permutation::identity(self.mapping.len());
        
        // If the exponent is 0, the result is the identity.
        if exp == 0 {
            return res;
        }

        let mut base = self.clone();

        while exp > 0 {
            // If the exponent is odd, multiply the result by the current base.
            if exp % 2 == 1 {
                res = res.op(&base);
            }
            // Square the base for the next iteration.
            base = base.op(&base);
            // Halve the exponent. when n is odd, it will be rounded down.
            exp /= 2;
        }
        res
    }

    /// using heap algorithm to generate permutation, only used for small order
    /// heap algorithm relies on stack to operate properly, thus cannot be parallelize
    pub fn generate_group_heap(n: usize) -> Result<Vec<Self>, AbsaglError> {
        
        // when n = 12, it'll take around 46 GB memory
        if n > 9 {
            log::error!("Order {} is too large for heap algorithm, maximum is 9, use generate_group instead", n);
            return Err(PermutationError::OrderIsTooLarge)?;
        }

        /// Recursive function to generate permutations using Heap's algorithm.
        /// k is the size of the current permutation being generated
        fn heap_recursive(k: usize, arr: &mut Vec<usize>, output: &mut Vec<Vec<usize>>) {
            if k == 1 {
                output.push(arr.clone());
            } else {
                // Recursively generate permutations for the first k-1 elements
                heap_recursive(k - 1, arr, output);
                for i in 0..(k - 1) {
                    // If k is even, swap the i-th element with the last element (k-1)
                    if k % 2 == 0 {
                        arr.swap(i, k - 1);
                    // If k is odd, swap the 0-th element with the last element (k-1)
                    } else {
                        arr.swap(0, k - 1);
                    }
                    heap_recursive(k - 1, arr, output);
                }
            }
        }

        let mut arr: Vec<usize> = (0..n).collect();
        let mut result = vec![];
        heap_recursive(n, &mut arr, &mut result);

        Ok(result.into_iter().map(|mapping| Permutation { mapping }).collect())
    }


    /// Generates all elements of the symmetric group S_n based on the mathematical theory of symmetric groups.
    /// where we use the fact that S_n can be generated by two permutations:
    /// 1. A simple transposition (e.g., (0 1))
    /// 2. A long cycle (e.g., (0 1 ... n-1)).
    /// 
    /// It uses a breadth-first search to generate all elements of the group.
    pub fn generate_group(n: usize) -> Result<Vec<Self>, AbsaglError> {
        
        if n == 0 {
            return Ok(vec![]);
        }
        if n == 1 {
            return Ok(vec![Permutation::identity(1)]);
        }

        // S_n for n >= 2 can be generated by two permutations:
        // a simple transposition (e.g., (0 1))
        let transposition = Permutation::from_cycles(&[vec![0, 1]], n)?;
        // and a long cycle (e.g., (0 1 ... n-1))
        let long_cycle = Permutation::from_cycles(&[(0..n).collect()], n)?;

        let generators = vec![transposition, long_cycle];

        let mut elements = HashSet::new();
        let mut queue = VecDeque::new();

        let identity = Permutation::identity(n);
        elements.insert(identity.clone());
        queue.push_back(identity);

        while let Some(current) = queue.pop_front() {
            for g in &generators {
                let new_element = current.op(g);
                if elements.insert(new_element.clone()) {
                    queue.push_back(new_element);
                }
            }
        }

        Ok(elements.into_iter().collect())
    }

    /// Generates a subgroup from a given set of generators.
    /// This is more efficient than generating the whole symmetric group and then finding the subgroup.
    /// It uses a breadth-first search to explore the subgroup generated by the given generators.
    /// # Arguments
    /// * `generators`: A slice of Permutation elements that generate the subgroup.
    /// # Returns
    /// A vector of Permutation elements that form the generated subgroup.
    pub fn generate_subgroup(generators: &[Self]) -> Result<Vec<Self>, AbsaglError> {
        if generators.is_empty() {
            return Ok(vec![]);
        }

        let n = generators[0].mapping.len();
        for g in generators.iter().skip(1) {
            if g.mapping.len() != n {
                log::error!("Generators must have the same size.");
                return Err(PermutationError::SizeNotMatch)?;
            }
        }

        let mut elements = std::collections::HashSet::new();
        let mut to_process = generators.to_vec();

        for g in generators {
            elements.insert(g.clone());
        }

        while let Some(p) = to_process.pop() {
            for g in generators {
                let new_element = p.op(g);
                if elements.insert(new_element.clone()) {
                    to_process.push(new_element);
                }
            }
        }

        Ok(elements.into_iter().collect())
    }

    /// Generates all elements of the alternating group A_n.
    pub fn generate_alternative_group(n: usize) -> Result<Vec<Self>, AbsaglError> {
        // 1. Generate the full symmetric group S_n.
        let all_permutations = Permutation::generate_group(n)?;

        // 2. Filter for even permutations and wrap them.
        let even_permutations = all_permutations
            .into_iter()
            // Keep only the permutations that are even.
            .filter(|p| p.is_even())
            // Collect the results into a new vector.
            .collect();
        
        Ok(even_permutations)
    }

}

impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut visited = vec![false; self.mapping.len()];
        let mut cycles = vec![];

        for i in 0..self.mapping.len() {
            if visited[i] || self.mapping[i] == i {
                continue;
            }
            let mut cycle = vec![i];
            visited[i] = true;
            let mut j = self.mapping[i];
            while j != i {
                cycle.push(j);
                visited[j] = true;
                j = self.mapping[j];
            }
            cycles.push(cycle);
        }

        // note that for identity permutation, cycles will be empty
        if cycles.is_empty() {
            write!(f, "(e)")
        } else {
            for cycle in cycles {
                write!(f, "(")?;
                // Use a peekable iterator to handle spacing correctly
                let mut iter = cycle.iter().peekable();
                while let Some(i) = iter.next() {
                    write!(f, "{}", i)?; // 0-based
                    // If there is a next element, print a space
                    if iter.peek().is_some() {
                        write!(f, " ")?;
                    }
                }
                
                write!(f, ") ")?; // Space between cycles, e.g., (1 2) (3 4)
                }
            Ok(())
        }
    }
}

/// overload the multiplication operator for Permutation
impl Mul for Permutation {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.op(&other)
    }
}

// overload Mul for for borrowed Permutation to avoid consuming the permutations.
impl<'a, 'b> Mul<&'b Permutation> for &'a Permutation {
    type Output = Permutation;

    fn mul(self, rhs: &'b Permutation) -> Self::Output {
        self.op(rhs)
    }
}


impl CanonicalRepr for Permutation {
    fn to_canonical_bytes(&self) -> Vec<u8> {
        
        self.mapping
            .iter()
            .flat_map(|&x| x.to_be_bytes())
            .collect()
    }
}
// todo: remove altenative group element
/// Create an Alternating Group Element from a Permutation
/// An alternating group is a subgroup of the symmetric group consisting of all even permutations.
#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct AlternatingGroupElement {
    pub permutation: Permutation,
}

impl GroupElement for AlternatingGroupElement {

    /// Perform the operation of two alternating group elements
    /// this is not safe, it will panic if the sizes of the two permutations are not equal
    /// by abstract algebra, the result of two even permutations is also an even permutation
    fn op(&self, other: &Self) -> Self {
        AlternatingGroupElement {
            permutation: self.permutation.op(&other.permutation),
        }
    }

    /// Inverse of an alternating group element, which is the permutation that undoes the effect of the original permutation
    fn inverse(&self) -> Self {
        AlternatingGroupElement {
            permutation: self.permutation.inverse(),
        }
    }

    
}

impl CheckedOp for AlternatingGroupElement {
    type Error = PermutationError;

    /// A fallible version of the group operation.
    fn checked_op(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.permutation.mapping.len() != other.permutation.mapping.len() {
            log::error!("Size mismatch: {} != {}", self.permutation.mapping.len(), other.permutation.mapping.len());
            Err(PermutationError::SizeNotMatch)
        } else {
            Ok(self.op(other))
        }
    }
}


impl AlternatingGroupElement {

   
    pub fn new(p: Permutation) -> Result<Self, AbsaglError> {
        if p.is_even() {
            Ok(AlternatingGroupElement { permutation: p })
        } else {
            log::error!("Cannot create AlternatingGroupElement from odd permutation: {:?}", p);
            Err(PermutationError::NotEvenPermutation)?
        }
    }

    /// Return identity element for Permutation(n)
    pub fn identity(size: usize) -> Self {
        AlternatingGroupElement {
            permutation: Permutation::identity(size),
        }
    }

    /// Generates all elements of the alternating group A_n.
    pub fn generate_group(n: usize) -> Result<Vec<Self>, AbsaglError> {
        // 1. Generate the full symmetric group S_n.
        let all_permutations = Permutation::generate_group(n)?;

        // 2. Filter for even permutations and wrap them.
        let even_permutations = all_permutations
            .into_iter()
            // Keep only the permutations that are even.
            .filter(|p| p.is_even())
            // Wrap each valid Permutation in our AlternatingGroupElement struct.
            .map(|p| AlternatingGroupElement { permutation: p })
            // Collect the results into a new vector.
            .collect();
        
        Ok(even_permutations)
    }
}

// implement Deref for AlternatingGroupElement to allow access to some methods of Permutation directly not involves returning a new Permutation
impl Deref for AlternatingGroupElement {
    type Target = Permutation;

    fn deref(&self) -> &Self::Target {
        &self.permutation
    }
}

impl fmt::Display for AlternatingGroupElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just write the inner permutation to the formatter.
        // Since `self.permutation` already implements Display, this works perfectly.
        write!(f, "{}", self.permutation)
    }
}


// endregion

// region: implment permutation group using HashMap

/// using HashMap to represent sparse permutation
/// this is useful when the permutation is sparse, i.e. only a few elements are perm
#[derive(Clone, PartialEq, Debug, Eq)]
pub struct SparsePerm {
    pub mapping: HashMap<usize, usize>,
}

impl GroupElement for SparsePerm {
    fn op(&self, other: &Self) -> Self {
        let mut mapping = HashMap::new();
        for (&k, &v) in &self.mapping {
            mapping.insert(k, v);
        }
        for (&k, &v) in &other.mapping {
            mapping.insert(k, self.mapping.get(&v).cloned().unwrap_or(v));
        }
        SparsePerm { mapping }
    }

    

    fn inverse(&self) -> Self {
        let mut inv = HashMap::new();
        for (&k, &v) in &self.mapping {
            inv.insert(v, k);
        }
        SparsePerm { mapping: inv }
    }

    
}

impl CheckedOp for SparsePerm {
    type Error = PermutationError;

    /// A fallible version of the group operation.
    fn checked_op(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.mapping.keys().any(|k| !other.mapping.contains_key(k)) {
            Err(PermutationError::SizeNotMatch)?
        } else {
            Ok(self.op(other))
        }
    }
}

impl SparsePerm {

    
    pub fn identity() -> Self {
        SparsePerm { mapping: HashMap::new() }
    }
}

impl fmt::Display for SparsePerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut visited = HashSet::new();
        let mut cycles = vec![];

        // Get all keys and values to find full support
        let support: HashSet<usize> = self
            .mapping
            .keys()
            .chain(self.mapping.values())
            .copied()
            .collect();

        for &start in &support {
            if visited.contains(&start) {
                continue;
            }

            let mut cycle = vec![];
            let mut x = start;
            loop {
                cycle.push(x);
                visited.insert(x);

                x = *self.mapping.get(&x).unwrap_or(&x);
                if x == start {
                    break;
                }
                if visited.contains(&x) {
                    // not a cycle (shouldn’t happen if mapping is valid), break to avoid infinite loop
                    break;
                }
            }

            // Only keep cycles of length > 1 (omit fixed points)
            if cycle.len() > 1 {
                cycles.push(cycle);
            }
        }

        if cycles.is_empty() {
            write!(f, "id")
        } else {
            let mut parts = vec![];
            for cycle in cycles {
                let inner = cycle.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
                parts.push(format!("({})", inner));
            }
            write!(f, "{}", parts.join(""))
        }
    }
}


impl Hash for SparsePerm {
    /// we need to implement Hash for SparePem because we cannot derive Hash on a struct contain `HashMap`
    /// the reaons is for any two HashMap, even if they have same key and value, 
    /// it still has another RandomState, so we can't have same hash value
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 1. Collect the key-value pairs into a vector.
        let mut pairs: Vec<_> = self.mapping.iter().collect();

        // 2. Sort the vector by key to ensure a deterministic order.
        //    This is the most important step!
        pairs.sort_unstable_by_key(|&(k, _)| k);

        // 3. Now hash the sorted vector.
        pairs.hash(state);
    }
}


// endregion




#[cfg(test)]
mod test_permutaion {
    use super::*;
    

    #[test]
    fn test_permutaion_create_success() {

        let a = Permutation::try_new(vec![0,1,2]).expect("should create permutation");
        assert_eq!(a.mapping(), &vec![0,1,2])

    }

    #[test]
    fn test_permutaion_create_fail() {
        let result = Permutation::try_new(vec![0,0,2]);
        match result {
            Err(AbsaglError::Permutation(PermutationError::NonDisjointCycles)) => {
                // Test passes, this is the expected outcome
            },
            _ => panic!("Expected Err(PermutationError::NonDisjointCycles), but got {:?}", result),
        }
    }

    #[test]
    fn test_permutation_op() {
        let a = Permutation::try_new(vec![0, 1, 2, 4, 3]).expect("should create permutation");
        let b = Permutation::try_new(vec![0, 2, 1, 3, 4]).expect("should create permutation");
        
        let c = a.op(&b);
        assert_eq!(c.mapping(), &vec![0, 2, 1, 4, 3]);
    }

    #[test]
    fn test_permutation_identity() {
        let a = Permutation::try_new(vec![0,1,2,3,4]).expect("should create element");
        let identity = Permutation::identity(5) ;
        println!("Identity mapping: {:?}", identity.mapping);
        assert_eq!(identity.mapping(), a.mapping());
    }

    #[test]
    fn test_permutation_is_even() {
        let perm = Permutation::try_new(vec![1, 0, 2, 4, 3]).expect("should create element");
        assert!(perm.is_even(), "The permutation should be even");
        
        let odd_perm = Permutation::try_new(vec![1, 0, 3, 4, 2]).expect("should create element");
        assert!(!odd_perm.is_even(), "The permutation should be odd");
    }

    #[test]
    fn test_permutation_inverse() {
        let a = Permutation::try_new(vec![2, 1, 0, 4, 3]).expect("should create element");
        let inverse = a.inverse();
        let b = inverse.op(&a);
        let idenity = Permutation::identity(a.mapping.len());
        assert_eq!(b.mapping, idenity.mapping);
        
    }
    #[test]
    fn test_permutation_checked_op_size_mismatch() {

        let a = Permutation::try_new(vec![0, 1, 2, 3]).expect("should create element");

        let b = Permutation::try_new(vec![0, 2, 1, 3, 4]).expect("should create element");
        let result = a.checked_op(&b);
        match result {
            Err(PermutationError::SizeNotMatch) => {
                // Test passes, this is the expected outcome
            },
            _ => panic!("Expected Err(PermutationError::SizeNotMatch), but got {:?}", result),
        }
    }

    #[test]
    fn test_permutation_from_cycles_out_of_bounds() {
        // The cycle contains an element out of bounds for the given size
        let cycles = vec![vec![0, 5]]; // 5 is out of bounds for size 4
        let size = 4;
        let result = Permutation::from_cycles(&cycles, size);
        
        match result {
            Err(AbsaglError::Permutation(PermutationError::CycleIndexOutOfBounds)) => {
                // Test passes, this is the expected outcome
            },
            _ => panic!("Expected Err(PermutationError::CycleIndexOutOfBounds), but got {:?}", result),
        }
    }

    #[test]
    fn test_permutation_from_cycles_valid() {
        // A valid cycle for size 5
        let cycles = vec![vec![0, 2, 4]];
        let size = 5;
        let perm = Permutation::from_cycles(&cycles, size).expect("Should construct permutation");
        // 0->2, 2->4, 4->0, 1->1, 3->3
        let expected = vec![2, 1, 4, 3, 0];
        assert_eq!(perm.mapping, expected);
    }

    #[test]
    fn test_permutaion_order() {
        
        let perm = Permutation::try_new(vec![1, 0, 3, 4, 2]).expect("should create element");
        let order = perm.order();
        assert_eq!(order, 6, "The order of the permutation should be 6");
    }

    #[test]
    fn test_permutation_pow() {
        let perm = Permutation::from_cycles(&vec![vec![0,1,2,3]], 4).expect("should create element");
        let result = perm.pow(1);
        let expected = Permutation::try_new(vec![1, 2, 3, 0]).expect("should create element");
        assert_eq!(result.mapping, expected.mapping);

        let result = perm.pow(2);
        let expected = Permutation::try_new(vec![2, 3, 0, 1]).expect("should create element");
        assert_eq!(result.mapping, expected.mapping);

        let result = perm.pow(3);
        let expected = Permutation::try_new(vec![3, 0, 1, 2]).expect("should create element");
        assert_eq!(result.mapping, expected.mapping);

        let result = perm.pow(4);
        let expected = Permutation::identity(4);
        assert_eq!(result.mapping, expected.mapping);
    }

    #[test]
    fn test_permutation_generate_group_heap() {
        let group = Permutation::generate_group_heap(3).expect("should generate group");
        assert_eq!(group.len(), 6, "The group should have 6 elements for S_3");
        // Check if the identity is present
        assert!(group.iter().any(|p| p.mapping == vec![0, 1, 2]));
    }

    #[test]
    fn test_permutation_generate_group() {
        let group = Permutation::generate_group(3).expect("should generate group");
        assert_eq!(group.len(), 6, "The group should have 6 elements for S_3");
        // Check if the identity is present
        assert!(group.iter().any(|p| p.mapping == vec![0, 1, 2]));
    }

    #[test]
    fn test_permutation_generate_subgroup() {
        // Example: Generating the cube motion group using its generators
        // The cube motion group can be generated by the following generators:
        let n = 8;
        // Note: cycle notation is 1-based, so we subtract 1 for 0-based indices.
        let above_cycles = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7]];
        let front_cycles = vec![vec![0, 3, 7, 4], vec![1, 2, 6, 5]];
        let right_cycles = vec![vec![0, 1, 5, 4], vec![2, 6, 7, 3]];

        let above = Permutation::from_cycles(&above_cycles, n).unwrap();
        let front = Permutation::from_cycles(&front_cycles, n).unwrap();
        let right = Permutation::from_cycles(&right_cycles, n).unwrap();

        let generators = vec![above, front, right];

        match Permutation::generate_subgroup(&generators) {
            Ok(cube_group) => {
                println!("Generated the cube motion group with {} elements.", cube_group.len());
                // The order of the rotational symmetry group of a cube is 24.
                assert_eq!(cube_group.len(), 24); 
            }
            Err(e) => {
                eprintln!("Error generating cube group: {:?}", e);
            }
        }
    }

    #[test]
    fn test_to_canonical_bytes() {
        let a = Permutation::try_new(vec![0,1]).expect("should create permutation");
        println!("canonical form: {:?}", a.to_canonical_bytes());
        let b : Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        assert_eq!(a.to_canonical_bytes(),b);
    
    }

    #[test]
    fn test_display() {
        let a = Permutation::try_new(vec![0, 2, 1, 4, 3]).expect("should create permutation");
        assert_eq!(format!("{}", a), "(1 2) (3 4) ");
    }

    #[test]
    fn test_display_id() {
        let a = Permutation::try_new(vec![0, 1, 2, 3, 4]).expect("should create permutation");
        assert_eq!(format!("{}", a), "(e)");
    }

}



#[cfg(test)]
mod test_alternating_group_element {
    use super::*;

    #[test]
    fn test_alternating_group_element_creation_fail() {

        let perm = Permutation::try_new(vec![1, 0, 3, 4, 2]).expect("should create element");
        let result = AlternatingGroupElement::new(perm);
        match result {
            Err(AbsaglError::Permutation(PermutationError::NotEvenPermutation)) => {
                //
            },
            _ => panic!("Expected Err(AbsaglError::Permutation(PermutationError::NotEvenPermutation)), but got {:?}", result),

        }
    }

    #[test]
    fn test_alternating_group_element_creation_success() {

        let perm = Permutation::try_new(vec![1, 0, 2, 4, 3]).expect("should create element");
        let ag = AlternatingGroupElement::new(perm).expect("Should create AlternatingGroupElement");
        assert_eq!(ag.mapping().len(), 5);
    }

    


    #[test]
    fn test_alternating_group_element_op() {

        let perm1 = Permutation::try_new(vec![0, 2, 1, 4, 3]).expect("should create element");
        let perm2 = Permutation::try_new(vec![0, 2, 1, 4, 3]).expect("should create element");
        let ag1 = AlternatingGroupElement::new(perm1).expect("Should create AlternatingGroupElement");
        let ag2 = AlternatingGroupElement::new(perm2).expect("Should create AlternatingGroupElement");
        let result = ag1.op(&ag2);
        assert_eq!(result.mapping(), &vec![0, 1, 2, 3, 4].into_iter().collect::<Vec<usize>>());
    }

    #[test]
    fn test_alternating_group_element_identity() {
        let identity = AlternatingGroupElement::identity(5);
        
        assert_eq!(identity.mapping(), &vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_alternating_group_element_inverse() {
 
        let perm = Permutation::try_new(vec![1, 0, 2, 4, 3]).expect("should create element");

        let ag = AlternatingGroupElement::new(perm).expect("Should create AlternatingGroupElement");
        let inverse = ag.inverse();
        let identity = AlternatingGroupElement::identity(5);
        let result = ag.op(&inverse);
        assert_eq!(result, identity, "The result of ag op inverse should be the identity element");
    }

    #[test]
    fn test_alternating_group_element_order() {
        
        let perm = Permutation::try_new(vec![1, 0, 2, 4, 3]).expect("should create element");

        let ag = AlternatingGroupElement::new(perm).expect("Should create AlternatingGroupElement");
        let order = ag.order();
        assert_eq!(order, 2, "The order of the alternating group element should be 2");
    }

    #[test]
    fn test_to_canonical_bytes() {
        let p = Permutation::from_cycles(&vec![vec![0,1,2]],3).expect("should create permutation");
        println!("p: {}", &p);
        let a = AlternatingGroupElement::new(p).expect("fail to create altenative");
        println!("canonical form: {:?}", a.to_canonical_bytes());
        let b : Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(a.to_canonical_bytes(),b);
    
    }



    
}
