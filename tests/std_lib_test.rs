use enum_dispatch::enum_dispatch;
use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

#[enum_dispatch(Hello)]
#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum Gender {
    Male(Male),
    Female(Female),
}

pub trait Hello {
    fn hello(&self);
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Male(String);

impl Male {
    pub fn new(name: &str) -> Male {
        Male(name.into())
    }
}

impl Deref for Male {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hello for Male {
    fn hello(&self) {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Female {}

impl Hello for Female {
    fn hello(&self) {
        todo!()
    }
}

impl From<Female> for Gender {
    fn from(value: Female) -> Self {
        Gender::Female(value)
    }
}

#[test]
fn test_enum_into() {
    // 实现了还是不行
    // let gender: Gender = Male::new("tom").into();
}

#[test]
fn test_deref_mut() {
    struct DerefMutExample<T> {
        value: T,
    }

    impl<T> Deref for DerefMutExample<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    // 如果没有实现 DerefMut，就无法把 实例声明为 mut 并且 使用 解引用的操作
    impl<T> DerefMut for DerefMutExample<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.value
        }
    }

    let mut x = DerefMutExample { value: 'a' };

    *x = 'b';
    assert_eq!('b', x.value);
}

#[test]
fn test_string_utf8() {
    let origin_string = "hello world \x77";
    let cow = String::from_utf8_lossy(origin_string.as_bytes());

    println!("{}", cow.to_string());

    // 非法字节会用 ? 代替
    let bytes = [0x61, 0x73, 0x63, 0x69, 0xC3, 0xBF]; // ASCII "asci" 后面跟着一个不完整的 UTF-8 序列

    let res = String::from_utf8_lossy(&bytes);
    println!("{}", res.to_string());
}

#[test]
fn test_from_utf8_lossy_cow() {
    let bytes = b"hello world"; // 有效的 utf-8
    let s = String::from_utf8_lossy(bytes);

    // bytes 是一个有效的 utf-8，s 将是一个 Cow::Borrowed 直接引用原始的字节切片
    if let Cow::Borrowed(s) = s {
        println!("s is a slice: {}", s);
    } else {
        println!("s is owned: {}", s)
    }
}

#[test]
fn test_ok_or() {
    let x = Some("foo");
    assert_eq!(x.ok_or(0), Ok("foo"));

    let x: Option<&str> = None;
    assert_eq!(x.ok_or(0), Err(0));
}
