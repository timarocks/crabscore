/// Library crate example demonstrating CrabScore analysis of Rust libraries.
/// 
/// This example shows how CrabScore handles library-only projects that don't
/// produce binary executables. The scoring system gracefully falls back to
/// static analysis and complexity-based estimation.
/// 
/// Expected CrabScore: 80-90/100 with bonuses for:
/// - Small Project Bonus (+2.0)
/// - Good Documentation (+2.0)
/// - Zero Dependencies (+3.0)

/// A simple mathematical utilities library for demonstration.
pub mod math {
    /// Calculate the greatest common divisor of two numbers using Euclid's algorithm.
    /// 
    /// # Arguments
    /// 
    /// * `a` - First number
    /// * `b` - Second number
    /// 
    /// # Returns
    /// 
    /// The greatest common divisor of `a` and `b`
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use library_crate::math::gcd;
    /// 
    /// assert_eq!(gcd(48, 18), 6);
    /// assert_eq!(gcd(17, 13), 1);
    /// ```
    pub fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    /// Calculate the least common multiple of two numbers.
    /// 
    /// # Arguments
    /// 
    /// * `a` - First number
    /// * `b` - Second number
    /// 
    /// # Returns
    /// 
    /// The least common multiple of `a` and `b`
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use library_crate::math::lcm;
    /// 
    /// assert_eq!(lcm(4, 6), 12);
    /// assert_eq!(lcm(15, 25), 75);
    /// ```
    pub fn lcm(a: u64, b: u64) -> u64 {
        if a == 0 || b == 0 {
            0
        } else {
            (a * b) / gcd(a, b)
        }
    }
}

/// String manipulation utilities
pub mod strings {
    /// Check if a string is a palindrome (reads the same forwards and backwards).
    /// 
    /// # Arguments
    /// 
    /// * `s` - The string to check
    /// 
    /// # Returns
    /// 
    /// `true` if the string is a palindrome, `false` otherwise
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use library_crate::strings::is_palindrome;
    /// 
    /// assert_eq!(is_palindrome("racecar"), true);
    /// assert_eq!(is_palindrome("hello"), false);
    /// assert_eq!(is_palindrome("A man a plan a canal Panama"), false); // case sensitive
    /// ```
    pub fn is_palindrome(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        
        for i in 0..len / 2 {
            if chars[i] != chars[len - 1 - i] {
                return false;
            }
        }
        
        true
    }

    /// Reverse the words in a string while preserving the word order.
    /// 
    /// # Arguments
    /// 
    /// * `s` - The input string
    /// 
    /// # Returns
    /// 
    /// A new string with words reversed
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use library_crate::strings::reverse_words;
    /// 
    /// assert_eq!(reverse_words("hello world"), "olleh dlrow");
    /// assert_eq!(reverse_words("rust is awesome"), "tsur si emosewa");
    /// ```
    pub fn reverse_words(s: &str) -> String {
        s.split_whitespace()
            .map(|word| word.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod math_tests {
        use super::super::math::*;

        #[test]
        fn test_gcd_basic() {
            assert_eq!(gcd(48, 18), 6);
            assert_eq!(gcd(17, 13), 1);
            assert_eq!(gcd(100, 25), 25);
        }

        #[test]
        fn test_gcd_edge_cases() {
            assert_eq!(gcd(0, 5), 5);
            assert_eq!(gcd(5, 0), 5);
            assert_eq!(gcd(1, 1), 1);
        }

        #[test]
        fn test_lcm_basic() {
            assert_eq!(lcm(4, 6), 12);
            assert_eq!(lcm(15, 25), 75);
            assert_eq!(lcm(7, 11), 77);
        }

        #[test]
        fn test_lcm_edge_cases() {
            assert_eq!(lcm(0, 5), 0);
            assert_eq!(lcm(5, 0), 0);
            assert_eq!(lcm(1, 10), 10);
        }
    }

    mod string_tests {
        use super::super::strings::*;

        #[test]
        fn test_palindrome_positive() {
            assert!(is_palindrome("racecar"));
            assert!(is_palindrome("level"));
            assert!(is_palindrome(""));
            assert!(is_palindrome("a"));
        }

        #[test]
        fn test_palindrome_negative() {
            assert!(!is_palindrome("hello"));
            assert!(!is_palindrome("rust"));
            assert!(!is_palindrome("Racecar")); // case sensitive
        }

        #[test]
        fn test_reverse_words() {
            assert_eq!(reverse_words("hello world"), "olleh dlrow");
            assert_eq!(reverse_words("rust is awesome"), "tsur si emosewa");
            assert_eq!(reverse_words(""), "");
            assert_eq!(reverse_words("single"), "elgnis");
        }
    }
}