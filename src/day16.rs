pub fn solve() -> (usize, usize) {
    let hex = std::fs::read_to_string("res/day16.txt").unwrap();
    let packet = Packet::from_hex(&hex).expect("Could not parse packet").0;

    (packet.version(), packet.value())
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Value {
        version: usize,
        value: usize,
    },

    Operator {
        version: usize,
        operation: u8,
        subpackets: Vec<Packet>,
    },
}

impl Packet {
    pub fn version(&self) -> usize {
        match self {
            Self::Value { version, .. } => *version,
            Self::Operator {
                version,
                subpackets,
                ..
            } => subpackets.iter().map(|p| p.version()).sum::<usize>() + *version,
        }
    }

    pub fn value(&self) -> usize {
        match self {
            Self::Value { value, .. } => *value,
            Self::Operator {
                subpackets,
                operation,
                ..
            } => match operation {
                0 => subpackets.iter().map(|p| p.value()).sum(),
                1 => subpackets.iter().map(|p| p.value()).product(),
                2 => subpackets.iter().map(|p| p.value()).min().unwrap(),
                3 => subpackets.iter().map(|p| p.value()).max().unwrap(),
                5 => (subpackets[0].value() > subpackets[1].value()).into(),
                6 => (subpackets[0].value() < subpackets[1].value()).into(),
                7 => (subpackets[0].value() == subpackets[1].value()).into(),
                e => panic!("Unexpected operation '{}'", e),
            },
        }
    }

    pub fn from_hex(bytes: &str) -> Option<(Self, usize)> {
        Self::from_bits(
            &bytes
                .chars()
                .map(|f| format!("{:04b}", f.to_digit(16).unwrap()))
                .collect::<String>(),
        )
    }

    pub fn from_bits(bits: &str) -> Option<(Self, usize)> {
        // 11 is the shortest possible packet, which represents a 4-bit value
        if bits.len() < 11 {
            return None;
        }

        let version = usize::from_str_radix(&bits[0..3], 2).unwrap();

        let operation = u8::from_str_radix(&bits[3..6], 2).unwrap();

        match operation {
            4 => {
                let mut i = 6;
                let mut num = String::new();

                loop {
                    num.push_str(&bits[i + 1..i + 5]);
                    match &bits[i..i + 1] {
                        "0" => {
                            i += 5;
                            break;
                        }
                        "1" => i += 5,
                        c => panic!("Unexpeted character '{}' encountered", c),
                    }
                }

                Some((
                    Self::Value {
                        version,
                        value: usize::from_str_radix(&num, 2).unwrap(),
                    },
                    i,
                ))
            }
            // Some Operator
            _ => {
                let mut subpackets = vec![];

                let read_bits = match &bits[6..7] {
                    "0" => {
                        let mut n_bits = usize::from_str_radix(&bits[7..22], 2).unwrap();
                        let mut i = 22;

                        while n_bits > 0 {
                            let (subpacket, read_bits) = Self::from_bits(&bits[i..])?;
                            subpackets.push(subpacket);
                            n_bits -= read_bits;
                            i += read_bits;
                        }

                        i
                    }
                    "1" => {
                        let n_subpacket = usize::from_str_radix(&bits[7..18], 2).unwrap();
                        let mut i = 18;

                        for _ in 0..n_subpacket {
                            let (subpacket, read_bits) = Self::from_bits(&bits[i..])?;
                            subpackets.push(subpacket);
                            i += read_bits;
                        }

                        i
                    }
                    c => panic!("Unexpeted character '{}' encountered", c),
                };

                Some((
                    Self::Operator {
                        version,
                        subpackets,
                        operation,
                    },
                    read_bits,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn examples() {
        assert_eq!(
            Packet::from_hex("D2FE28"),
            Some((
                Packet::Value {
                    version: 6,
                    value: 2021,
                },
                21
            ))
        );

        assert_eq!(
            Packet::from_hex("38006F45291200"),
            Some((
                Packet::Operator {
                    version: 1,
                    operation: 6,
                    subpackets: vec![
                        Packet::Value {
                            version: 6,
                            value: 10,
                        },
                        Packet::Value {
                            version: 2,
                            value: 20,
                        }
                    ]
                },
                49
            ))
        );

        assert_eq!(
            Packet::from_hex("EE00D40C823060"),
            Some((
                Packet::Operator {
                    version: 7,
                    operation: 3,
                    subpackets: vec![
                        Packet::Value {
                            version: 2,
                            value: 1,
                        },
                        Packet::Value {
                            version: 4,
                            value: 2,
                        },
                        Packet::Value {
                            version: 1,
                            value: 3,
                        }
                    ]
                },
                51
            ))
        );

        assert_eq!(
            Packet::from_hex("8A004A801A8002F478").unwrap().0.version(),
            16
        );

        assert_eq!(
            Packet::from_hex("620080001611562C8802118E34")
                .unwrap()
                .0
                .version(),
            12
        );

        assert_eq!(
            Packet::from_hex("C0015000016115A2E0802F182340")
                .unwrap()
                .0
                .version(),
            23
        );

        assert_eq!(
            Packet::from_hex("A0016C880162017C3686B18A3D4780")
                .unwrap()
                .0
                .version(),
            31
        );

        assert_eq!(Packet::from_hex("C200B40A82").unwrap().0.value(), 3);

        assert_eq!(Packet::from_hex("04005AC33890").unwrap().0.value(), 54);

        assert_eq!(Packet::from_hex("880086C3E88112").unwrap().0.value(), 7);
        assert_eq!(Packet::from_hex("CE00C43D881120").unwrap().0.value(), 9);
        assert_eq!(Packet::from_hex("D8005AC2A8F0").unwrap().0.value(), 1);
        assert_eq!(Packet::from_hex("F600BC2D8F").unwrap().0.value(), 0);
        assert_eq!(Packet::from_hex("9C005AC2F8F0").unwrap().0.value(), 0);
        assert_eq!(
            Packet::from_hex("9C0141080250320F1802104A08")
                .unwrap()
                .0
                .value(),
            1
        );
    }
}
