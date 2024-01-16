use num_bigint::{BigUint, RandBigInt};
use rand::Rng;

pub struct ZKP {
    /// Prime order
    p: BigUint,

    /// Group order
    q: BigUint,

    /// Generators
    alpha: BigUint,
    beta: BigUint,
}

impl ZKP {
    /// alpha^x mod p
    /// output = n^exponent mod modulus
    pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
        n.modpow(exponent, modulus)
    }

    /// output = s = k - c * x mod q
    pub fn solve(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
        if *k >= c * x {
            return (k - c * x).modpow(&BigUint::from(1u32), &self.q);
        }

        return &self.q - (c * x - k).modpow(&BigUint::from(1u32), &self.q);
    }

    /// cond1: r1 = alpha^s * y1^c
    /// cond2: r2 = beta^s * y2^c
    pub fn verify(
        &self,
        r1: &BigUint,
        r2: &BigUint,
        y1: &BigUint,
        y2: &BigUint,
        c: &BigUint,
        s: &BigUint,
    ) -> bool {
        let cond1 = *r1
            == (&self.alpha.modpow(s, &self.p) * y1.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);
        let cond2 = *r2
            == (&self.beta.modpow(s, &self.p) * y2.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);
        cond1 && cond2
    }

    pub fn generate_random_number_below(bound: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();

        rng.gen_biguint_below(bound)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_toy_example() {
        /// all the constants
        let _alpha: BigUint = BigUint::from(4u32);
        let beta: BigUint = BigUint::from(9u32);
        let p: BigUint = BigUint::from(23u32);
        /// prime order
        let q: BigUint = BigUint::from(11u32);
        /// group order
        let zkp = ZKP {
            p: p.clone(),
            q: q.clone(),
            alpha: _alpha.clone(),
            beta: beta.clone(),
        };

        /// secret
        let x: BigUint = BigUint::from(6u32);

        /// Random constant value
        let k: BigUint = BigUint::from(7u32);

        /// Challene: verifier side
        let c: BigUint = BigUint::from(4u32);

        /// Compute the y1 and y2
        /// y1 = alpha^x mod p
        /// y2 = beta^x mod p
        let y1 = ZKP::exponentiate(&_alpha, &x, &p);
        let y2 = ZKP::exponentiate(&beta, &x, &p);

        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = ZKP::exponentiate(&_alpha, &k, &p);
        let r2 = ZKP::exponentiate(&beta, &k, &p);
        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        let s: BigUint = zkp.solve(&k, &c, &x);
        assert_eq!(s, BigUint::from(5u32));

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);

        // Man-in-the-middle
        let x_fake = BigUint::from(7u32);
        let s_fake: BigUint = zkp.solve(&k, &c, &x_fake);
        let result_fake = zkp.verify(&r1, &r2, &y1, &y2, &c, &s_fake);
        assert!(!result_fake);
    }

    #[test]
    fn test_toy_example_with_random_numbers() {
        // all the constants
        let _alpha: BigUint = BigUint::from(4u32);
        let beta: BigUint = BigUint::from(9u32);
        let p: BigUint = BigUint::from(23u32); // prime order
        let q: BigUint = BigUint::from(11u32); // group order

        let zkp = ZKP {
            p: p.clone(),
            q: q.clone(),
            alpha: _alpha.clone(),
            beta: beta.clone(),
        };
        // let zkp = ZKP { p, q, alpha: _alpha, beta };

        // secret
        let x: BigUint = BigUint::from(6u32);

        // Random constant value
        let k: BigUint = ZKP::generate_random_number_below(&q);

        // Challene: verifier side
        let c: BigUint = ZKP::generate_random_number_below(&q);

        // Compute the y1 and y2
        // y1 = alpha^x mod p
        // y2 = beta^x mod p
        let y1 = ZKP::exponentiate(&_alpha, &x, &p);
        let y2 = ZKP::exponentiate(&beta, &x, &p);
        let r1 = ZKP::exponentiate(&_alpha, &k, &p);
        let r2 = ZKP::exponentiate(&beta, &k, &p);
        let s: BigUint = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);
    }
}
