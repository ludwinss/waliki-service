mod adapters;
mod context;
mod platform;

fn main() -> std::io::Result<()> {
    adapters::http::actix::main()
}
