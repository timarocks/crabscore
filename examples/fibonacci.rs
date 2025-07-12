/// Fibonacci calculation example with comprehensive documentation and tests.
/// 
/// This example demonstrates how good documentation and test coverage
/// can significantly improve your CrabScore. The bonus system rewards
/// Rust best practices.
/// 
/// Expected CrabScore: 85-95/100 with bonuses for:
/// - Small Project Bonus (+2.0)
/// - Excellent Documentation (+2.0)
/// - Good Test Coverage (+2.0)
/// - Zero Dependencies (+3.0)

/// Calculate the nth Fibonacci number using efficient recursion.
/// 
/// # Arguments
/// 
/// * `n` - The position in the Fibonacci sequence (0-indexed)
/// 
/// # Returns
/// 
/// The Fibonacci number at position `n`
/// 
/// # Examples
/// 
/// ```rust
/// use fibonacci::fibonacci;
/// 
/// assert_eq!(fibonacci(0), 0);
/// assert_eq!(fibonacci(1), 1);
/// assert_eq!(fibonacci(10), 55);
/// ```
/// 
/// # Performance
/// 
/// This implementation has O(2^n) time complexity for demonstration.
/// In production, consider using dynamic programming for better performance.
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// Calculate Fibonacci using dynamic programming for better performance.
/// 
/// # Arguments
/// 
/// * `n` - The position in the Fibonacci sequence
/// 
/// # Returns
/// 
/// The Fibonacci number at position `n`
/// 
/// # Performance
/// 
/// This implementation has O(n) time complexity and O(1) space complexity.
pub fn fibonacci_dp(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    
    let mut prev = 0u64;
    let mut curr = 1u64;
    
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    
    curr
}

fn main() {
    println!("Fibonacci Calculator");
    println!("===================");
    
    for i in 0..=10 {
        println!("fibonacci({}) = {}", i, fibonacci_dp(i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_base_cases() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn test_fibonacci_sequence() {
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
    }

    #[test]
    fn test_fibonacci_dp_matches_recursive() {
        for i in 0..=20 {
            assert_eq!(fibonacci(i), fibonacci_dp(i), "Mismatch at position {}", i);
        }
    }

    #[test]
    fn test_fibonacci_dp_performance() {
        // This should complete quickly even for larger numbers
        let result = fibonacci_dp(50);
        assert!(result > 0);
    }
}