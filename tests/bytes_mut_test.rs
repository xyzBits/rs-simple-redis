use bytes::{Buf, BufMut, BytesMut};

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

#[test]
fn test_peekable() {
    let xs = [1, 2, 3];

    // 返回一个迭代器
    let mut iter = xs.iter().peekable();

    // peek 相当于 next
    assert_eq!(iter.peek(), Some(&&1));

    let xs = [1, 2, 3];

    let mut iter = xs.iter().peekable();

    assert_eq!(iter.peek_mut(), Some(&mut &1));
}

#[test]
fn test_bytes_mut_start_with() {
    let mut bytes = BytesMut::new();
    bytes.put_slice(b"hello");

    let res = bytes.starts_with(&"h".as_bytes());
    println!("{}", res);
}

#[test]
fn test_bytes_mut_split() {
    let mut a = BytesMut::from(&b"hello world"[..]);

    // Splits the buffer into two at the given index.
    // Afterwards self contains elements [at, len), and the returned BytesMut contains elements [0, at).
    let b = a.split_to(5);

    println!("{:?}", String::from_utf8_lossy(&a[..]));

    println!("{}", String::from_utf8_lossy(&b));
}

#[test]
fn test_bytes_mut_advance() {
    let mut buf = BytesMut::from(b"hello world");

    buf.advance(3);
}
