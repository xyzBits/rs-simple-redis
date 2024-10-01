use crate::cmd::{
    extract_args, validate_command, CommandError, CommandExecutor, HGet, HGetAll, HSet, RESP_OK,
};
use crate::{Backend, RespArray, RespFrame, RespMap};

impl CommandExecutor for HGet {
    fn execute(self, backend: &Backend) -> RespFrame {
        backend
            .hget(&self.key, &self.field)
            .unwrap_or_else(|| RespFrame::Null(crate::RespNull))
    }
}

impl CommandExecutor for HGetAll {
    fn execute(self, backend: &Backend) -> RespFrame {
        let hmap = backend.hmap.get(&self.key);

        match hmap {
            None => RespArray::new([]).into(),
            Some(hmap) => {
                let mut map = RespMap::new();

                for item in hmap.iter() {
                    let key = item.key().to_owned();
                    map.insert(key, item.value().to_owned());
                }
                map.into()
            }
        }
    }
}

impl CommandExecutor for HSet {
    fn execute(self, backend: &Backend) -> RespFrame {
        backend.hset(self.key, self.field, self.value);
        RESP_OK.clone()
    }
}

impl TryFrom<RespArray> for HGet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hget"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();

        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field))) => Ok(HGet {
                key: String::from_utf8(key.0)?,
                field: String::from_utf8(field.0)?,
            }),
            _ => Err(CommandError::InvalidArgument(
                "Invalid key or field".to_string(),
            )),
        }
    }
}

impl TryFrom<RespArray> for HGetAll {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hgetall"], 1)?;

        let mut args = extract_args(value, 1)?.into_iter();

        match args.next() {
            Some(RespFrame::BulkString(key)) => Ok(HGetAll {
                key: String::from_utf8(key.0)?,
                sort: false,
            }),
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

impl TryFrom<RespArray> for HSet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hset"], 3)?;

        let mut args = extract_args(value, 1)?.into_iter();

        match (args.next(), args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field)), Some(value)) => {
                Ok(HSet {
                    key: String::from_utf8(key.0)?,
                    field: String::from_utf8(field.0)?,
                    value,
                })
            }

            _ => Err(CommandError::InvalidArgument(
                "Invalid key, field or value".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Backend, RespDecode, RespMap};
    use crate::{RespArray, RespFrame};
    use bytes::BytesMut;

    use crate::cmd::{CommandExecutor, HGet, HGetAll, HSet, RESP_OK};
    use anyhow::Result;

    #[test]
    fn test_hget_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();

        // cmd = hget map hello
        buf.extend_from_slice(b"*3\r\n$4\r\nhget\r\n$3\r\nmap\r\n$5\r\nhello\r\n");

        let resp_array = RespArray::decode(&mut buf)?;

        let hget_cmd: HGet = resp_array.try_into()?;

        assert_eq!(hget_cmd.key, "map");
        assert_eq!(hget_cmd.field, "hello");

        Ok(())
    }

    #[test]
    fn test_hget_all_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        // cmd = hgetall map
        buf.extend_from_slice(b"*2\r\n$7\r\nhgetall\r\n$3\r\nmap\r\n");

        let resp_array = RespArray::decode(&mut buf)?;
        let hget_all_cmd: HGetAll = resp_array.try_into()?;

        assert_eq!(hget_all_cmd.key, "map");

        Ok(())
    }

    #[test]
    fn test_hset_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        // cmd = hset map hello world
        buf.extend_from_slice(b"*4\r\n$4\r\nhset\r\n$3\r\nmap\r\n$5\r\nhello\r\n$5\r\nworld\r\n");

        let resp_array = RespArray::decode(&mut buf)?;

        let hset_cmd: HSet = resp_array.try_into()?;

        assert_eq!(hset_cmd.key, "map");
        assert_eq!(hset_cmd.field, "hello");
        assert_eq!(hset_cmd.value, RespFrame::BulkString(b"world".into()));

        Ok(())
    }

    #[test]
    fn test_hset_hgetall_commands() -> Result<()> {
        let backend = Backend::new();

        let cmd = HSet {
            key: "map".into(),
            field: "hello".into(),
            value: RespFrame::BulkString(b"world".into()),
        };

        let result = cmd.execute(&backend);
        assert_eq!(result, RESP_OK.clone());

        let cmd = HSet {
            key: "map".into(),
            field: "hello1".into(),
            value: RespFrame::BulkString(b"world1".into()),
        };

        cmd.execute(&backend);

        let cmd = HGet {
            key: "map".into(),
            field: "hello1".into(),
        };

        let result = cmd.execute(&backend);
        assert_eq!(result, RespFrame::BulkString(b"world1".into()));

        let cmd = HGetAll {
            key: "map".into(),
            sort: false,
        };

        let result = cmd.execute(&backend);

        let mut expected = RespMap::new();
        expected.insert("hello".to_string(), RespFrame::BulkString(b"world".into()));
        expected.insert(
            "hello1".to_string(),
            RespFrame::BulkString(b"world1".into()),
        );

        assert_eq!(result, expected.into());
        Ok(())
    }
}
