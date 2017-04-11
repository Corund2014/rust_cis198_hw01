#![cfg(test)]
use problem2::{mat_mult,Matrix};
use problem3::sieve;



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
