#![allow(unused)]

fn main() {
    struct Data {
        value: i32,
    }

    impl Drop for Data {
        fn drop(&mut self) {
            println!("Calling drop {}", self.value);
        }
    }

    {
        let d0 = Data { value: 0 };
    }

    let d1 = Data { value: 1 };
    drop(d1);

    let d2 = Data { value: 2 };
}
