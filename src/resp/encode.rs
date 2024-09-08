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
    fn encode(self) -> Vec<u8> {
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

// simple string: "+OK\r\n"
impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

// error: "-Error message\r\n"
impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}

// integer: ":[<+|->]<value>\r\n"
impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self.is_negative() { "" } else { "+" };
        format!(":{}{}\r\n", sign, self).into_bytes()
    }
}

// bulk string: "$<length>\r\n<data>\r\n"
impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len() + 16);

        buf.extend_from_slice(&format!("${}\r\n", self.len()).into_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

// null bulk string: "$-1\r\n"
impl RespEncode for RespNullBulkString {
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }
}

// array: "*<number-of-elements>\r\n<element-1>...<element-n>"
//         - "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);

        let prefix = format!("*{}\r\n", self.0.len()).into_bytes();
        buf.extend_from_slice(&prefix);

        for frame in self.0 {
            // 这里的 encode 方法由 enum dispatch 路由
            buf.extend_from_slice(&frame.encode());
        }

        buf
    }
}

// null array: "*-1\r\n"
impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        b"*-1\r\n".to_vec()
    }
}

// null: "_\r\n"
impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}

//boolean: "#<t|f>\r\n"
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        let result = if self { "t" } else { "f" };
        format!("#{}\r\n", result).into_bytes()
    }
}

// double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(32);

        let ret = if self.abs() > 1e+8 || self.abs() < 1e-8 {
            format!(",{:+e}\r\n", self)
        } else {
            let sign = if self < 0.0 { "" } else { "+" };
            format!(",{}{}\r\n", sign, self)
        };

        buf.extend_from_slice(&ret.into_bytes());
        buf
    }
}

//  map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
impl RespEncode for RespMap {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);

        buf.extend_from_slice(&format!("%{}\r\n", self.len()).into_bytes());
        for (key, value) in self.0 {
            buf.extend_from_slice(&SimpleString::new(key).encode());
            buf.extend_from_slice(&value.encode());
        }

        buf
    }
}

// set: "~<number-of-elements>\r\n<element-1>...<element-n>"
impl RespEncode for RespSet {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);

        buf.extend_from_slice(&format!("~{}\r\n", self.len()).into_bytes());

        for frame in self.0 {
            buf.extend_from_slice(&frame.encode());
        }

        buf
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
