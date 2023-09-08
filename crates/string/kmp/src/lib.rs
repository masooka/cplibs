/// Computes the Knuth-Morris-Pratt (KMP) prefix function for a given string.
///
/// The KMP prefix function is an array `pi` of length `n` where `n` is the
/// length of the string. For each `i`, `pi[i]` is the length of the longest
/// proper prefix of the substring `s[0..i+1]` which is also a proper suffix of
/// this substring.
///
/// # Example
///
/// ```
/// # use kmp::kmp_prefix;
/// let s = "abcdabc";
/// let pi = kmp_prefix(s);
/// println!("{:?}", pi);  // Output will be [0, 0, 0, 0, 1, 2, 3]
/// ```
pub fn kmp_prefix(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut pi = vec![0; n];
    let s_bytes = s.as_bytes();

    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && s_bytes[i] != s_bytes[j] {
            j = pi[j - 1];
        }
        if s_bytes[i] == s_bytes[j] {
            j += 1;
        }
        pi[i] = j;
    }

    pi
}
