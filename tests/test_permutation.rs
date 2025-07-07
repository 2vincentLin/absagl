
use absagl::groups::permutation::Permutation;
use absagl::groups::permutation::SparsePerm;
use absagl::groups::GroupElement;



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