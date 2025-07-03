

use absagl::groups::dihedral::DihedralElement;
use absagl::groups::permutation::AlternatingGroupElement;
use absagl::groups::GroupGenerators;
use absagl::groups::GroupElement;
use absagl::groups::permutation::Permutation;
use absagl::groups::modulo::Modulo;



use std::collections::HashMap;
use std::iter::Cycle;
use std::pin;
use log::{info, warn, error, debug};




fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // Initialize the logger

    let cycle1 = vec![vec![0, 1, 2]];
    let cycle2 = vec![vec![2, 1, 0]];

    let g1 = Permutation::from_cycles(&cycle1, 3)?;
    let g2 = Permutation::from_cycles(&cycle2, 3)?;

    let g3 = g1.safe_op(&g2)?;

    println!("result: {}", g3);


    let d1 = DihedralElement::new(3, false, 12)?;

    println!("Dihedral Element: {}", d1);

    println!("Identity: {}", DihedralElement::identity(12));

    println!("Inverse: {}", d1.inverse());

    println!("Order: {}", d1.order());

    let d3 = GroupGenerators::generate_dihedral_group(3)?;

    for element in d3.elements {
        println!("Dihedral Element: {}", element);
    }




    Ok(())
}
