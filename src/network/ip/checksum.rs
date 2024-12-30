pub fn checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    for i in 0..data.len() / 2 {
        sum += u16::from_be_bytes(data[2 * i..2 * i + 2].try_into().unwrap()) as u32;
    }
    if data.len() % 2 == 1 {
        sum += (data[data.len() - 1] as u32) << 8;
    }
    while sum >> 16 != 0 {
        sum = (sum >> 16) + (sum & 0xffff);
    }
    sum = !sum;
    sum as u16
}

#[cfg(test)]
mod tests {
    #[test]
    fn checksum() {
        let data: [u8; 64] = [
            8, 0, 193, 180, 0, 18, 3, 33, 148, 17, 74, 103, 0, 0, 0, 0, 138, 204, 11, 0, 0, 0, 0,
            0, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
            37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55,
        ];
        let result = super::checksum(&data);
        assert_eq!(64, data.len());
        assert_eq!(0, result);
    }
}
