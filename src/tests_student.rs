#![cfg(test)]
use problem2::{mat_mult,Matrix};
use problem3::sieve;
use problem4::{hanoi,Peg,Move};

const a_b:Move = (Peg::A,Peg::B);
const a_c:Move = (Peg::A,Peg::C);
const b_a:Move = (Peg::B,Peg::A);
const b_c:Move = (Peg::B,Peg::C);
const c_a:Move = (Peg::C,Peg::A);
const c_b:Move = (Peg::C,Peg::B);

#[test]
fn identity_matrix_multiplication()
{
    let identity3x3:Matrix=vec![vec![1.0,0.0,0.0],
                                vec![0.0,1.0,0.0],
                                vec![0.0,0.0,1.0]];
    let random_matrix:Matrix=vec![vec![-1.5,4.2,2.9],
                                vec![0.0,10.4,-3.0],
                                vec![9.2,-50.4,9.2]];
    let result_matrix=mat_mult(&random_matrix,&identity3x3);
    assert_eq!(random_matrix,result_matrix );
}

#[test]
fn primes_up_to_ten()
{
    let test_vector:Vec<u32>=vec![2,3,5,7];
    let result=sieve(10);
    assert_eq!(test_vector,result);
}

#[test]
fn tower_1()
{
    let test_vector=vec![a_c];
    let result=hanoi(1,Peg::A,Peg::B,Peg::C);
    assert_eq!(test_vector,result);
}

#[test]
fn tower_2()
{
    let test_vector=vec![a_b,a_c,b_c];
    let result=hanoi(1,Peg::A,Peg::B,Peg::C);
    assert_eq!(test_vector,result);
}

#[test]
fn tower_3()
{
    let test_vector=vec![a_c,a_b,c_b,a_c,b_a,b_c,c_a];
    let result=hanoi(1,Peg::A,Peg::B,Peg::C);
    assert_eq!(test_vector,result);
}
