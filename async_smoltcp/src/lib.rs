use std::net::{IpAddr, SocketAddr};

use smoltcp::wire::{IpAddress, IpEndpoint, IpListenEndpoint};

pub use device::TunDevice;
pub use tcp::TcpStream;
pub use udp::UdpSocket;

mod device;
mod tcp;
mod udp;

pub trait Tun: Clone {
    type Packet: Packet;

    /// Receive data from tun device, if nothing to read WouldBlock will be return.
    fn receive(&self) -> std::io::Result<Option<Self::Packet>>;

    /// Send data to tun device
    fn send(&self, packet: Self::Packet) -> std::io::Result<()>;

    /// Allocate a packet which can hold len bytes data.
    fn allocate_packet(&self, len: usize) -> std::io::Result<Self::Packet>;
}

pub trait Packet {
    fn as_mut(&mut self) -> &mut [u8];
    fn as_ref(&self) -> &[u8];
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

trait TypeConverter {
    type TargetType;
    fn convert(self) -> Self::TargetType;
}

impl TypeConverter for IpEndpoint {
    type TargetType = SocketAddr;
    fn convert(self) -> Self::TargetType {
        SocketAddr::new(self.addr.convert(), self.port)
    }
}

impl TypeConverter for IpAddress {
    type TargetType = IpAddr;
    fn convert(self) -> Self::TargetType {
        match self {
            IpAddress::Ipv4(v4) => IpAddr::V4(v4.0.into()),
            IpAddress::Ipv6(v6) => IpAddr::V6(v6.0.into()),
        }
    }
}

impl TypeConverter for IpListenEndpoint {
    type TargetType = IpEndpoint;
    fn convert(self) -> Self::TargetType {
        IpEndpoint::new(self.addr.unwrap(), self.port)
    }
}
