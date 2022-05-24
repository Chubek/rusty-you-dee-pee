use crate::utils::conv::*;

#[repr(u8)]
pub enum DSCP {
    Default = 0,
    CS1 = 8,
    AF11 = 10,
    AF12 = 12,
    AF13 = 14,
    CS2 = 16,
    AF21 = 18,
    AF22 = 20,
    AF23 = 22,
    CS3 = 24,
    AF31 = 26,
    AF32 = 28,
    AF33 = 30,
    CS4 = 32,
    AF41 = 34,
    AF42 = 36,
    AF43 = 38,
    CS5 = 40,
    EF = 46,
    CS6 = 48,
    CS7 = 56,
}

impl From<u8> for DSCP {
    fn from(u: u8) -> Self {
        match u {
            0 => Self::Default,
            8 => Self::CS1,
            10 => Self::AF11,
            12 => Self::AF12,
            14 => Self::AF13,
            16 => Self::CS2,
            18 => Self::AF21,
            20 => Self::AF22,
            22 => Self::AF23,
            24 => Self::CS3,
            26 => Self::AF31,
            28 => Self::AF32,
            30 => Self::AF33,
            32 => Self::CS4,
            34 => Self::AF41,
            36 => Self::AF42,
            38 => Self::AF43,
            40 => Self::CS5,
            46 => Self::EF,
            48 => Self::CS6,
            56 => Self::CS7,
            _ => Self::Default,
        }
    }
}

impl Into<u8> for DSCP {
    fn into(self) -> u8 {
        self as u8
    }
}

pub struct IpV4Addr {
    raw: u32,
    fixed: String,
}

impl IpV4Addr {
    pub fn from(u32_bytes: Vec<u8>) -> Self {
        let raw = convert_bytes_to_u32(u32_bytes.clone());
        let fixed = convert_bytes_to_ipv4_addr(u32_bytes);

        IpV4Addr { raw, fixed }
    }

    pub fn from_string(ip: String) -> Self {
        let fixed = ip.clone();
        let raw = convert_ipv4_addr_to_u32(ip);

        IpV4Addr { fixed, raw }
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        let b = convert_u32_to_bytes(self.raw);

        b
    }
}

impl From<Vec<u8>> for IpV4Addr {
    fn from(b: Vec<u8>) -> Self {
        Self::from(b)
    }
}

impl Into<Vec<u8>> for IpV4Addr {
    fn into(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl From<String> for IpV4Addr {
    fn from(ip: String) -> Self {
        Self::from_string(ip)
    }
}

impl Into<String> for IpV4Addr {
    fn into(self) -> String {
        self.fixed
    }
}

pub struct Flag {
    more_fragment: bool,
    fragment_offset: u8,
}

impl Flag {
    pub fn from(v: Vec<u8>) -> Self {
        let flag_bit = v[0];
        let fragment_offset = v[1];

        let more_fragment = match flag_bit {
            1 => true,
            2 => false,
            _ => false,
        };

        Flag {
            more_fragment,
            fragment_offset,
        }
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        let flag_byte = match self.more_fragment {
            true => 1,
            false => 2,
        };

        vec![flag_byte, self.fragment_offset]
    }
}

impl From<Vec<u8>> for Flag {
    fn from(v: Vec<u8>) -> Self {
        Self::from(v)
    }
}

impl Into<Vec<u8>> for Flag {
    fn into(self) -> Vec<u8> {
        self.into_bytes()
    }
}

pub struct IpV4 {
    version: u8,
    dscp: DSCP,
    length: u16,
    identification: u16,
    flag: Flag,
    ttl: u8,
    protocol_code: u8,
    checksum: u16,
    source_addr: IpV4Addr,
    destination_addr: IpV4Addr,
}

impl IpV4 {
    pub fn new(
        version: u8,
        dscp: DSCP,
        length: u16,
        identification: u16,
        flag: Flag,
        ttl: u8,
        protocol_code: u8,
        checksum: u16,
        source_addr: IpV4Addr,
        destination_addr: IpV4Addr,
    ) -> Self {
        IpV4 {
            version,
            dscp,
            length,
            identification,
            flag,
            ttl,
            protocol_code,
            checksum,
            source_addr,
            destination_addr,
        }
    }

    pub fn from(v: Vec<u8>) -> Self {
        let version = v[0];
        let dscp_part = v[1];
        let length_part = v[2..4].to_vec();
        let id_part = v[4..6].to_vec();
        let flags_part = v[6..8].to_vec();
        let ttl = v[8];
        let protocol_code = v[9];
        let checksum_part = v[10..12].to_vec();
        let source_address_part = v[12..16].to_vec();
        let dest_address_part = v[16..20].to_vec();

        let dscp: DSCP = dscp_part.into();
        let length = convert_bytes_to_u16(length_part);
        let identification = convert_bytes_to_u16(id_part);
        let flag: Flag = flags_part.into();
        let checksum = convert_bytes_to_u16(checksum_part);
        let source_addr: IpV4Addr = source_address_part.into();
        let destination_addr: IpV4Addr = dest_address_part.into();

        IpV4 {
            version,
            dscp,
            length,
            identification,
            flag,
            ttl,
            protocol_code,
            checksum,
            source_addr,
            destination_addr,
        }
    }
}

impl From<Vec<u8>> for IpV4 {
    fn from(_: Vec<u8>) -> Self {
        todo!()
    }
}

impl Into<Vec<u8>> for IpV4 {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}
