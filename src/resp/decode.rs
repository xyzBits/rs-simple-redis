/*
- 如何解析 Frame
    - simple string: "+OK\r\n"
    - error: "-Error message\r\n"
    - bulk error: "!<length>\r\n<error>\r\n"
    - integer: ":[<+|->]<value>\r\n"
    - bulk string: "$<length>\r\n<data>\r\n"
    - null bulk string: "$-1\r\n"
    - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
        - "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
    - null array: "*-1\r\n"
    - null: "_\r\n"
    - boolean: "#<t|f>\r\n"
    - double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
    - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
    - set: "~<number-of-elements>\r\n<element-1>...<element-n>"
 */

use crate::{RespDecode, RespEncode, RespError, RespFrame, SimpleString};
use anyhow::Result;
use bytes::BytesMut;

impl RespDecode for RespFrame {
    const PREFIX: &'static str = "";

    fn decode(buf: &mut BytesMut) -> std::result::Result<RespFrame, RespError> {
        todo!()
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        todo!()
    }
}

impl RespDecode for SimpleString {
    const PREFIX: &'static str = "+";

    fn decode(buf: &mut BytesMut) -> std::result::Result<RespFrame, RespError> {
        todo!()
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
