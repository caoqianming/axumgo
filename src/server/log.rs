use std::io;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt, EnvFilter};
use tracing_appender::non_blocking::WorkerGuard;

pub fn init() -> WorkerGuard{
    let is_dev = std::env::var("RUST_ENV").map_or(true, |v| v == "development");

    let filter_layer = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(if is_dev { "info" } else { "warning" }));

    let file_appender = rolling::daily("./logs", "log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(
            fmt::layer()
                .with_target(true) // 可以选择是否在日志中包含target信息
                .with_ansi(true) // 是否添加 ANSI 颜色代码
                .with_writer(io::stdout),
        ) // 控制台输出
        .with(
            fmt::layer()
                .with_writer(non_blocking) // 文件输出
                .with_ansi(false),
        ) // ANSI 颜色在文件中通常不需要
        .init();
    _guard
}
