#[allow(dead_code)]
enum Part {
    One,
    Two,
}

type Position = usize;
type Version = usize;
type TypeID = usize;

#[derive(Debug)]
enum Packet {
    Literal(Version, TypeID, Option<usize>),
    Operator(Version, TypeID, Option<Vec<(Position, Packet)>>),
}

fn hex_to_bin(val: char) -> [char; 4] {
    match val {
        '0' => ['0', '0', '0', '0'],
        '1' => ['0', '0', '0', '1'],
        '2' => ['0', '0', '1', '0'],
        '3' => ['0', '0', '1', '1'],
        '4' => ['0', '1', '0', '0'],
        '5' => ['0', '1', '0', '1'],
        '6' => ['0', '1', '1', '0'],
        '7' => ['0', '1', '1', '1'],
        '8' => ['1', '0', '0', '0'],
        '9' => ['1', '0', '0', '1'],
        'A' => ['1', '0', '1', '0'],
        'B' => ['1', '0', '1', '1'],
        'C' => ['1', '1', '0', '0'],
        'D' => ['1', '1', '0', '1'],
        'E' => ['1', '1', '1', '0'],
        'F' => ['1', '1', '1', '1'],
        _ => unreachable!(),
    }
}

fn bin_to_dec(binary: &[char]) -> usize {
    usize::from_str_radix(&binary.iter().collect::<String>(), 2).unwrap()
}

fn parse_packet(binary: &[char]) -> (Position, Packet) {
    let version = bin_to_dec(&binary[0..3]);
    let type_id = bin_to_dec(&binary[3..6]);

    match type_id {
        4 => {
            let mut chunks: Vec<char> = Vec::new();
            let mut pos = 6;
            for chunk in binary[6..].chunks_exact(5) {
                chunks.extend_from_slice(&chunk[1..5]);
                pos += 5;
                if chunk[0] == '0' {
                    break;
                }
            }
            let parsed_value = bin_to_dec(&chunks);

            (pos, Packet::Literal(version, type_id, Some(parsed_value)))
        }

        _ => {
            let mut subpackets: Vec<(Position, Packet)> = Vec::new();
            let mut pos = 7;
            match binary[6] {
                '0' => {
                    let length = bin_to_dec(&binary[7..7 + 15]);
                    pos += 15;
                    while pos - 16 < length {
                        let (size, packet) = parse_packet(&binary[pos..]);
                        pos += size;
                        subpackets.push((pos, packet));
                    }
                }
                '1' => {
                    let num_chunks = bin_to_dec(&binary[7..7 + 11]);
                    pos += 11;
                    while subpackets.len() < num_chunks {
                        let (size, packet) = parse_packet(&binary[pos..]);
                        pos += size;
                        subpackets.push((pos, packet));
                    }
                }
                _ => unreachable!(),
            }
            (pos, Packet::Operator(version, type_id, Some(subpackets)))
        }
    }
}

fn version_sum(bundle: (Position, Packet)) -> usize {
    match bundle {
        (_, Packet::Literal(version, _, _)) => version,
        (_, Packet::Operator(version, _, Some(subpackets))) => {
            version + subpackets.into_iter().map(version_sum).sum::<usize>()
        }
        _ => unreachable!(),
    }
}

fn operations(bundle: (Position, Packet)) -> usize {
    match bundle {
        (_, Packet::Literal(_, _, Some(value))) => value,
        (_, Packet::Operator(_, type_id, Some(subpackets))) => match type_id {
            0 => subpackets.into_iter().map(operations).sum(),
            1 => subpackets.into_iter().map(operations).product(),
            2 => subpackets.into_iter().map(operations).min().unwrap(),
            3 => subpackets.into_iter().map(operations).max().unwrap(),
            5 => {
                let mut sub_iter = subpackets.into_iter();
                let a = operations(sub_iter.next().unwrap());
                let b = operations(sub_iter.next().unwrap());

                (a > b) as usize
            }
            6 => {
                let mut sub_iter = subpackets.into_iter();
                let a = operations(sub_iter.next().unwrap());
                let b = operations(sub_iter.next().unwrap());

                (a < b) as usize
            }
            7 => {
                let mut sub_iter = subpackets.into_iter();
                let a = operations(sub_iter.next().unwrap());
                let b = operations(sub_iter.next().unwrap());

                (a == b) as usize
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
fn do_it(part: Part, content: &str) -> usize {
    let binary: Vec<char> = content.trim().chars().fold(vec!['0'; 0], |mut acc, v| {
        acc.extend_from_slice(&hex_to_bin(v));
        acc
    });

    let the_stuff = parse_packet(&binary);

    match part {
        Part::One => version_sum(the_stuff),
        Part::Two => operations(the_stuff),
    }
}

#[cfg(test)] // cargo test -- --show-output [TEST_NAME]
mod tests {
    use super::*;

    const SAMPLE_01: &str = "D2FE28";
    const SAMPLE_02: &str = "38006F45291200";
    const SAMPLE_03: &str = "EE00D40C823060";
    const SAMPLE_04: &str = "8A004A801A8002F478";
    const SAMPLE_05: &str = "620080001611562C8802118E34";
    const SAMPLE_06: &str = "C0015000016115A2E0802F182340";
    const SAMPLE_07: &str = "A0016C880162017C3686B18A3D4780";
    const INPUT: &str = include_str!("data/input.txt");

    #[test]
    fn part_1() {
        assert_eq!(do_it(Part::One, SAMPLE_01), 6);
        assert_eq!(do_it(Part::One, SAMPLE_02), 9);
        assert_eq!(do_it(Part::One, SAMPLE_03), 14);
        assert_eq!(do_it(Part::One, SAMPLE_04), 16);
        assert_eq!(do_it(Part::One, SAMPLE_05), 12);
        assert_eq!(do_it(Part::One, SAMPLE_06), 23);
        assert_eq!(do_it(Part::One, SAMPLE_07), 31);
        assert_eq!(do_it(Part::One, INPUT), 943);
    }

    const SAMPLE_08: &str = "C200B40A82";
    const SAMPLE_09: &str = "04005AC33890";
    const SAMPLE_10: &str = "880086C3E88112";
    const SAMPLE_11: &str = "CE00C43D881120";
    const SAMPLE_12: &str = "D8005AC2A8F0";
    const SAMPLE_13: &str = "F600BC2D8F";
    const SAMPLE_14: &str = "9C005AC2F8F0";
    const SAMPLE_15: &str = "9C0141080250320F1802104A08";

    #[test]
    fn part_2() {
        assert_eq!(do_it(Part::Two, SAMPLE_08), 3);
        assert_eq!(do_it(Part::Two, SAMPLE_09), 54);
        assert_eq!(do_it(Part::Two, SAMPLE_10), 7);
        assert_eq!(do_it(Part::Two, SAMPLE_11), 9);
        assert_eq!(do_it(Part::Two, SAMPLE_12), 1);
        assert_eq!(do_it(Part::Two, SAMPLE_13), 0);
        assert_eq!(do_it(Part::Two, SAMPLE_14), 0);
        assert_eq!(do_it(Part::Two, SAMPLE_15), 1);
        assert_eq!(do_it(Part::Two, INPUT), 167737115857);
    }
}
