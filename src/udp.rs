use crate::utils::math::ones_complement;
use crate::ip::IpV4;

pub struct UdpHeader {
    source_port: u16,
    destination_port: u16,
    length: u16,
    checksum: u16,
}

impl UdpHeader {
    pub fn new(source_port: u16, destination_port: u16, length: u16) -> Self {
        let sum = source_port + destination_port + length;

        let checksum = ones_complement(sum);

        UdpHeader { source_port, destination_port, length, checksum }
    }
}

impl From<Vec<u8>> for UdpHeader {
    fn from(_: Vec<u8>) -> Self {
        todo!()
    }
}

impl Into<Vec<u8>> for UdpHeader {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}

pub struct Udp {
    header: UdpHeader,
    data: Vec<u8>,
    ip: IpV4,
}

impl Udp {
    fn new(source_port: u16,
            destination_port: u16,
            length: u16, data: Vec<u8>, 
            ip: IpV4) -> Self {
        let header = UdpHeader::new(source_port, destination_port, length);

        Udp {header, data, ip}
    }

    fn new_with_header(header: UdpHeader, data: Vec<u8>, ip: IpV4) -> Self {
        Udp {header, data, ip}
    }
}

impl From<Vec<u8>> for Udp {
    fn from(v: Vec<u8>) -> Self {
        let ipv4_part = v[0..20].to_vec();

        let ip: IpV4 = ipv4_part.into();

        let header_part = v[20..28].to_vec();

        let header: UdpHeader = header_part.into();

        let data = v[28..].to_vec();

        Self::new_with_header(header, data, ip)
    }
}