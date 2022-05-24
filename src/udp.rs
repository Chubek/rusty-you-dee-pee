use crate::ip::IpV4;
use crate::utils::conv::{convert_bytes_to_u16, convert_u16_to_bytes};
use crate::utils::math::ones_complement;

#[derive(Clone)]
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

        UdpHeader {
            source_port,
            destination_port,
            length,
            checksum,
        }
    }

    pub fn from(v: Vec<u8>) -> Self {
        let source_port = convert_bytes_to_u16(v[0..2].to_vec());
        let destination_port = convert_bytes_to_u16(v[2..4].to_vec());
        let length = convert_bytes_to_u16(v[4..6].to_vec());
        let checksum = convert_bytes_to_u16(v[6..8].to_vec());

        UdpHeader {
            source_port,
            destination_port,
            length,
            checksum,
        }
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        let b = {
            let mut v = Vec::<u8>::new();

            v.extend(convert_u16_to_bytes(self.source_port));
            v.extend(convert_u16_to_bytes(self.destination_port));
            v.extend(convert_u16_to_bytes(self.length));
            v.extend(convert_u16_to_bytes(self.checksum));

            v
        };

        b
    }
}

impl From<Vec<u8>> for UdpHeader {
    fn from(v: Vec<u8>) -> Self {
        Self::from(v)
    }
}

impl Into<Vec<u8>> for UdpHeader {
    fn into(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl From<(String, String, usize)> for UdpHeader {
    fn from(item: (String, String, usize)) -> Self {
        let source_port = item.0.parse::<u16>().unwrap();
        let destination_port = item.1.parse::<u16>().unwrap();
        let length = item.2 as u16;

        Self::new(source_port, destination_port, length)
    }
}

pub struct Udp {
    header: UdpHeader,
    data: Vec<u8>,
    ip: IpV4,
}

impl Udp {
    pub fn new(
        source_port: u16,
        destination_port: u16,
        length: u16,
        data: Vec<u8>,
        ip: IpV4,
    ) -> Self {
        let header = UdpHeader::new(source_port, destination_port, length);

        Udp { header, data, ip }
    }

    pub fn new_with_header(header: UdpHeader, data: Vec<u8>, ip: IpV4) -> Self {
        Udp { header, data, ip }
    }

    pub fn new_with_addr(source_ip: String, dest_ip: String, data: Vec<u8>) -> Self {
        let (
                source_addr, 
                source_port, 
                dest_addr, 
                dest_port
            ) = {
            let mut sip_split = source_ip.split(":");
            let mut dip_split = dest_ip.split(":");

            (
                sip_split.next().unwrap().to_string(),
                sip_split.next().unwrap().to_string(),
                dip_split.next().unwrap().to_string(),
                dip_split.next().unwrap().to_string()
            )
        };

        let length = data.len();

        let header = (source_port, dest_port, length).into();

        let ip = (source_addr, dest_addr, length + 8).into();

        Udp { header, data, ip }
    }

    pub fn from(v: Vec<u8>) -> Self {
        let ipv4_part = v[0..20].to_vec();

        let ip: IpV4 = ipv4_part.into();

        let header_part = v[20..28].to_vec();

        let header: UdpHeader = header_part.into();

        let data = v[28..].to_vec();

        Self::new_with_header(header, data, ip)
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        let b = {
            let ip_bytes: Vec<u8> = self.ip.clone().into();
            let header_bytes: Vec<u8> = self.header.clone().into();

            let mut v = Vec::<u8>::new();

            v.extend(ip_bytes);
            v.extend(header_bytes);
            v.extend(self.data.clone());

            v
        };

        b
    }
}

impl From<Vec<u8>> for Udp {
    fn from(v: Vec<u8>) -> Self {
        Self::from(v)
    }
}

impl Into<Vec<u8>> for Udp {
    fn into(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl From<(String, String, Vec<u8>)> for Udp {
    fn from(item: (String, String, Vec<u8>)) -> Self {
        Self::new_with_addr(item.0, item.1, item.2)
    }
}