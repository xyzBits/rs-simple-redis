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

use crate::{
    BulkString, RespArray, RespDecode, RespEncode, RespError, RespFrame, RespMap, RespNull,
    RespNullArray, RespNullBulkString, RespSet, SimpleError, SimpleString,
};
use anyhow::Result;
use bytes::BytesMut;

const CRLF: &[u8] = b"\r\n";
const CRLF_LEN: usize = CRLF.len();

impl RespDecode for RespFrame {
    const PREFIX: &'static str = "";

    fn decode(buf: &mut BytesMut) -> std::result::Result<RespFrame, RespError> {
        // 返回一个迭代器
        let mut iter = buf.iter().peekable();

        match iter.peek() {
            Some(b'+') => {
                let frame = SimpleString::decode(buf)?;
                Ok(frame.into())
            }

            _ => Err(RespError::InvalidFrameType(format!(
                "expect_length: unknown frame type: {:?}",
                buf
            ))),
        }
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

impl RespDecode for SimpleError {
    const PREFIX: &'static str = "-";

    fn decode(buf: &mut BytesMut) -> std::result::Result<RespFrame, RespError> {
        todo!()
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        todo!()
    }
}

impl RespDecode for RespNull {
    const PREFIX: &'static str = "_";

    fn decode(buf: &mut BytesMut) -> std::result::Result<RespFrame, RespError> {
        todo!()
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        todo!()
    }
}

impl RespDecode for RespNullArray {
    const PREFIX: &'static str = "*";

    fn decode(buf: &mut BytesMut) -> std::result::Result<RespFrame, RespError> {
        todo!()
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        todo!()
    }
}

impl RespDecode for RespNullBulkString {
    const PREFIX: &'static str = "$";

    fn decode(buf: &mut BytesMut) -> std::result::Result<RespFrame, RespError> {
        todo!()
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        todo!()
    }
}

impl RespDecode for i64 {
    const PREFIX: &'static str = ":";

    fn decode(buf: &mut BytesMut) -> std::result::Result<RespFrame, RespError> {
        todo!()
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        todo!()
    }
}

impl RespDecode for bool {
    const PREFIX: &'static str = "#";

    fn decode(buf: &mut BytesMut) -> std::result::Result<RespFrame, RespError> {
        todo!()
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        todo!()
    }
}

impl RespDecode for BulkString {
    const PREFIX: &'static str = "$";

    fn decode(buf: &mut BytesMut) -> std::result::Result<RespFrame, RespError> {
        todo!()
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        todo!()
    }
}

impl RespDecode for RespArray {
    const PREFIX: &'static str = "*";

    fn decode(buf: &mut BytesMut) -> std::result::Result<RespFrame, RespError> {
        todo!()
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        todo!()
    }
}

impl RespDecode for f64 {
    const PREFIX: &'static str = ",";

    fn decode(buf: &mut BytesMut) -> std::result::Result<RespFrame, RespError> {
        todo!()
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        todo!()
    }
}

impl RespDecode for RespMap {
    const PREFIX: &'static str = "%";

    fn decode(buf: &mut BytesMut) -> std::result::Result<RespFrame, RespError> {
        todo!()
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        todo!()
    }
}

impl RespDecode for RespSet {
    const PREFIX: &'static str = "~";

    fn decode(buf: &mut BytesMut) -> std::result::Result<RespFrame, RespError> {
        todo!()
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        todo!()
    }
}

fn extract_fixed_data(
    buf: &mut BytesMut,
    expect: &str,
    expect_type: &str,
) -> Result<(), RespError> {
    todo!()
}

fn extract_simple_frame_data(buf: &[u8], prefix: &str) -> Result<usize, RespError> {
    todo!()
}

fn find_crlf(buf: &[u8], nth: usize) -> Option<usize> {
    todo!()
}

fn parse_length(buf: &[u8], prefix: &str) -> Result<(usize, usize), RespError> {
    todo!()
}

fn calc_total_length(buf: &[u8], end: usize, len: usize, prefix: &str) -> Result<usize, RespError> {
    todo!()
}

#[cfg(test)]
mod tests {}
