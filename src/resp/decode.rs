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
use bytes::{Buf, BytesMut};
use std::i64;

const CRLF: &[u8] = b"\r\n";
const CRLF_LEN: usize = CRLF.len();

impl RespDecode for RespFrame {
    const PREFIX: &'static str = "";

    fn decode(buf: &mut BytesMut) -> std::result::Result<Self, RespError> {
        // 返回一个迭代器
        let mut iter = buf.iter().peekable();

        match iter.peek() {
            Some(b'+') => {
                let frame = SimpleString::decode(buf)?;
                Ok(frame.into())
            }

            Some(b'-') => {
                let frame = SimpleError::decode(buf)?;
                Ok(frame.into())
            }

            Some(b':') => {
                let frame = i64::decode(buf)?;
                Ok(frame.into())
            }

            Some(b'$') => match RespNullBulkString::decode(buf) {
                Ok(frame) => Ok(frame.into()),
                Err(RespError::NotComplete) => Err(RespError::NotComplete),
                Err(_) => {
                    let frame = BulkString::decode(buf)?;
                    Ok(frame.into())
                }
            },

            Some(b'*') => match RespNullArray::decode(buf) {
                Ok(frame) => Ok(frame.into()),
                Err(RespError::NotComplete) => Err(RespError::NotComplete),
                Err(_) => {
                    let frame = RespArray::decode(buf)?;
                    Ok(frame.into())
                }
            },

            Some(b'_') => {
                let frame = RespNull::decode(buf)?;
                Ok(frame.into())
            }

            Some(b'#') => {
                let frame = bool::decode(buf)?;
                Ok(frame.into())
            }

            Some(b',') => {
                let frame = f64::decode(buf)?;
                Ok(frame.into())
            }

            Some(b'%') => {
                let frame = RespMap::decode(buf)?;
                Ok(frame.into())
            }

            Some(b'~') => {
                let frame = RespSet::decode(buf)?;
                Ok(frame.into())
            }

            _ => Err(RespError::InvalidFrameType(format!(
                "expect_length: unknown frame type: {:?}",
                buf
            ))),
        }
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(b'*') => RespArray::expect_length(buf),
            Some(b'~') => RespSet::expect_length(buf),
            Some(b'%') => RespMap::expect_length(buf),
            Some(b'$') => BulkString::expect_length(buf),
            Some(b':') => i64::expect_length(buf),
            Some(b'+') => SimpleString::expect_length(buf),
            Some(b'-') => SimpleError::expect_length(buf),
            Some(b'#') => bool::expect_length(buf),
            Some(b',') => f64::expect_length(buf),
            Some(b'_') => RespNull::expect_length(buf),

            _ => Err(RespError::NotComplete),
        }
    }
}

impl RespDecode for SimpleString {
    // simple string: "+OK\r\n"
    // 目的是取出 +_\r\n 中间的内容
    const PREFIX: &'static str = "+";

    fn decode(buf: &mut BytesMut) -> std::result::Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;

        // split 后，buf 中包含 [at, len)
        // data 中包含 [0, at)
        let data = buf.split_to(end + CRLF_LEN);

        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);

        Ok(SimpleString::new(s.to_string()))
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN)
    }
}

// -Error message\r\n
impl RespDecode for SimpleError {
    const PREFIX: &'static str = "-";

    fn decode(buf: &mut BytesMut) -> std::result::Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;

        let data = buf.split_to(end + CRLF_LEN);

        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);

        Ok(SimpleError::new(s.to_string()))
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN)
    }
}

// "_\r\n"
impl RespDecode for RespNull {
    const PREFIX: &'static str = "_";

    fn decode(buf: &mut BytesMut) -> std::result::Result<Self, RespError> {
        extract_fixed_data(buf, "_\r\n", "Null")?;
        Ok(RespNull)
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        Ok(3)
    }
}

// *-1\r\n
impl RespDecode for RespNullArray {
    const PREFIX: &'static str = "*";

    fn decode(buf: &mut BytesMut) -> std::result::Result<Self, RespError> {
        extract_fixed_data(buf, "*-1\r\n", "NullArray")?;
        Ok(RespNullArray)
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        Ok(4)
    }
}

// $-1\r\n
impl RespDecode for RespNullBulkString {
    const PREFIX: &'static str = "$";

    fn decode(buf: &mut BytesMut) -> std::result::Result<Self, RespError> {
        extract_fixed_data(buf, "$-1\r\n", "NullBulkString")?;
        Ok(RespNullBulkString)
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        Ok(5)
    }
}

// :[<+|->]<value>\r\n
impl RespDecode for i64 {
    const PREFIX: &'static str = ":";

    fn decode(buf: &mut BytesMut) -> std::result::Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;

        let data = buf.split_to(end + CRLF_LEN);

        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);

        Ok(s.parse()?)
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN)
    }
}

// #<t|f>\r\n
impl RespDecode for bool {
    const PREFIX: &'static str = "#";

    fn decode(buf: &mut BytesMut) -> std::result::Result<Self, RespError> {
        match extract_fixed_data(buf, "#t\r\n", "Bool") {
            Ok(_) => Ok(true),
            Err(RespError::NotComplete) => Err(RespError::NotComplete),
            Err(_) => match extract_fixed_data(buf, "#f\r\n", "Bool") {
                Ok(_) => Ok(false),
                Err(e) => Err(e),
            },
        }
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        Ok(4)
    }
}

