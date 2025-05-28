

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
