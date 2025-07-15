
use absagl::groups::permutation::Permutation;
use absagl::groups::permutation::SparsePerm;
use absagl::groups::GroupElement;

///
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 
/// 


// add test hash to each test mod
// use std::collections::hash_map::DefaultHasher;
// use std::hash::{Hash, Hasher};

// // Helper function to calculate the hash of any hashable type.
// fn calculate_hash<T: Hash>(t: &T) -> u64 {
//     let mut s = DefaultHasher::new();
//     t.hash(&mut s);
//     s.finish()
// }
// use std::collections::HashSet;

// #[test]
// fn test_coset_in_hashset() {
//     // 1. Setup: Same as before.
//     let g = dihedral_group(3);
//     let n = g.get_subgroup_by_name("A3");
//     let rep1 = g.get_element_by_name("f0");
//     let coset1 = Coset { representative: rep1, ... };
//     let rep2 = g.get_element_by_name("f1");
//     let coset2 = Coset { representative: rep2, ... };

//     // Sanity check that they are indeed equal.
//     assert_eq!(coset1, coset2);

//     // 2. Use a HashSet.
//     let mut set = HashSet::new();
//     set.insert(coset1);

//     // 3. Check for the second coset and the set's length.
//     //    If hash/eq were wrong, contains() might be false and len() might become 2.
//     assert!(set.contains(&coset2), "Set should contain the equivalent coset");
//     assert_eq!(set.len(), 1, "Set should only contain one unique coset");
// }

#[cfg(test)]
mod test_sparse_permutation {
    use super::*;

    #[test]
    fn test_sparse_permutation_op() {
        let a = SparsePerm {
            mapping: vec![(0, 1), (1, 2), (2, 0)].into_iter().collect(),
        };
        let b = SparsePerm {
            mapping: vec![(1, 2), (2, 0), (0, 1)].into_iter().collect(),
        };
        let c = a.op(&b);

        assert_eq!(c.mapping.get(&0), Some(&2));
        assert_eq!(c.mapping.get(&1), Some(&0));
        assert_eq!(c.mapping.get(&2), Some(&1));
    }

    #[test]
    fn test_sparse_permutation_identity() {
        let a = SparsePerm {
            mapping: vec![(0, 1), (1, 2), (2, 0)].into_iter().collect(),
        };
        let identity = SparsePerm::identity();
        let b = a.op(&identity);

        assert_eq!(b.mapping, a.mapping);
    }

    #[test]
    fn test_sparse_permutation_inverse() {
        let a = SparsePerm {
            mapping: vec![(0, 1), (1, 2), (2, 0)].into_iter().collect(),
        };
        let inverse = a.inverse();
        let b = a.op(&inverse);
        println!("Inverse mapping: {:?}", inverse.mapping);
        println!("Result of a op inverse: {:?}", b.mapping);
        // The result should be the identity permutation
        let mut identitical = true;
        for i in 0..b.mapping.len() {
            if b.mapping.get(&(i as usize)) != Some(&(i as usize)) {
                println!("Expected {} but got {:?}", i, b.mapping.get(&(i as usize)));
                identitical = false;
            }
        }
        assert_eq!(identitical, true);
    }
}