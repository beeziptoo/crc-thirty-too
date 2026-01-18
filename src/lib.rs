// Copyright © 2026 Randy Barlow and Andrew Halle

//! # crc-thirty-too
//!
//! TODO: Make a nice description.
//!
//! This is ported from [RFC 1952](https://www.rfc-editor.org/rfc/rfc1952.html#section-8).

use std::sync::LazyLock;

static FAST_CRC_TABLE: LazyLock<[u32; 256]> = LazyLock::new(|| {
    let mut data = [0_u32; 256];

    for (n, c) in data.iter_mut().enumerate() {
        *c = n as u32;
        for _ in 0..8 {
            if *c % 2 == 0 {
                *c >>= 1;
            } else {
                *c = 0xedb88320 ^ *c >> 1;
            }
        }
    }

    data
});

pub fn update_crc(crc: u32, bytes: &[u8]) -> u32 {
    let mut c = crc ^ u32::MAX;

    for byte in bytes {
        let idx: usize = ((c as usize) ^ (*byte as usize)) & 0xFF;
        c = (*FAST_CRC_TABLE)[idx] ^ (c >> 8);
    }

    c ^ u32::MAX
}

pub fn crc(bytes: &[u8]) -> u32 {
    update_crc(0, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = b"hello world";

        let actual = crc(input);

        // hex(binascii.crc32(b'hello world'))
        let expected = 0xd4a1185;

        assert_eq!(actual, expected);
    }
}
