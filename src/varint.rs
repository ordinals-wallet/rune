pub struct VarInt {}

impl VarInt {
    pub fn get_bytes(length: u64) -> Vec<u8> {
        if length <= 0xfc {
            vec![length as u8]
        } else if length <= (1<<16)-1 {
            let mut push1 = vec![0xfd];
            push1.extend((length as u16).to_le_bytes());
            push1
        } else if length <= (1<<32)-1 {
            let mut push2 = vec![0xfe];
            push2.extend((length as u32).to_le_bytes());
            push2
        } else {
            let mut push4 = vec![0xff];
            push4.extend((length as u64).to_le_bytes());
            push4
        }
    }

    pub fn read_bytes(bytes: Vec<u8>) -> (u64, u8) {
        match bytes[0] {
            0xff => (
                u64::from_le_bytes(bytes[1..9].try_into().unwrap()) as u64,
                9,
            ),
            0xfe => (
                u32::from_le_bytes(bytes[1..5].try_into().unwrap()) as u64,
                5,
            ),
            0xfd => (
                u16::from_le_bytes(bytes[1..3].try_into().unwrap()) as u64,
                3,
            ),
            _ => (u8::from_le_bytes([bytes[0]]) as u64, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::varint::VarInt;

    #[test]
    fn varint() {
        assert_eq!((106, 1), VarInt::read_bytes(hex::decode("6a").unwrap()));
        assert_eq!((550, 3), VarInt::read_bytes(hex::decode("fd2602").unwrap()));
        assert_eq!(
            (998000, 5),
            VarInt::read_bytes(hex::decode("fe703a0f00").unwrap())
        );
    }
}
