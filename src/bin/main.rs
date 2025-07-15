

use absagl::error::AbsaglError;
use absagl::groups::dihedral::DihedralElement;
use absagl::groups::factor::Coset;
use absagl::groups::permutation::AlternatingGroupElement;
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
use rayon::result;


use std::marker::PhantomData;
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


    let a = Modulo::<Additive>::new(0, 3).expect("Failed to create Modulo element");
    let b = Modulo::<Additive>::new(1, 3).expect("Failed to create Modulo element");

    let group1 = FiniteGroup::new(vec![a, b]);
    let group2= FiniteGroup::new(vec![b, a]);

    if group1 == group2 {
        println!("they are equal");
    } else {
        println!("they are not equal");
    }

    let a = Permutation::new(vec![0,1,2,3]).expect("should create permutation");
    let k: Vec<u8> = a.mapping().iter().flat_map(|&x| x.to_be_bytes()).collect();
    println!("k: {:?}", k);

    let a: Vec<u8> = vec![0,0,0,0,0,0,0,1]; // 1
    let b: Vec<u8> = vec![0,0,0,0,0,0,1,0]; // 

    let mut v = vec![a.clone(),b.clone()];
    v.sort();

    println!("1st v after sort: {:?}", v);

    let mut v = vec![b,a];
    v.sort();

    println!("2nd v after sort: {:?}", v);

    let a = Permutation::new(vec![0,1]).expect("should create permutation");
    println!("canonical form: {:?}", a.to_canonical_bytes());
    let b : Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    if a.to_canonical_bytes() == b {
        println!("a, b are equal");
    }

    let a = Modulo::<Additive>::new(2, 5).expect("should create permutation");
    println!("canonical form: {:?}", a.to_canonical_bytes());
    let b : Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 5];
    if a.to_canonical_bytes() == b {
        println!("a, b are equal");
    }


    let d1 = DihedralElement::new(1, false,9).unwrap();
    println!("d1: {:?}", d1);
    println!("d1 to canonical: {:?}", d1.to_canonical_bytes());
    

    let e = Modulo::<Additive>::new(0, 8).expect("should create element");
    let a = Modulo::<Additive>::new(2, 8).expect("should create element");
    let b = Modulo::<Additive>::new(4, 8).expect("should create element");
    let c = Modulo::<Additive>::new(6, 8).expect("should create element");


    let group1 = FiniteGroup::new(vec![e,a,b,c]);
    let group2 = FiniteGroup::new(vec![e,b]);

    let coset1 = Coset::new(a, &group1).unwrap();
    let coset2 = Coset::new(b, &group2).unwrap();

    println!("canonical form for coset1: {:?}", coset1.get_canonical_representative());
    println!("canonical form for coset2: {:?}", coset2.get_canonical_representative());
    

    Ok(())
}
