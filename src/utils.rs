pub mod math {
    pub fn ones_complement(u: u16) -> u16 {
        let bin = format!("{:016x}", u);

        let chars_complement = bin
                            .chars()
                            .map(|x| {
                                match x {
                                    '0' => '1',
                                    '1' => '0',
                                    _ => '0',
                                }
                            });

        let u_complement_str = String::from_iter(chars_complement);

        let u_complement = u16::from_str_radix(u_complement_str.as_str(), 2).unwrap();
    
        u_complement
    }
}

pub mod conv {
    pub fn convert_u16_to_bytes(u: u16) -> Vec<u8> {
        let bin = format!("{:016b}", u);

        let (d1, d2) = {
            let bin_chars = bin.chars().collect::<Vec<char>>();

            let d1_i = bin_chars[0..8].iter();
            let d2_i = bin_chars[8..16].iter();

            (String::from_iter(d1_i), String::from_iter(d2_i))
        };

        let el1 = u8::from_str_radix(d1.as_str(), 2).unwrap();
        let el2 = u8::from_str_radix(d2.as_str(), 2).unwrap();

        vec![el1, el2]
    }

    pub fn convert_bytes_to_u16(b: Vec<u8>) -> u16 {
        let b1 = b[0];
        let b2 = b[1];

        let bin = format!("{:08b}{:08b}", b1, b2);

        let u = u16::from_str_radix(bin.as_str(), 2).unwrap();

        u
    }

    pub fn convert_u32_to_bytes(u: u32) -> Vec<u8> {
        let bin = format!("{:032b}", u);

        let (d1, d2, d3, d4) = {
            let bin_chars = bin.chars().collect::<Vec<char>>();

            let d1_i = bin_chars[0..8].iter();
            let d2_i = bin_chars[8..16].iter();
            let d3_i = bin_chars[16..24].iter();
            let d4_i = bin_chars[24..32].iter();

            (   String::from_iter(d1_i),
                String::from_iter(d2_i),
                String::from_iter(d3_i),
                String::from_iter(d4_i)
            )
        };

        let el1 = u8::from_str_radix(d1.as_str(), 2).unwrap();
        let el2 = u8::from_str_radix(d2.as_str(), 2).unwrap();
        let el3 = u8::from_str_radix(d3.as_str(), 2).unwrap();
        let el4 = u8::from_str_radix(d4.as_str(), 2).unwrap();

        vec![el1, el2, el3, el4]
    }

    pub fn convert_bytes_to_u32(b: Vec<u8>) -> u32 {
        let b1 = b[0];
        let b2 = b[1];
        let b3 = b[2];
        let b4 = b[3];

        let bin = format!("{:08b}{:08b}{:08b}{:08b}", b1, b2, b3, b4);

        let u = u32::from_str_radix(bin.as_str(), 2).unwrap();

        u
    }

    pub fn convert_bytes_to_ipv4_addr(v: Vec<u8>) -> String {
        format!("{}.{}.{}.{}", v[0], v[1], v[2], v[3])
    }
}