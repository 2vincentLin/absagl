

/// Checks whether the given `mapping` is a valid permutation of `0..n`.
pub fn is_mapping_valid(mapping: &[usize]) -> bool {
    let n = mapping.len();
    let mut seen = vec![false; n];

    for &val in mapping {
        if val >= n || seen[val] {
            return false; // Out-of-bounds or duplicate
        }
        seen[val] = true;
    }

    true
}

/// Computes the greatest common divisor (GCD) of two numbers using the Euclidean algorithm.
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Computes the least common multiple (LCM) of two numbers using the GCD.
pub fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 { 0 } else { (a * b) / gcd(a, b) }
}


/// Performs the Extended Euclidean Algorithm to find the GCD
/// and the coefficients that solve Bézout's identity.
/// Returns a tuple (gcd, u, v) such that a*u + b*v = gcd.
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    // Base case
    if b == 0 {
        return (a, 1, 0);
    }

    // Recursive call
    let (g, u_prime, v_prime) = extended_gcd(b, a % b);

    // Update coefficients using the results of the recursive call
    let u = v_prime;
    let v = u_prime - (a / b) * v_prime;

    (g, u, v)
}

/// Finds the modular multiplicative inverse of a number 'a' modulo 'n'.
/// Returns `Some(inverse)` if it exists (i.e., gcd(a, n) == 1),
/// otherwise returns `None`.
pub fn modular_inverse(a: i64, n: i64) -> Option<i64> {
    let (g, u, _) = extended_gcd(a, n);

    if g != 1 {
        // The inverse does not exist if the numbers are not coprime.
        None
    } else {
        // The coefficient `u` is our inverse. We use the modulo trick
        // `(u % n + n) % n` to ensure the result is positive.
        Some((u % n + n) % n)
    }
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_gcd() {
        let result = gcd(2024 as usize, 748 as usize);
        assert_eq!(result, 44 as usize)
    }




}