pub fn factorial(n:u32)->u32 {
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
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial2() {
        assert!(factorial(4) == 24);
    }

    #[test]
    #[ignore]
    fn test_factorial3() {
        assert!(factorial(5) == 120);
    }

    #[test]
    #[should_panic(expected="bad")]
    fn test_factorial4() {
        assert!(factorial(5) == 120);
        panic!("oh no something bad happened");
    }

    #[test]
    fn test_factorial5()->Result<(), String> {
        Ok(())
        //Err("Gone wrong".to_string())
    }
}

