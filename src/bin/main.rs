

use absagl::error::AbsaglError;
use absagl::groups::dihedral::DihedralElement;
use absagl::groups::factor::Coset;
use absagl::groups::permutation::AlternatingGroupElement;
use absagl::groups::Group;
use absagl::groups::GroupGenerators;
use absagl::groups::GroupElement;
use absagl::groups::permutation::Permutation;
use absagl::groups::modulo::Modulo;
use absagl::groups::FiniteGroup;



use std::collections::HashMap;
use std::iter::Cycle;
use std::pin;
use log::{info, warn, error, debug};




fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // Initialize the logger

        let a = Modulo::new(0, 3).expect("Failed to create Modulo element");
        let b = Modulo::new(1, 3).expect("Failed to create Modulo element");

        let group = FiniteGroup::new(vec![a, b]);
        println!("is group closed: {:?}", group.is_closed());


        // let coset1 = Coset::new(b, &group).expect("msg");

        match Coset::new(b, &group) {
            Ok(_) => println!("success"),
            Err(e) => println!("failed for: {}", e)
        }


    Ok(())
}
