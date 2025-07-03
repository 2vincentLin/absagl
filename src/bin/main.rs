

use absagl::groups::permutation::AlternatingGroupElement;
use absagl::groups::GroupGenerators;
use absagl::groups::GroupElement;
use absagl::groups::permutation::Permutation;
use absagl::groups::modulo::Modulo;



use std::collections::HashMap;
use std::iter::Cycle;
use log::{info, warn, error, debug};




fn main() {
    env_logger::init(); // Initialize the logger

    let perm1 = Permutation {
            mapping: vec![0, 2, 1, 4, 3].into_iter().collect(),
        };
    let perm2 = Permutation {
        mapping: vec![0, 2, 1, 4, 3].into_iter().collect(),
    };
    println!("permutation 1: {}", perm1);

    println!("order of permutation 1: {}", perm1.order());

    let ag1 = AlternatingGroupElement::new(perm1).expect("Should create AlternatingGroupElement");
    let ag2 = AlternatingGroupElement::new(perm2).expect("Should create AlternatingGroupElement");


    println!("Alternating Group Element 1: {}", ag1);
    let result = ag1.op(&ag2);
    println!("Result of operation: {:?}", result);
    println!("Result of operation: {}", result);

    let g = GroupGenerators::generate_alternating_group(4)
        .expect("Should generate alternating group");

    println!("Generated Alternating Group: {:?}", g.order());
    for element in &g.elements {
        println!("Element: {}", element);
    }

    let is_closed = g.is_closed();
    println!("Is the group closed? {}", is_closed);
}
