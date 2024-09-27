mod decode;
mod encode;

use bytes::BytesMut;
use enum_dispatch::enum_dispatch;
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};
use thiserror::Error;

// trait 上也要注明 enum dispatch
#[enum_dispatch]
pub trait RespEncode {
    // 为什么要把 self 直接 consume
    fn encode(self) -> Vec<u8>;
}

/// decode 将字节数据转为 需要的数据结构
/// 表示一个类型的大小在编译期是已知的，也就是说，编译器可以在编译时确定该类型所占的内存空间
/// 实现 RespDecode 的 类型必须有固定的大小，意味着实现这个 trait 的类型不能是动态大小的类型，例如 Vec<T> String
pub trait RespDecode: Sized {
    const PREFIX: &'static str;

    // 拿到一个 bytesMut，然后对里面的数据进行 decode
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;

    fn expect_length(buf: &[u8]) -> Result<usize, RespError>;
}

/// anyhow 帮你自动 convert error
/// this error 灵活的转换 error
#[derive(Debug, Error, PartialEq, Eq)]
pub enum RespError {
    #[error("Invalid frame: {0}")]
    InvalidFrame(String),

    #[error("Invalid frame type: {0}")]
    InvalidFrameType(String),

    #[error("Invalid frame length: {0}")]
    InvalidFrameLength(isize),

    #[error("Frame is not complete")]
    NotComplete,

    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Parse float error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}

// 在 enum 上注明你要使用哪个 trait ，然后 enum 的成员都要实现 这个 trait
// enum dispatch 会为 enum 中的成员实现 from 和 into
// 枚举中的这些成员的 结构，都要实现 RespEncode，否则 编译无法通过
#[enum_dispatch(RespEncode)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RespFrame {
    SimpleString(SimpleString),

    Error(SimpleError),

    Integer(i64),

    BulkString(BulkString),

    NullBulkString(RespNullBulkString),

    Array(RespArray),

    NullArray(RespNullArray),

    Null(RespNull),

    Boolean(bool),

    Double(f64),

    Map(RespMap),

    Set(RespSet),
}

// impl RespFrame {
//     pub fn encode(&self) -> Vec<u8> {
//         todo!()
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct SimpleString(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct SimpleError(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct BulkString(pub(crate) Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct RespNull;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespArray(pub(crate) Vec<RespFrame>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct RespNullArray;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct RespNullBulkString;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespMap(BTreeMap<String, RespFrame>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespSet(Vec<RespFrame>);

// 我们要直接访问里面包裹的数据，所以需要 deref
impl Deref for SimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for SimpleError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespMap {
    type Target = BTreeMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 实现了 deref 只能使用 immutable deref
impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for RespSet {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SimpleString {
    pub fn new(s: impl Into<String>) -> SimpleString {
        SimpleString(s.into())
    }
}

impl SimpleError {
    pub fn new(s: impl Into<String>) -> SimpleError {
        SimpleError(s.into())
    }
}

impl BulkString {
    pub fn new(s: impl Into<Vec<u8>>) -> BulkString {
        BulkString(s.into())
    }
}

impl RespArray {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> RespArray {
        RespArray(s.into())
    }
}

impl RespMap {
    pub fn new() -> RespMap {
        RespMap(BTreeMap::new())
    }
}

impl Default for RespMap {
    fn default() -> Self {
        RespMap::new()
    }
}

impl RespSet {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> RespSet {
        RespSet(s.into())
    }
}

impl From<&str> for SimpleString {
    fn from(s: &str) -> Self {
        SimpleString(s.to_string())
    }
}

impl From<&str> for RespFrame {
    fn from(s: &str) -> Self {
        SimpleString(s.to_string()).into()
    }
}

impl From<&str> for SimpleError {
    fn from(s: &str) -> Self {
        SimpleError(s.to_string())
    }
}

impl From<&str> for BulkString {
    fn from(s: &str) -> Self {
        BulkString(s.as_bytes().to_vec())
    }
}

impl From<&[u8]> for BulkString {
    fn from(s: &[u8]) -> Self {
        BulkString(s.to_vec())
    }
}

impl From<&[u8]> for RespFrame {
    fn from(s: &[u8]) -> Self {
        BulkString(s.to_vec()).into()
    }
}

impl<const N: usize> From<&[u8; N]> for BulkString {
    fn from(s: &[u8; N]) -> Self {
        BulkString(s.to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for RespFrame {
    fn from(s: &[u8; N]) -> Self {
        BulkString(s.to_vec()).into()
    }
}

impl AsRef<str> for SimpleString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// 用于将一个类型转换为另一个类型的引用
impl AsRef<[u8]> for BulkString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{RespFrame, SimpleString};

    #[test]
    fn test_convert() {
        let frame: RespFrame = SimpleString::new("tome").into();
    }
}