// bulk string: "$<length>\r\n<data>\r\n"
impl RespDecode for BulkString {
    const PREFIX: &'static str = "$";

    fn decode(buf: &mut BytesMut) -> std::result::Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;

        let remained = &buf[end + CRLF_LEN..];
        if remained.len() < len + CRLF_LEN {
            return Err(RespError::NotComplete);
        }

        buf.advance(end + CRLF_LEN);

        let data = buf.split_to(len + CRLF_LEN);
        Ok(BulkString::new(data[..len].to_vec()))
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN + end + CRLF_LEN)
    }
}

impl RespDecode for RespArray {
    const PREFIX: &'static str = "*";

    fn decode(buf: &mut BytesMut) -> std::result::Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        let total_len = calc_total_length(buf, end, len, Self::PREFIX)?;

        if buf.len() < total_len {
            return Err(RespError::NotComplete);
        }

        buf.advance(end + CRLF_LEN);

        let mut frames = Vec::with_capacity(len);
        for _ in 0..len {
            frames.push(RespFrame::decode(buf)?);
        }

        Ok(RespArray::new(frames))
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        calc_total_length(buf, end, len, Self::PREFIX)
    }
}

impl RespDecode for f64 {
    const PREFIX: &'static str = ",";

    // double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
    fn decode(buf: &mut BytesMut) -> std::result::Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;

        let data = buf.split_to(end + CRLF_LEN);

        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);

        Ok(s.parse()?)
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN)
    }
}

impl RespDecode for RespMap {
    const PREFIX: &'static str = "%";

    fn decode(buf: &mut BytesMut) -> std::result::Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        let total_len = calc_total_length(buf, end, len, Self::PREFIX)?;

        if buf.len() < total_len {
            return Err(RespError::NotComplete);
        }

        let mut frames = RespMap::new();

        for _ in 0..len {
            let key = SimpleString::decode(buf)?;
            let value = RespFrame::decode(buf)?;
            frames.insert(key.0, value);
        }

        Ok(frames)
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        calc_total_length(buf, end, len, Self::PREFIX)
    }
}

impl RespDecode for RespSet {
    const PREFIX: &'static str = "~";

    fn decode(buf: &mut BytesMut) -> std::result::Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;

        let total_len = calc_total_length(buf, end, len, Self::PREFIX)?;

        if buf.len() < total_len {
            return Err(RespError::NotComplete);
        }

        let mut frames = Vec::new();

        for _ in 0..len {
            frames.push(RespFrame::decode(buf)?);
        }

        Ok(RespSet::new(frames))
    }

    fn expect_length(buf: &[u8]) -> std::result::Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        calc_total_length(buf, end, len, Self::PREFIX)
    }
}

fn extract_fixed_data(
    buf: &mut BytesMut,
    expect: &str,
    expect_type: &str,
) -> Result<(), RespError> {
    if buf.len() < expect.len() {
        return Err(RespError::NotComplete);
    }

    if !buf.starts_with(expect.as_bytes()) {
        return Err(RespError::InvalidFrameType(format!(
            "expect: {}, got: {:?}",
            expect_type, buf
        )));
    }

    buf.advance(expect.len());
    Ok(())
}

fn extract_simple_frame_data(buf: &[u8], prefix: &str) -> Result<usize, RespError> {
    if buf.len() < 3 {
        return Err(RespError::NotComplete);
    }

    if !buf.starts_with(prefix.as_bytes()) {
        return Err(RespError::InvalidFrameType(format!(
            "expected: SimpleString({}), got: {:?}",
            prefix, buf
        )));
    }

    // 如果  find_crlf 返回 None，那么就会返回 RespError::NotComplete 这个 error
    // \r\n 的位置
    let end = find_crlf(buf, 1).ok_or(RespError::NotComplete)?;
    Ok(end)
}

// 遇到一个 \r\n ，就返回 \r 的位置
fn find_crlf(buf: &[u8], nth: usize) -> Option<usize> {
    let mut count = 0;
    for i in 1..buf.len() - 1 {
        if buf[i] == b'\r' && buf[i + 1] == b'\n' {
            count += 1;
            if count == nth {
                return Some(i);
            }
        }
    }

    None
}

fn parse_length(buf: &[u8], prefix: &str) -> Result<(usize, usize), RespError> {
    let end = extract_simple_frame_data(buf, prefix)?;

    let s = String::from_utf8_lossy(&buf[prefix.len()..end]);

    Ok((end, s.parse()?))
}

fn calc_total_length(buf: &[u8], end: usize, len: usize, prefix: &str) -> Result<usize, RespError> {
    let mut total = end + CRLF_LEN;
    let mut data = &buf[total..];

    match prefix {
        "*" | "~" => {
            for _ in 0..len {
                let len = RespFrame::expect_length(data)?;
                data = &data[len..];
                total += len;
            }

            Ok(total)
        }

        "%" => {
            for _ in 0..len {
                let len = SimpleString::expect_length(data)?;

                data = &data[len..];
                total += len;

                let len = RespFrame::expect_length(data)?;
                data = &data[len..];
                total += len;
            }

            Ok(total)
        }

        _ => Ok(len + CRLF_LEN),
    }
}

#[cfg(test)]
mod tests {}
