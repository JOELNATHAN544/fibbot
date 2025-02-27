use num::BigUint;
pub fn fib_sequence(n: u64) -> BigUint {
    let zero = BigUint::from(0u32);
    let one = BigUint::from(1u32);
    let mut a = zero.clone();
    let mut b = one.clone();

    for _ in 0..n {
        let next = &a + &b;
        a = b.clone();
        b = next;
    }

    a
}
