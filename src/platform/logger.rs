use env_logger::{Builder, Env};
use log::{debug, error, info, trace, warn};
use std::io::Write;

pub fn init() {
    let env = Env::default().filter_or("RUST_LOG", "info");

    let use_color = std::env::var_os("NO_COLOR").is_none();

    Builder::from_env(env)
        .format(move |buf, record| {
            let ts = buf.timestamp_millis();

            let (lvl_open, lvl_close) = if use_color {
                match record.level() {
                    log::Level::Error => ("\x1b[1;31m", "\x1b[0m"),
                    log::Level::Warn => ("\x1b[1;33m", "\x1b[0m"),
                    log::Level::Info => ("\x1b[32m", "\x1b[0m"),
                    log::Level::Debug => ("\x1b[34m", "\x1b[0m"),
                    log::Level::Trace => ("\x1b[35m", "\x1b[0m"),
                }
            } else {
                ("", "")
            };

            writeln!(
                buf,
                "{ts} {lvl_open}{level}{lvl_close} {target} {file}:{line} - {msg}",
                ts = ts,
                level = record.level(),
                lvl_open = lvl_open,
                lvl_close = lvl_close,
                target = record.target(),
                file = record.file().unwrap_or(""),
                line = record.line().unwrap_or(0),
                msg = record.args(),
            )
        })
        .init();
}

#[allow(dead_code)]
pub fn log_info(message: &str) {
    info!("{message}");
}
#[allow(dead_code)]
pub fn log_error(message: &str) {
    error!("{message}");
}
#[allow(dead_code)]
pub fn log_debug(message: &str) {
    debug!("{message}");
}
#[allow(dead_code)]
pub fn log_trace(message: &str) {
    trace!("{message}");
}
#[allow(dead_code)]
pub fn log_warn(message: &str) {
    warn!("{message}");
}
