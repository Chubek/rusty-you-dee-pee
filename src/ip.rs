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

pub struct IpV4Addr {
    raw: u32,
    fixed: String,
}

impl IpV4Addr {
    fn from(raw: u32) -> Self {
        let u32_bytes = convert_u32_to_bytes(raw.clone());
        let fixed = convert_bytes_to_ipv4_addr(u32_bytes);

        IpV4Addr { raw, fixed }
    }
}


pub struct IpV4 {
     version: u8,

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

