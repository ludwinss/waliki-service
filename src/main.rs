mod adapters;
mod context;
mod platform;
use platform::logger;

fn main() -> std::io::Result<()> {
    logger::init();
    adapters::http::actix::main()
}
