use crate::cmd::{Command, CommandExecutor};
use crate::{Backend, RespDecode, RespEncode, RespError, RespFrame};
use anyhow::Result;
use bytes::BytesMut;
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder, Framed};
use tracing::info;

#[derive(Debug)]
struct RespFrameCodec;

#[derive(Debug)]
struct RedisRequest {
    frame: RespFrame,
    backend: Backend,
}

#[derive(Debug)]
struct RedisResponse {
    frame: RespFrame,
}

pub async fn stream_handler(stream: TcpStream, backend: Backend) -> Result<()> {
    // 使用 tokio 框架对网络流进行封装
    // Framed 提供了一种方便的方式来处理网络协议
    // stream 代表底层的网络连接
    // RespFrameCodec 定义了如何对数据进行编码和解码
    // 自定义网络协议，使用 Frame 来处理
    let mut framed = Framed::new(stream, RespFrameCodec);

    loop {
        match framed.next().await {
            Some(Ok(frame)) => {
                // 这里已经解析成 RespFrame 了
                info!("Received frame: {:?}", frame);

                let request = RedisRequest {
                    frame,
                    backend: backend.clone(),
                };

                let response = request_handler(request).await?;

                info!("Sending response: {:?}", response);
                // send 方法是 SinkExt 这个 trait 中的
                framed.send(response.frame).await?;
            }
            Some(Err(e)) => return Err(e),
            None => return Ok(()),
        }
    }
}

async fn request_handler(request: RedisRequest) -> Result<RedisResponse> {
    let (frame, backend) = (request.frame, request.backend);
    let cmd = Command::try_from(frame)?;
    info!("Executing command: {:?}", cmd);

    let frame = cmd.execute(&backend);
    Ok(RedisResponse { frame })
}

// 将 frame 转为 字节数组，用来发送出去
impl Encoder<RespFrame> for RespFrameCodec {
    type Error = anyhow::Error;

    fn encode(
        &mut self,
        item: RespFrame,
        dst: &mut BytesMut,
    ) -> std::result::Result<(), Self::Error> {
        let encoded = item.encode();
        dst.extend_from_slice(&encoded);
        Ok(())
    }
}

// 字节数组转为 RespFrame
impl Decoder for RespFrameCodec {
    type Item = RespFrame;
    type Error = anyhow::Error;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> std::result::Result<Option<Self::Item>, Self::Error> {
        info!("decode stream to frame");
        match RespFrame::decode(src) {
            Ok(frame) => Ok(Some(frame)),
            Err(RespError::NotComplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
