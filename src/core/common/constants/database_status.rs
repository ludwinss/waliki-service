pub enum DatabaseStatus {
    Connected,
    Disconnected,
    Error(String),
}
