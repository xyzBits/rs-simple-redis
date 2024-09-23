use crate::cmd::{CommandExecutor, Get, Set, RESP_OK};
use crate::{Backend, RespFrame, RespNull};

impl CommandExecutor for Get {
    fn execute(self, backend: &Backend) -> RespFrame {
        backend.get(&self.key).unwrap_or_else(|| RespFrame::Null(RespNull))
    }
}


impl CommandExecutor for Set {
    fn execute(self, backend: &Backend) -> RespFrame {
        backend.set(self.key, self.value);
        RESP_OK.clone()
    }
}