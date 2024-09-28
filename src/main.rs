use rs_simple_redis::{stream_handler, Backend};
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber::fmt::Subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Rust 日志系统中，这个函数主要用于初始化一个简单的、基于文本的日志输出，将配置 tracing crate 的日志记录器
    // 以便将日志信息输出到 标准输出，通常是终端
    // tracing_subscriber::fmt::init();
    // 初始化日志记录器，设置日志级别为 debug
    let filter = tracing_subscriber::filter::LevelFilter::DEBUG;
    let subscriber = Subscriber::builder()
        .with_max_level(filter)
        .with_target(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let addr = "0.0.0.0:6379";
    info!("Simple-Redis-Server is listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;

    let backend = Backend::new();

    loop {
        let (stream, remote_socket_addr) = listener.accept().await?;
        info!("Accepted connection from {}", remote_socket_addr);
        let cloned_backend = backend.clone();

        tokio::spawn(async move {
            match stream_handler(stream, cloned_backend).await {
                Ok(_) => {
                    info!("Connection from {} exited", remote_socket_addr);
                }

                Err(e) => {
                    warn!("handle error for {}, error: {}", remote_socket_addr, e);
                }
            }
        });
    }
}
