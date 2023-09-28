use super::*;

pub struct VarInt {}

pub fn encode(n: u128) -> Vec<u8> {
    if n <= 252 {
        vec![n as u8]
    } else if n <= 0xff {
        let mut push1 = vec![0xfd];
        push1.extend((n as u16).to_le_bytes());
        push1
    } else if n <= 0xffff {
        let mut push2 = vec![0xfe];
        push2.extend((n as u32).to_le_bytes());
        push2
    } else {
        let mut push4 = vec![0xff];
        push4.extend((n as u64).to_le_bytes());
        push4
    }
}

pub fn decode(buffer: &[u8]) -> Result<(u128, usize)> {
    let res = match buffer[0] {
        0xff => (u64::from_le_bytes(buffer[1..9].try_into()?) as u128, 9),
        0xfe => (u32::from_le_bytes(buffer[1..5].try_into()?) as u128, 5),
        0xfd => (u16::from_le_bytes(buffer[1..3].try_into()?) as u128, 3),
        _ => (u8::from_le_bytes([buffer[0]]) as u128, 1),
    };

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varint() {
        assert_eq!((106, 1), decode(&hex::decode("6a").unwrap()).unwrap());

        assert_eq!((550, 3), decode(&hex::decode("fd2602").unwrap()).unwrap());
        assert_eq!(
            (998000, 5),
            decode(&hex::decode("fe703a0f00").unwrap()).unwrap()
        );
    }
}
