// mod groups;
// mod utils;

use absagl::groups::GroupGenerators;
use absagl::groups::GroupElement;
use absagl::groups::permutation::Permutation;
use absagl::groups::modulo::Modulo;



// use crate::groups::GroupElement;
// use crate::groups::modulo::Modulo;
// use crate::groups::permutation::Permutation;
// use crate::groups::permutation::SparsePerm;
use std::collections::HashMap;



fn main() {
    let a = Permutation {
        mapping: vec![0,1,2,4,3].into_iter().collect(),
    };
    let b = Permutation {
        mapping: vec![0,2,1,3,4].into_iter().collect(),
    };
    let c = a.op(&b);
    assert_eq!(c.mapping, vec![0, 2, 1, 4, 3].into_iter().collect::<Vec<_>>());
    println!("Result of a op b: {:?}", c.mapping);

    println!("is_even: {}", c.is_even());

    let identity = Permutation::identity(5);
    println!("is_even: {}", identity.is_even());

    let d = Permutation {
        mapping: vec![0,2,3,4,1].into_iter().collect(),
    };

    println!("d is even: {}", d.is_even());
    
    let p = match Permutation::from_cycles(&vec![vec![1,2], vec![3,5]], 6) {
        Ok(p) => {
            println!("Permutation from cycles: {:?}", p.mapping);
            p
        },
        Err(e) => {
            eprintln!("Error creating permutation from cycles: {:?}", e);
            return;
        }
    };

    println!("Permutation from cycles: {}", p);

    let p = match Permutation::generate_group(3) {
        Ok(group) => {
            // println!("Generated permutation group: {:?}", group);
            group
        },
        Err(e) => {
            eprintln!("Error generating permutation group: {:?}", e);
            return;
        }
    };
    println!("length of generated group: {}", p.len());


}
