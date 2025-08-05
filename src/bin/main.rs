

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
use rayon::vec;


use std::marker::PhantomData;
use std::collections::HashMap;
use std::iter::Cycle;
use std::pin;
use log::{info, warn, error, debug};




fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // Initialize the logger

    
    let n = 3;
    let sigma = Permutation::from_cycles(&vec![vec![0,1]], n)?;
    let tau = Permutation::from_cycles(&[(0..n).collect::<Vec<_>>()], n)?;
    let tau_inverse = tau.inverse();

    println!("sigma, tau, tau_inverse: {}, {}, {}", sigma, tau, tau_inverse);

    // let result = tau * sigma * tau_inverse;
    // println!("result: {}", result);

    let result = (tau.clone()*tau.clone()) * sigma * (tau_inverse.clone()*tau_inverse);
    println!("result: {}", result);

    let n = 4;
    let sigma = Permutation::from_cycles(&vec![vec![0,1]], n)?;
    let tau = Permutation::from_cycles(&[(0..n).collect::<Vec<_>>()], n)?;
    let tau_inverse = tau.inverse();

    println!("sigma, tau, tau_inverse: {}, {}, {}", sigma, tau, tau_inverse);

    // let result = tau * sigma * tau_inverse;
    // println!("result: {}", result);

    let result = (&tau * &tau) * sigma * (&tau_inverse * &tau_inverse);
    println!("result: {}", result);

    println!("13 /=2 = {}", 13 / 2);

    Ok(())
}
