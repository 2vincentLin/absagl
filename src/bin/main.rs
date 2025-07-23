

use absagl::error::AbsaglError;
use absagl::groups::dihedral::DihedralElement;
use absagl::groups::factor::Coset;
use absagl::groups::factor::CosetError;
use absagl::groups::factor::FactorGroup;
use absagl::groups::permutation::AlternatingGroupElement;
use absagl::groups::permutation::PermutationError;
use absagl::groups::CanonicalRepr;
use absagl::groups::CheckedOp;
use absagl::groups::Group;
use absagl::groups::GroupGenerators;
use absagl::groups::GroupElement;
use absagl::groups::permutation::Permutation;
use absagl::groups::modulo::Modulo;
use absagl::groups::FiniteGroup;
use absagl::groups::Multiplicative;
use absagl::groups::{Additive};
use absagl::utils;
use absagl::show;
use absagl::homomorphism::Homomorphism;
use absagl::groups::factor::CosetSide;
use rayon::result;


use std::marker::PhantomData;
use std::collections::HashMap;
use std::iter::Cycle;
use std::pin;
use log::{info, warn, error, debug};




fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // Initialize the logger

    
    



    let valid_mapping = |m: &Modulo<Additive>| Modulo::<Additive>::new(m.value() % 2, 2).unwrap();
    let hom = Homomorphism::new(valid_mapping, None);

    println!("Homomorphism: {:?}", hom);

    let valid_mapping = |m: &Modulo<Additive>| Modulo::<Additive>::new(m.value() % 2, 2).unwrap();
    let hom = Homomorphism::new(valid_mapping, Some("to Z_2".to_string()));

    println!("Homomorphism: {:?}", hom);

    println!("# Example: Homomorphism Kernel\n");

    let z4 = show!(GroupGenerators::generate_modulo_group_add(4).unwrap());
    let identity_h = show!(Modulo::<Additive>::new(0, 2).unwrap());

    let mapping = |m: &Modulo<Additive>| Modulo::<Additive>::new(m.value() % 2, 2).unwrap();

    let hom = show!(Homomorphism::try_new(&z4, mapping, None).unwrap());

    show!(hom.kernel(&z4, &identity_h));

    let a = Modulo::<Multiplicative>::new(1,3).unwrap();
    println!("order of {} is {}", a, a.order());

    let a = Modulo::<Additive>::new(0,3).unwrap();
    println!("order of {:?} is {}", a, a.order());

    let a = Permutation::new(vec![0, 2, 1, 4, 3]).expect("should create permutation");
    println!("a: {}", a);
    

    Ok(())
}
