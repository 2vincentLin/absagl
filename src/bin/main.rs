

use absagl::error::AbsaglError;
use absagl::groups::dihedral::DihedralElement;
use absagl::groups::factor::Coset;
use absagl::groups::factor::CosetError;
use absagl::groups::factor::FactorGroup;
use absagl::groups::permutation::AlternatingGroupElement;
use absagl::groups::permutation::PermutationError;
use absagl::groups::CanonicalRepr;
use absagl::groups::Group;
use absagl::groups::GroupGenerators;
use absagl::groups::GroupElement;
use absagl::groups::permutation::Permutation;
use absagl::groups::modulo::Modulo;
use absagl::groups::FiniteGroup;
use absagl::groups::Multiplicative;
use absagl::groups::{Additive};
use absagl::utils;
use absagl::groups::factor::CosetSide;
use rayon::result;


use std::marker::PhantomData;
use std::collections::HashMap;
use std::iter::Cycle;
use std::pin;
use log::{info, warn, error, debug};




fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // Initialize the logger

    
    
    let a = Permutation::new(vec![0,1,2]).unwrap();
    let b = Permutation::new(vec![0,1]).unwrap();
    let s3 = GroupGenerators::generate_permutation_group(3).unwrap();
    println!("s3: {:?}", s3);

    let a3 = FiniteGroup::new(Permutation::generate_alternative_group(3).unwrap()).unwrap();
    println!("a3: {:?}", a3);

    let coset1 = Coset::new(a, &s3, CosetSide::Left).unwrap();
    let coset2 = Coset::new(b, &s3, CosetSide::Left).unwrap();

    match coset1.safe_op(&coset2) {
        Ok(_) => println!("good"),
        Err(CosetError::Element(e)) => {
            println!("error is {:?}", &e);

            match e.downcast_ref::<PermutationError>() {
                Some(&PermutationError::SizeNotMatch) => println!("this one: SIZE NOT MATCH"),
                Some(c) => println!("other error: {:?}", c),
                None => println!("it's none")
            }

            dbg!(&e);

            // match e {
            //     PermutationError::SizeNotMatch => println!("this one"),
            //     _ => println!("others")
            // }
            
            if let Some(error) = e.downcast_ref::<PermutationError>() {
                println!("inner error is: {:?}", error);

            }
        
        },
        Err(e) => println!("other error: {:?}", e)
    }



    // let f3 = FactorGroup::new(&s3, &a3);
    // println!("f3: {:?}", f3);

    // let m6 = GroupGenerators::generate_modulo_group_mul(6).unwrap();
    // println!("m6: {:?}", m6);


    // let a = Modulo::new(1,3).unwrap();
    // let z3 = GroupGenerators::generate_modulo_group_add(3).unwrap();

    // let coset1 = Coset::new(a, &z3, CosetSide::Left).unwrap();

    // let b = Modulo::new(1,4).unwrap();
    // let z4 = GroupGenerators::generate_modulo_group_add(4).unwrap();

    // let coset2 = Coset::new(b, &z4, CosetSide::Left).unwrap();

    // match coset1.safe_op(&coset2) {
    //     Ok(_) => println!("good"),
    //     Err(AbsaglError::Other(e)) => println!("error is {:?}", e),
    //     Err(e) => println!("other error: {:?}", e)
    // }

    

    Ok(())
}
