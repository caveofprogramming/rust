pub fn factorial(n:i32)->i32 {
    if n == 0 {
        1
    }
    else {
        n * factorial(n-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial1() {
        assert_eq!(factorial(4), 24, "Factorial 4 should be 24");
    }

    #[test]
    fn test_factorial2() {
        assert!(factorial(4)==24);
    }

    #[test]
    #[ignore]
    fn test_factorial3() {
        assert!(factorial(0)==1);
    }

    #[test]
    #[should_panic(expected="massive")]
    fn test_factorial4() {
        assert!(factorial(0)==1);
        panic!("This is a massive panic");
    }

    #[test]
    fn test_factorial5()->Result<(), String> {
        //Err("Bad result".to_string())
        Ok(())
    }
}