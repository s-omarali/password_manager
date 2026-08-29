use vault_core::secure_buffer::SecureBuffer;

fn make_test_buffer(data: &[u8]) -> SecureBuffer
{
    let mut buffer = SecureBuffer::new(data.len());
    buffer.as_mut_slice().copy_from_slice(data);
    buffer
}

#[test]
fn test_secure_buffer_holds_data()
{
    let buf = make_test_buffer(b"Hello, world!");
    assert_eq!(buf.as_slice(), b"Hello, world!");
}

#[test]
fn test_buffer_zeroed_at_creation()
{
    let buf = SecureBuffer::new(10);
    assert_eq!(buf.as_slice(), &[0; 10]);
}