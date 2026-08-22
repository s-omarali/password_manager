use zeroize::ZeroizeOnDrop;

// derive tells compiler to automatically generate default trait implementations for a custom struct
#[derive(ZeroizeOnDrop)] // according to docs this struct will be zeroized on drop  
pub struct SecureBuffer 
{
    data: Vec<u8> // u8 bc decrypted text or derived key is binary data. 8 bits = 1 byte
}
// zeros the entire vector and then deallocates with Drop

impl SecureBuffer 
{
    pub fn new(size:usize) -> Self // usize = size_t in c++
    {
        let data = vec![0u8; size]; // 0u8 means u8 set to 0. creating vector of size 'size' all 0 u8s
        Self {data}
    }

    pub fn as_slice(&self) -> &[u8] // returning &[u8] (a slice) instead of the vector. this is deref coercion.
    // we use deref coercion because we dont want to return a vector that can grow/shrink if its a fixed-size secret. a slice cannot change
    {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8]
    {
        &mut self.data
    }

    pub fn len(&self) -> usize
    {
        self.data.len()
    }
}
