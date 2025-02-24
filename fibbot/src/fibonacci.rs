fn fibonacci(n: i64) -> i64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tes_fibo() {
        assert!(fibonacci(0) == 0)
    }
    #[test]
    fn test_fibo2() {
        assert!(fibonacci(1) == 1)
    }
    #[test]
    fn tes_fibo3() {
        assert!(fibonacci(5) == 8)
    }
}
