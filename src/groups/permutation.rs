use crate::groups::{GroupElement};
use crate::utils;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug)]
pub enum PermutationError {
    SizeNotMatch,
    CycleIndexOutOfBounds,
    OrderIsTooLarge,
    NonDisjointCycles,
    // Add more as needed
}


// region: implement permutation group using Vec (standard way in many computational group theory libraries)
/// note that unlike math symbol, in Vector representation, if we see Vector [1, 2, 0],
/// it means 0 -> 1, 1 -> 2, 2 -> 0, it means index 0 map to 1, index 1 map to 2, index 2 map to 0
/// 

#[derive(Clone, PartialEq, Debug)]
pub struct Permutation {
    pub mapping: Vec<usize>,
}

impl GroupElement for Permutation {
    type Error = PermutationError;
    fn op(&self, other: &Self) -> Self {
        assert_eq!(self.mapping.len(), other.mapping.len());
        let mapping = other.mapping.iter().map(|&i| self.mapping[i]).collect();
        Permutation { mapping }
    }
    fn identity() -> Self {
        eprintln!("Warning: Using identity without size may lead to unexpected behavior. Consider using Permutation::identity(size)");
        Permutation { mapping: vec![] } // You may want to pass size as parameter
    }
    fn inverse(&self) -> Self {
        let mut inv = vec![0; self.mapping.len()];
        for (i, &v) in self.mapping.iter().enumerate() {
            inv[v] = i;
        }
        Permutation { mapping: inv }
    }
    fn safe_op(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.mapping.len() != other.mapping.len() {
            Err(PermutationError::SizeNotMatch)
        } else {
            Ok(self.op(other))
        }
    }
}

impl Permutation {

    // shadow the identity function in GroupElement trait since we need to pass size
    pub fn identity(size: usize) -> Self {
        Permutation { mapping: (0..size).collect() }
    }

    // use cycle decomposition to determine if the permutation is even or odd
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

    // Construct a permutation from a list of cycles
    pub fn from_cycles(cycles: &[Vec<usize>], n: usize) -> Result<Self, PermutationError> {
        // Check for out-of-bounds indices
        for cycle in cycles {
            for &idx in cycle {
                if idx >= n {
                    return Err(PermutationError::CycleIndexOutOfBounds);
                }
            }
        }

        let mut mapping: Vec<usize> = (0..n).collect();

        for cycle in cycles {
            if cycle.len() < 2 { continue; }
            for i in 0..cycle.len() {
                let from = cycle[i]; // Convert to 0-based index
                let to = cycle[(i + 1) % cycle.len()]; // Convert to 0-based index
                mapping[from] = to;
            }
        }

        if utils::is_mapping_valid(&mapping) {
            Ok(Permutation { mapping })
        } else {
            Err(PermutationError::NonDisjointCycles)
        }


    }

    // using heap algorithm to generate permutation, only used for small order
    // heap algorithm relies on stack to operate properly, thus cannot be parallelize
    pub fn generate_group(n: usize) -> Result<Vec<Self>, PermutationError> {
        
        // when n = 12, it'll take around 46 GB memory
        if n > 11 {
            return Err(PermutationError::OrderIsTooLarge);
        }

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

        if cycles.is_empty() {
            write!(f, "()")
        } else {
            for cycle in cycles {
                write!(f, "(")?;
                for i in cycle {
                    write!(f, "{} ", i + 1)?; // math-style, 1-based
                }
                write!(f, "\x08)")?; // backspace to remove last space
            }
            Ok(())
        }
    }
}



// endregion

// region: implment permutation group using HashMap


#[derive(Clone, PartialEq, Debug)]
pub struct SparsePerm {
    pub mapping: HashMap<usize, usize>,
}

impl GroupElement for SparsePerm {
    type Error = PermutationError;
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

    fn identity() -> Self {
        SparsePerm { mapping: HashMap::new() }
    }

    fn inverse(&self) -> Self {
        let mut inv = HashMap::new();
        for (&k, &v) in &self.mapping {
            inv.insert(v, k);
        }
        SparsePerm { mapping: inv }
    }

    fn safe_op(&self, other: &Self) -> Result<Self, Self::Error> {
        if self.mapping.keys().any(|k| !other.mapping.contains_key(k)) {
            Err(PermutationError::SizeNotMatch)
        } else {
            Ok(self.op(other))
        }
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

// endregion

