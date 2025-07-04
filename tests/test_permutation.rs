
use absagl::groups::permutation::Permutation;
use absagl::groups::permutation::SparsePerm;
use absagl::groups::GroupElement;



#[cfg(test)]
mod test_permutaion {
    use super::*;

    #[test]
    fn test_permutation_op() {
        let a = Permutation {
            mapping: vec![0, 1, 2, 4, 3].into_iter().collect(),
        };
        let b = Permutation {
            mapping: vec![0, 2, 1, 3, 4].into_iter().collect(),
        };
        let c = a.op(&b);
        assert_eq!(c.mapping, vec![0, 2, 1, 4, 3].into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_permutation_identity() {
        let id_mapping = vec![0, 1, 2, 3, 4].into_iter().collect::<Vec<usize>>();
        let identity = Permutation::identity(5) ;
        println!("Identity mapping: {:?}", identity.mapping);
        assert_eq!(identity.mapping(), &id_mapping);
    }

    #[test]
    fn test_permutation_inverse() {
        let a = Permutation {
            mapping: vec![2, 1, 0, 4, 3].into_iter().collect(),
        };
        let inverse = a.inverse();
        let b = inverse.op(&a);
        let idenity = Permutation::identity(a.mapping.len());
        assert_eq!(b.mapping, idenity.mapping);
        
    }
    #[test]
    fn test_permutation_safe_op_size_mismatch() {
        let a = Permutation {
            mapping: vec![0, 1, 2, 3].into_iter().collect(),
        };
        let b = Permutation {
            mapping: vec![0, 2, 1, 3, 4].into_iter().collect(),
        };
        let result = a.safe_op(&b);
        assert!(result.is_err(), "safe_op should return Err on size mismatch");
    }

    #[test]
    fn test_permutation_from_cycles_out_of_bounds() {
        // The cycle contains an element out of bounds for the given size
        let cycles = vec![vec![0, 5]]; // 5 is out of bounds for size 4
        let size = 4;
        let result = Permutation::from_cycles(&cycles, size);
        assert!(
            result.is_err(),
            "from_cycles should return Err when cycle contains out-of-bounds element"
        );
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
        let perm = Permutation {
            mapping: vec![1, 0, 3, 4, 2].into_iter().collect(),
        };
        let order = perm.order();
        assert_eq!(order, 6, "The order of the permutation should be 6");
    }


}


#[cfg(test)]
mod test_alternating_group_element {
    use super::*;
    use absagl::groups::permutation::AlternatingGroupElement;

    #[test]
    fn test_alternating_group_element_identity() {
        let identity = AlternatingGroupElement::identity(5);
        let id_mapping = vec![0, 1, 2, 3, 4].into_iter().collect::<Vec<usize>>();
        
        assert_eq!(identity.mapping(), &id_mapping);
    }

    #[test]
    fn test_alternating_group_element_creation() {
        let perm = Permutation {
            mapping: vec![1, 0, 2, 4, 3].into_iter().collect(),
        };
        let ag = AlternatingGroupElement::new(perm).expect("Should create AlternatingGroupElement");
        assert_eq!(ag.mapping().len(), 5);
    }


    #[test]
    fn test_alternating_group_element_op() {
        let perm1 = Permutation {
            mapping: vec![0, 2, 1, 4, 3].into_iter().collect(),
        };
        let perm2 = Permutation {
            mapping: vec![0, 2, 1, 4, 3].into_iter().collect(),
        };
        let ag1 = AlternatingGroupElement::new(perm1).expect("Should create AlternatingGroupElement");
        let ag2 = AlternatingGroupElement::new(perm2).expect("Should create AlternatingGroupElement");
        let result = ag1.op(&ag2);
        assert_eq!(result.mapping(), &vec![0, 1, 2, 3, 4].into_iter().collect::<Vec<usize>>());
    }

    #[test]
    fn test_alternating_group_element_inverse() {
        let perm = Permutation {
            mapping: vec![1, 0, 2, 4, 3].into_iter().collect(),
        };
        let ag = AlternatingGroupElement::new(perm).expect("Should create AlternatingGroupElement");
        let inverse = ag.inverse();
        let identity = AlternatingGroupElement::identity(5);
        let result = ag.op(&inverse);
        assert_eq!(result, identity, "The result of ag op inverse should be the identity element");
    }

    #[test]
    fn test_alternating_group_element_order() {
        let perm = Permutation {
            mapping: vec![1, 0, 2, 4, 3].into_iter().collect(),
        };
        let ag = AlternatingGroupElement::new(perm).expect("Should create AlternatingGroupElement");
        let order = ag.order();
        assert_eq!(order, 2, "The order of the alternating group element should be 2");
    }

    
}


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