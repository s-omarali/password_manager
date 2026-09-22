use argon2::Argon2;
// use crate::secure_buffer::SecureBuffer;


// need to turn passworrd into 32 raw bytes. will use that for aead encryption key
pub fn derive_key(password: &[u8], salt: &[u8], output: &mut [u8]) -> Result<(), argon2::Error> 
{
    return Argon2::default().hash_password_into(password, salt, output);
}