

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
use absagl::groups::Multiplicative;
use absagl::groups::{Additive};
use absagl::utils;
use rayon::result;



use std::collections::HashMap;
use std::iter::Cycle;
use std::pin;
use log::{info, warn, error, debug};




fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // Initialize the logger

    let a = Modulo::<Additive>::new(0, 3).expect("Failed to create Modulo element");
    let b = Modulo::<Additive>::new(1, 3).expect("Failed to create Modulo element");

    let group = FiniteGroup::new(vec![a, b]);
    println!("is group closed: {:?}", group.is_closed());


    // let coset1 = Coset::new(b, &group).expect("msg");

    match Coset::new(b, &group) {
        Ok(_) => println!("success"),
        Err(e) => println!("failed for: {}", e)
    }


    let a = 46 as i64;
    let b = 17 as i64;
    let result = utils::extended_gcd(a, b);
    println!("result is {:?}", result);


    let inverse = utils::modular_inverse(17, 46).unwrap();
    println!("inverse is {}", inverse);

    // let a = Modulo::<Additive>::new(0, 3).expect("Failed to create Modulo element");
    // let b = Modulo::<Multiplicative>::new(1, 3).expect("Failed to create Modulo element");

    // let result = a.op(&b);
    // println!("result is {:?}", result);

    // let group = Modulo::<Additive>::generate_group(5);
    // println!("the group is {:?}", group);

    let group = Modulo::<Multiplicative>::generate_group(4);
    println!("the group is {:?}", group);

    let group = Modulo::<Additive>::generate_group(4);
    println!("the group is {:?}", group);


    Ok(())
}
