use bytes::{BufMut, BytesMut};

#[test]
fn test_bytes_mut() {
    let mut buf = BytesMut::with_capacity(64);

    buf.put_u8(b'h');
    buf.put_u8(b'e');
    buf.put(&b"llo"[..]);

    assert_eq!(&buf[..], b"hello");

    println!("{}", String::from_utf8(buf.to_vec()).unwrap());

    let a = buf.freeze();

    let b = a.clone();

    assert_eq!(&a[..], b"hello");
    assert_eq!(&b[..], b"hello");
}
