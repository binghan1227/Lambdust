//! Example demonstrations of lambda calculus evaluation

use crate::church::*;
use crate::eval::{bind_vars, trace_eval};
use crate::expr::{app, var};

pub fn test_if_true() {
    println!("=== Test: IF TRUE THEN x ELSE y ===");
    let test = church_if(church_true(), var("x".to_string()), var("y".to_string()));
    let bound = bind_vars(*test);
    trace_eval(bound, 10, true, true);
}

pub fn test_if_false() {
    println!("=== Test: IF FALSE THEN x ELSE y ===");
    let test = church_if(church_false(), var("x".to_string()), var("y".to_string()));
    let bound = bind_vars(*test);
    trace_eval(bound, 10, true, true);
}

pub fn test_not_true() {
    println!("=== Test: NOT TRUE ===");
    let test = church_not(church_true());
    let bound = bind_vars(*test);
    trace_eval(bound, 10, true, true);
}

pub fn test_not_false() {
    println!("=== Test: NOT FALSE ===");
    let test = church_not(church_false());
    let bound = bind_vars(*test);
    trace_eval(bound, 10, true, true);
}

pub fn test_and_true_true() {
    println!("=== Test: AND TRUE TRUE ===");
    let test = church_and(church_true(), church_true());
    let bound = bind_vars(*test);
    trace_eval(bound, 10, true, true);
}

pub fn test_and_true_false() {
    println!("=== Test: AND TRUE FALSE ===");
    let test = church_and(church_true(), church_false());
    let bound = bind_vars(*test);
    trace_eval(bound, 10, true, true);
}

pub fn test_succ_zero() {
    println!("=== Test: SUCC 0 (should give 1) ===");
    let test = app(church_succ(), church_zero());
    let bound = bind_vars(*test);
    trace_eval(bound, 10, true, true);
}

pub fn test_succ_two() {
    println!("=== Test: SUCC 2 (should give 3) ===");
    let test = app(church_succ(), church_two());
    let bound = bind_vars(*test);
    trace_eval(bound, 10, true, true);
}

pub fn test_add_one_two() {
    println!("=== Test: ADD 1 2 (should give 3) ===");
    let test = app(app(church_add(), church_one()), church_two());
    let bound = bind_vars(*test);
    trace_eval(bound, 10, true, true);
}

pub fn test_add_zero_two() {
    println!("=== Test: ADD 0 2 (should give 2) ===");
    let test = app(app(church_add(), church_zero()), church_two());
    let bound = bind_vars(*test);
    trace_eval(bound, 10, true, true);
}

pub fn test_mult_two_three() {
    println!("=== Test: MULT 2 3 (should give 6) ===");
    let test = app(app(church_mult(), church_two()), church_three());
    let bound = bind_vars(*test);
    trace_eval(bound, 10, true, true);
}

pub fn test_mult_zero_three() {
    println!("=== Test: MULT 0 3 (should give 0) ===");
    let test = app(app(church_mult(), church_zero()), church_three());
    let bound = bind_vars(*test);
    trace_eval(bound, 10, true, true);
}

pub fn test_is_zero_zero() {
    println!("=== Test: IS_ZERO 0 ===");
    let test = app(church_is_zero(), church_zero());
    let bound = bind_vars(*test);
    trace_eval(bound, 10, true, true);
}

pub fn test_is_zero_one() {
    println!("=== Test: IS_ZERO 1 ===");
    let test = app(church_is_zero(), church_one());
    let bound = bind_vars(*test);
    trace_eval(bound, 10, true, true);
}

pub fn test_is_zero_two() {
    println!("=== Test: IS_ZERO 2 ===");
    let test = app(church_is_zero(), church_two());
    let bound = bind_vars(*test);
    trace_eval(bound, 10, true, true);
}

pub fn run_all_examples() {
    println!("========== BOOLEAN TESTS ==========\n");
    test_if_true();
    println!();
    test_if_false();
    println!();
    test_not_true();
    println!();
    test_not_false();
    println!();
    test_and_true_true();
    println!();
    test_and_true_false();

    println!("\n========== NUMERAL TESTS ==========\n");
    test_succ_zero();
    println!();
    test_succ_two();
    println!();
    test_add_one_two();
    println!();
    test_add_zero_two();
    println!();
    test_mult_two_three();
    println!();
    test_mult_zero_three();
    println!();
    test_is_zero_zero();
    println!();
    test_is_zero_one();
    println!();
    test_is_zero_two();
}
