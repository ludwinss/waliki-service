use std::{fmt, sync::Once};
use tracing_log::LogTracer;
use tracing_subscriber::{
    EnvFilter, Registry, fmt as ts_fmt,
    layer::{Layer, SubscriberExt},
};

static INIT: Once = Once::new();

#[derive(Clone)]
pub struct LogOptions {
    pub level_directives: Option<String>,
    pub json: bool,
    pub fallback_crates: &'static [(&'static str, &'static str)],
}
impl Default for LogOptions {
    fn default() -> Self {
        Self {
            level_directives: std::env::var("RUST_LOG").ok(),
            json: false,
            fallback_crates: &[
                ("info", ""),
                ("actix_web", "info"),
                ("waliki_service", "debug"),
                ("oidc", "debug"),
            ],
        }
    }
}

pub struct Redact<T>(pub T);
impl<T: fmt::Display> fmt::Display for Redact<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "*****")
    }
}
impl<T: fmt::Debug> fmt::Debug for Redact<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "*****")
    }
}

#[macro_export]
macro_rules! audit {
    ($($arg:tt)*) => {
        ::tracing::event!(target: "audit", ::tracing::Level::INFO, audit=true, $($arg)*);
    }
}

pub fn init(opts: LogOptions) {
    INIT.call_once(|| {
        let _ = LogTracer::builder()
            .with_max_level(log::LevelFilter::Trace)
            .init();

        let env_or_opts = opts.level_directives;

        let filter = match env_or_opts {
            Some(s) if !s.trim().is_empty() => EnvFilter::new(s),
            _ => {
                let mut s = String::new();
                for (target, level) in opts.fallback_crates {
                    if target == &"info" && level.is_empty() {
                        if !s.is_empty() {
                            s.push(',');
                        }
                        s.push_str("info");
                    } else {
                        if !s.is_empty() {
                            s.push(',');
                        }
                        s.push_str(target);
                        s.push('=');
                        s.push_str(level);
                    }
                }
                EnvFilter::new(s)
            }
        };

        let fmt_base = ts_fmt::layer()
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_timer(ts_fmt::time::UtcTime::rfc_3339());

        let fmt_layer = if opts.json {
            fmt_base.json().with_ansi(false).boxed()
        } else {
            fmt_base.boxed()
        };

        let subscriber = Registry::default().with(filter).with(fmt_layer);

        tracing::subscriber::set_global_default(subscriber)
            .expect("failed to set global tracing subscriber");
    });
}
