use anyhow::Result;

pub const INPUT_PATH: &str = "inputs/day/16/input";

pub fn part_one(contents: &str) -> Result<usize> {
    let packets = load_packets(contents)?;
    Ok(read_packet(packets.as_slice(), 0).1)
}

pub fn part_two(contents: &str) -> Result<usize> {
    let packets = load_packets(contents)?;
    Ok(read_packet(packets.as_slice(), 0).2)
}

fn read_packet(bits: &[u32], index: usize) -> (usize, usize, usize) {
    let mut i = index;
    let mut version = read_value(bits, i, 3);
    i += 3;
    let packet_type = read_value(bits, i, 3);
    i += 3;

    if packet_type == 4 {
        let (read, result) = read_literal(bits, i);
        ((i - index) + read, version, result)
    } else {
        let mut results = vec![];
        let length_type = read_value(bits, i, 1);
        i += 1;
        if length_type == 0 {
            let mut packets_bit_len = read_value(bits, i, 15);
            i += 15;
            while packets_bit_len > 0 {
                let (op_read, op_version, op_result) = read_packet(bits, i);
                i += op_read;
                results.push(op_result);
                packets_bit_len -= op_read;
                version += op_version;
            }
            (
                i - index,
                version,
                apply_operator(packet_type, results.as_slice()),
            )
        } else {
            let mut packets_len = read_value(bits, i, 11);
            i += 11;
            while packets_len > 0 {
                let (op_read, op_version, op_result) = read_packet(bits, i);
                i += op_read;
                results.push(op_result);
                packets_len -= 1;
                version += op_version;
            }
            (
                i - index,
                version,
                apply_operator(packet_type, results.as_slice()),
            )
        }
    }
}

fn read_literal(bits: &[u32], index: usize) -> (usize, usize) {
    let mut i = index;
    let mut literal = 0;
    loop {
        let loop_break = read_value(bits, i, 1) == 0;
        i += 1;
        literal = literal << 4;
        literal = literal | read_value(bits, i, 4);
        i += 4;
        if loop_break {
            break;
        }
    }
    (i - index, literal)
}

fn apply_operator(packet_type: usize, terms: &[usize]) -> usize {
    match packet_type {
        0 => terms.iter().sum(),
        1 => terms.iter().product(),
        2 => *terms.iter().min().expect("empty terms"),
        3 => *terms.iter().max().expect("empty terms"),
        // packet type 4 is for literals
        5 => {
            if terms[0] > terms[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if terms[0] < terms[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if terms[0] == terms[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("invalid packet type"),
    }
}

fn load_packets(contents: &str) -> Result<Vec<u32>> {
    Ok(contents
        .as_bytes()
        .iter()
        .flat_map(|b| hex(*b))
        .fold((Vec::new(), 7), |(mut bits, shifts), bit| {
            if shifts == 7 {
                bits.push((bit as u32) << (shifts * 4));
                (bits, shifts - 1)
            } else if let Some(store) = bits.last_mut() {
                *store = *store | ((bit as u32) << (shifts * 4));
                let shifts = if shifts == 0 { 7 } else { shifts - 1 };
                (bits, shifts)
            } else {
                // fold initializes shifts so that first if branch is always reached
                // meaning there is always at least one u32 in bits
                unreachable!();
            }
        })
        .0)
}

fn read_value(bits: &[u32], index: usize, len: usize) -> usize {
    let bits_index = index / 32;
    let bit_index = index % 32;
    if bit_index + len > 32 {
        // value to be read crosses u32 boundary
        let truncated_len = 32 - bit_index;
        let remainder_len = (bit_index + len) - 32;
        let first = read_value(bits, index, truncated_len);
        let second = read_value(bits, index + truncated_len, remainder_len);
        (first << remainder_len) | second
    } else {
        let packed_bit = bits.get(bits_index).unwrap();
        let mut bit = packed_bit << bit_index;
        bit = bit >> (32 - len);
        bit as usize
    }
}

fn hex(c: u8) -> Result<u8> {
    match c {
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(anyhow::anyhow!("invalid hex character")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_value() {
        let bits = [1108168701, 187793664];

        // inside a u32
        assert_eq!(6, read_value(&bits, 12, 3));
        // across u32s
        assert_eq!(2, read_value(&bits, 30, 3));
    }

    #[test]
    fn test_read_packet() {
        // simple literal
        assert_eq!(
            (21, 6, 2021),
            read_packet(load_packets("D2FE28").unwrap().as_slice(), 0)
        );

        // versions sum correctly
        assert_eq!(
            1 + 6 + 2,
            read_packet(load_packets("38006F45291200").unwrap().as_slice(), 0).1
        );
        assert_eq!(
            7 + 2 + 4 + 1,
            read_packet(load_packets("EE00D40C823060").unwrap().as_slice(), 0).1
        );
        assert_eq!(
            4 + 1 + 5 + 6,
            read_packet(load_packets("8A004A801A8002F478").unwrap().as_slice(), 0).1
        );

        // applies operations correctly

        assert_eq!(
            3,
            read_packet(load_packets("C200B40A82").unwrap().as_slice(), 0).2
        );
        assert_eq!(
            54,
            read_packet(load_packets("04005AC33890").unwrap().as_slice(), 0).2
        );
        assert_eq!(
            7,
            read_packet(load_packets("880086C3E88112").unwrap().as_slice(), 0).2
        );
        assert_eq!(
            9,
            read_packet(load_packets("CE00C43D881120").unwrap().as_slice(), 0).2
        );
        assert_eq!(
            1,
            read_packet(load_packets("D8005AC2A8F0").unwrap().as_slice(), 0).2
        );
        assert_eq!(
            0,
            read_packet(load_packets("F600BC2D8F").unwrap().as_slice(), 0).2
        );
        assert_eq!(
            0,
            read_packet(load_packets("9C005AC2F8F0").unwrap().as_slice(), 0).2
        );
        assert_eq!(
            1,
            read_packet(
                load_packets("9C0141080250320F1802104A08")
                    .unwrap()
                    .as_slice(),
                0
            )
            .2
        );
    }
}
