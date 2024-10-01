use crate::{Backend, RespArray, RespError, RespFrame, SimpleString};
use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use thiserror::Error;

mod hmap;
mod map;

// 宏的作用是定义一个静态变量，并且保证这个变量在第一次被使用的时候才会被初始化
// 可以确保在多线程环境下，变量只会被初始化一次，从而避免了竞态条件的发生
// 生命周期与整个程序相同
lazy_static! {
    static ref RESP_OK: RespFrame = SimpleString::new("OK").into();
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("{0}")]
    RespError(#[from] RespError),

    // `?` could not convert error type `FromUtf8Error` to `CommandError`
    // 当编译器返回一个 std::string::FromUtf8Error 时，编译器会自动转换为 Utf8Error
    // {0} 是一个点位符，在发生错误时，将信息填充进来
    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

#[enum_dispatch]
pub trait CommandExecutor {
    fn execute(self, backend: &Backend) -> RespFrame;
}

#[derive(Debug)]
#[enum_dispatch(CommandExecutor)]
pub enum Command {
    Get(Get),

    Set(Set),

    HGet(HGet),

    HSet(HSet),

    HGetAll(HGetAll),
}

#[derive(Debug)]
pub struct Get {
    key: String,
}

#[derive(Debug)]
pub struct Set {
    key: String,
    value: RespFrame,
}

#[derive(Debug)]
pub struct HGet {
    key: String,
    field: String,
}

#[derive(Debug)]
pub struct HSet {
    key: String,
    field: String,
    value: RespFrame,
}

#[derive(Debug)]
pub struct HGetAll {
    key: String,
    sort: bool,
}

#[derive(Debug)]
pub struct Unrecognized;

impl TryFrom<RespFrame> for Command {
    type Error = CommandError;

    fn try_from(value: RespFrame) -> Result<Self, Self::Error> {
        match value {
            RespFrame::Array(array) => array.try_into(),
            _ => Err(CommandError::InvalidCommand(
                "Command mut be an array".to_string(),
            )),
        }
    }
}

// RespArray 是一个 RespFrame 的 vector
impl TryFrom<RespArray> for Command {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        // "get" "hello"
        // let x = b"get";
        // let x1 = "get".as_bytes().as_ref();
        // match x1 {
        //     b"get" => println!("hello"),
        //     _ => println!("world"),
        // };

        match value.first() {
            Some(RespFrame::BulkString(ref cmd)) => match cmd.as_ref() {
                b"get" => Ok(Get::try_from(value)?.into()),
                b"set" => Ok(Set::try_from(value)?.into()),
                b"hget" => Ok(HGet::try_from(value)?.into()),
                b"hset" => Ok(HSet::try_from(value)?.into()),
                b"hgetall" => Ok(HGetAll::try_from(value)?.into()),
                _ => Err(CommandError::InvalidCommand(format!(
                    "Invalid command: {}",
                    String::from_utf8_lossy(cmd.as_ref())
                ))),
            },

            _ => Err(CommandError::InvalidCommand(
                "Command mut have a BulkString as the first argument".to_string(),
            )),
        }
    }
}

impl CommandExecutor for Unrecognized {
    fn execute(self, _: &Backend) -> RespFrame {
        RESP_OK.clone()
    }
}

// "get" "hello"
fn validate_command(
    value: &RespArray,
    names: &[&'static str],
    n_args: usize,
) -> Result<(), CommandError> {
    if value.len() != n_args + names.len() {
        return Err(CommandError::InvalidArgument(format!(
            "{} command must have exactly {} argument",
            names.join(" "),
            n_args
        )));
    }

    for (i, name) in names.iter().enumerate() {
        match value[i] {
            RespFrame::BulkString(ref cmd) => {
                if cmd.as_ref().to_ascii_lowercase() != name.as_bytes() {
                    return Err(CommandError::InvalidCommand(format!(
                        "Invalid command: expected {}, got {}",
                        name,
                        String::from_utf8_lossy(cmd.as_ref())
                    )));
                }
            }
            _ => {
                return Err(CommandError::InvalidCommand(
                    "Command must have a BulkString as the first argument".to_string(),
                ))
            }
        }
    }

    Ok(())
}

fn extract_args(value: RespArray, start: usize) -> Result<Vec<RespFrame>, CommandError> {
    Ok(value.0.into_iter().skip(start).collect::<Vec<RespFrame>>())
}

#[cfg(test)]
mod test {
    use crate::cmd::{Command, CommandExecutor};
    use crate::{Backend, RespArray, RespDecode, RespFrame, RespNull};
    use anyhow::Result;
    use bytes::BytesMut;

    #[test]
    fn test_command() -> Result<()> {
        let mut buf = BytesMut::new();

        // 以 * 号开头，证明里面有多这个 frame，那么整体是一个 RespArray
        // $ 开头的，证明是  bulkString
        buf.extend_from_slice(b"*2\r\n$3\r\nget\r\n$5\r\nhello\r\n");

        let frame = RespArray::decode(&mut buf)?;

        // 把这个 frame 的 array 转成一个 cmd
        // 比如 get hello 里面有两个 frame get 和 hello
        // 从中抽取出 get 已经参数 hello
        let cmd: Command = frame.try_into()?;

        println!("{:?}", cmd);

        let backend = Backend::new();

        let ret = cmd.execute(&backend);

        assert_eq!(ret, RespFrame::Null(RespNull));

        Ok(())
    }
}
