mod adapters;
mod platform;

fn main() -> std::io::Result<()> {
    adapters::http::actix::main()
}
