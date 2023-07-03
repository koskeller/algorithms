pub fn is_balanced(s: &str) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        if c == '(' {
            stack.push(c);
        } else if c == ')' {
            if stack.pop().is_none() {
                return false;
            }
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok() {
        assert!(is_balanced(""));
        assert!(is_balanced("()"));
        assert!(is_balanced("((())())()"));
        assert!(is_balanced("()()"));
        assert!(is_balanced("(())"));
        assert!(is_balanced("(()())"));
        assert!(is_balanced("(((())))"));
        assert!(is_balanced("((()()))"));
        assert!(is_balanced("(a(b)c)"));
        assert!(is_balanced("(()d)e(f(g))"));
        assert!(!is_balanced(")(a(b)c)"));

        assert!(!is_balanced("("));
        assert!(!is_balanced(")"));
        assert!(!is_balanced(")()("));
        assert!(!is_balanced("(()"));
        assert!(!is_balanced("())"));
        assert!(!is_balanced(")("));
        assert!(!is_balanced("((((()))"));
        assert!(!is_balanced("()())"));
        assert!(!is_balanced("())"));

        assert!(is_balanced(&"()".repeat(10_000)));
    }
}
