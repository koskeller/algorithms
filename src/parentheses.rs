#![allow(unused)]
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

pub fn longest_parentheses(s: &str) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    stack.push(-1);

    let mut max = 0;
    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            stack.push(i as i32);
        } else if c == ')' {
            stack.pop();
            if stack.is_empty() {
                stack.push(i as i32);
            } else {
                max = std::cmp::max(max, i as i32 - stack.last().unwrap());
            }
        }
    }
    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_balanced_works() {
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

    #[test]
    fn longest_parentheses_works() {
        assert_eq!(longest_parentheses(""), 0);
        assert_eq!(longest_parentheses(")("), 0);

        assert_eq!(longest_parentheses("()"), 2);
        assert_eq!(longest_parentheses("()("), 2);
        assert_eq!(longest_parentheses("())"), 2);
        assert_eq!(longest_parentheses("(()"), 2);
        assert_eq!(longest_parentheses(")()"), 2);

        assert_eq!(longest_parentheses("((())"), 4);
        assert_eq!(longest_parentheses("()()"), 4);
        assert_eq!(longest_parentheses("(())"), 4);
        assert_eq!(longest_parentheses("()())"), 4);

        assert_eq!(longest_parentheses("((()))"), 6);
        assert_eq!(longest_parentheses("(((())))"), 8);
        assert_eq!(longest_parentheses("(((()))"), 6);
    }
}
