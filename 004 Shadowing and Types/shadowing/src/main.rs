fn main() {
    let mut x = 1.2;
    println!("{x}");
    
    {
        let x = 1;
        println!("{x}");
    }
    
    println!("{x}");
}
