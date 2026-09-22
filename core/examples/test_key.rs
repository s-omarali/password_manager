use vault_core::crypto::derive_key;
fn main()
{
    let password = b"abc123";
    let salt = b"22232323";
    let mut output = [0u8; 32];
    match derive_key(password, salt, &mut output)
    {
        Ok(()) => println!("{:?}", output),
        Err(e) => println!("derviation failed: {:?}", e)
    }
}
