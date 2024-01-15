use num_bigint::BigUint;

/// alpha^x mod p
/// output = n^exponent mod modulus
pub fn exponentiate(
    n: &BigUint,
    exponent: &BigUint,
    modulus: &BigUint
) -> BigUint {
    n.modpow(exponent, modulus)
}

/// output = s = k - c * x mod q
pub fn solve(
    k: &BigUint,
    c: &BigUint,
    x: &BigUint,
    q: &BigUint
) -> BigUint {
    if *k >= c * x {
        return (k - c * x).modpow(&BigUint::from(1u32), q);
    }

    return q - (c * x - k).modpow(&BigUint::from(1u32), q);
}

/// cond1: r1 = alpha^s * y1^c
/// cond2: r2 = beta^s * y2^c
pub fn verify(
    r1: &BigUint,
    r2: &BigUint,
    y1: &BigUint,
    y2: &BigUint,
    aplha: &BigUint,
    beta: &BigUint,
    c: &BigUint,
    s: &BigUint,
    p: &BigUint
) -> bool {
    let cond1 = *r1 == (aplha.modpow(s, p) * y1.modpow(c, p)).modpow(&BigUint::from(1u32), p);
    let cond2 = *r2 == (beta.modpow(s, p) * y2.modpow(c, p)).modpow(&BigUint::from(1u32), p);
    cond1 && cond2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_toy_example() {
        // all the constants
        let _alpha:BigUint = BigUint::from(4u32);
        let beta:BigUint = BigUint::from(9u32);
        let p:BigUint = BigUint::from(23u32); // prime order
        let q:BigUint = BigUint::from(11u32);  // group order

        // secret
        let x:BigUint = BigUint::from(6u32);

        // Random constant value
        let k:BigUint = BigUint::from(7u32);


        // Challene: verifier side
        let c:BigUint = BigUint::from(4u32);

        // Compute the y1 and y2
        // y1 = alpha^x mod p
        // y2 = beta^x mod p
        let y1 = exponentiate(&_alpha, &x, &p);
        let y2 = exponentiate(&beta, &x, &p);

        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = exponentiate(&_alpha, &k, &p);
        let r2 = exponentiate(&beta, &k, &p);
        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        let s: BigUint = solve(&k, &c, &x, &q);
        assert_eq!(s, BigUint::from(5u32));

        let result = verify(&r1, &r2, &y1, &y2, &_alpha, &beta, &c, &s, &p);
        assert!(result);


        // Man-in-the-middle
        let x_fake = BigUint::from(7u32);
        let s_fake: BigUint = solve(&k, &c, &x_fake, &q);
        let result_fake = verify(&r1, &r2, &y1, &y2, &_alpha, &beta, &c, &s_fake, &p);
        assert!(!result_fake);

    }
}