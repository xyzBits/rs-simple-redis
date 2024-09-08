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
    BulkString, RespArray, RespEncode, RespFrame, RespMap, RespNull, RespNullArray,
    RespNullBulkString, RespSet, SimpleError, SimpleString,
};

/// encode 结构体数据 --> 字节数组

const BUF_CAP: usize = 1024 * 4;

/// #[enum_dispatch(RespEncode)] 这个注解所生成的代码
/*impl RespEncode for RespFrame {
    fn encode(&self) -> Vec<u8> {
        match self {
            RespFrame::SimpleString(inner) => inner.encode(),
            RespFrame::Error(inner) => inner.encode(),
            RespFrame::Integer(inner) => inner.encode(),
            RespFrame::BulkString(inner) => inner.encode(),
            RespFrame::NullBulkString(inner) => inner.encode(),
            RespFrame::Array(inner) => inner.encode(),
            RespFrame::NullArray(inner) => inner.encode(),
            RespFrame::Null(inner) => inner.encode(),
            RespFrame::Boolean(inner) => inner.encode(),
            RespFrame::Double(inner) => inner.encode(),
            RespFrame::Map(inner) => inner.encode(),
            RespFrame::Set(inner) => inner.encode(),
        }
    }
}*/

impl RespEncode for SimpleString {
    fn encode(&self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

impl RespEncode for SimpleError {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl RespEncode for i64 {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl RespEncode for BulkString {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl RespEncode for RespNullBulkString {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl RespEncode for RespArray {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl RespEncode for RespNullArray {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl RespEncode for RespNull {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl RespEncode for bool {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl RespEncode for f64 {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl RespEncode for RespMap {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl RespEncode for RespSet {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RespFrame;

    #[test]
    fn test_string_to_vec_u8() {
        let data = "hello world".to_owned();

        let _bytes = data.as_bytes();

        let _byte_array = data.into_bytes();
    }

    #[test]
    fn test_simple_string_encode() {
        let frame: RespFrame = SimpleString::new("OK".to_string()).into();

        // 这个 decode 由 enum_dispatch 实现提供
        assert_eq!(frame.encode(), b"+OK\r\n");

        let simple_string = SimpleString::new("OK".to_string());

        assert_eq!(simple_string.encode(), b"+OK\r\n");
    }
}
