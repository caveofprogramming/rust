fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    let mut iter = numbers.iter();

    let num1 = iter.next();
    let num2 = iter.next();

    println!("First number: {:?}", num1.unwrap());
    println!("Second number: {:?}", num2.unwrap());

    for number in iter {
        println!("{}", number);
    }

    let sum1 = numbers.iter().sum::<i32>();
    let sum2:i32 = numbers.iter().sum();
    println!("Sum1: {}", sum1);
    println!("Sum2: {}", sum2);

    let count = numbers.iter().count();
    println!("Count: {}", count);

    for num in numbers.iter_mut() {
        *num *= 10;
    }

    println!("Numbers after mutation: {:?}", numbers);

    let mapped:Vec<_> = numbers.iter().map(|x| x + 100).collect();
    println!("Mapped: {:?}", mapped);

    let filtered:Vec<_> = numbers.iter().filter(|x| (*x / 10) % 2 == 0).collect();
    println!("Filtered: {:?}", filtered);
}
