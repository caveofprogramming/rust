fn main() {
    /*
     * Struct with generics
     * impl with specific and generic types
     * Function with generics (add)
     */

    struct Values<T> {
        x: T,
        y: T,
    }

    impl Values<i32> {
        fn sum(&self)->i32 {
            return self.x + self.y;
        }
    }

    impl<T> Values<T> {
        fn x(&self)->&T {
           return &self.x; 
        }
    }

    
    let values = Values { x: 10, y: 11 };
    
    println!("x: {}", values.x());
    let result = values.sum();
    println!("{result}");

    let total = sum(5, 6);
    println!("{total}");

    let total = sum(1.1, 2.2);
    println!("{total}");
}

fn sum<T: std::ops::Add<Output = T>>(v1: T, v2: T) -> T {
    return v1 + v2;
}
