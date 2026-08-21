fn loop_and_return_vec(max_len: i32) -> Vec<i32>
{
    let mut vector = vec![];
    for i in 1..max_len
    {
        vector.push(i);

    }
    return vector;
} 


fn main()
{
    let result = loop_and_return_vec(5);
    println!("{:?}", result)
}