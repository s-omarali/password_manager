fn main()
{
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("The number is: {}", number);
    }
    println!("{}", numbers.len()) // can still access because we used iter() which borrows the array instead of taking ownership
}