use crate::cmd::{CommandError, CommandExecutor, Get, Set, RESP_OK};
use crate::{Backend, RespArray, RespFrame, RespNull};

impl CommandExecutor for Get {
    fn execute(self, backend: &Backend) -> RespFrame {
        backend
            .get(&self.key)
            .unwrap_or_else(|| RespFrame::Null(RespNull))
    }
}

impl CommandExecutor for Set {
    fn execute(self, backend: &Backend) -> RespFrame {
        backend.set(self.key, self.value);
        RESP_OK.clone()
    }
}

impl TryFrom<RespArray> for Get {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<RespArray> for Set {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        todo!()
    }
}
