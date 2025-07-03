

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