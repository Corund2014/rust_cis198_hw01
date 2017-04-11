/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    // TODO
    let mut primes:Vec<u32>=Vec::new();
    let mut not_primes:Vec<u32>=Vec::new();
    for i in 2..n
    {
        
        if !not_primes.contains(&i) {primes.push(i);}
        let mut x=2*i;
        while x<n
        {
            if !not_primes.contains(&x) {not_primes.push(x);}
            x+=i;
        }

    }
    primes
}
