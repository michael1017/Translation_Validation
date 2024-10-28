fn main() {
    let mut input = String::new();
    println!("Enter two integers: ");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().expect("Not an integer");
    let b: i32 = iter.next().unwrap().parse().expect("Not an integer");

    let sum = a + b;
    println!("Sum: {}", sum);
}