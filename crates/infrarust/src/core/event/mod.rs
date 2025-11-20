use crate::network::packet::Packet;

#[derive(Debug, Clone)]
pub enum GatewayMessage {
    Shutdown,
}

#[derive(Debug, Clone)]
pub enum MinecraftCommunication<T> {
    RawData(Vec<u8>),
    Packet(Packet),
    Shutdown,
    /// Disconnect the client with a custom message.
    /// Used when the upstream server is unreachable during login.
    DisconnectWithMessage(String),
    CustomData(T),
}
