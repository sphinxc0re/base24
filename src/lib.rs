use std::collections::BTreeMap;

const ALPHABET: &str = "ZAC2B3EF4GH5TK67P8RS9WXY";
const ALPHABET_LENGTH: usize = ALPHABET.len();

struct Base24 {
    encode_map: BTreeMap<usize, char>,
    decode_map: BTreeMap<char, usize>,
}

impl Base24 {
    pub fn new() -> Base24 {
        Base24 {
            encode_map: ALPHABET.char_indices().collect(),
            decode_map: ALPHABET
                .char_indices()
                .map(|(idx, kar)| (kar, idx))
                .chain(
                    ALPHABET
                        .to_lowercase()
                        .char_indices()
                        .map(|(idx, kar)| (kar, idx)),
                )
                .collect(),
        }
    }

    pub fn encode(&self, data: &[u8]) -> String {
        assert!(
            data.len() % 4 == 0,
            "Input data length must be a multiple of 4 bytes (32 bits)"
        );

        data.chunks(4)
            .map(|chunk| u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .map(|mut value| {
                (0..7)
                    .into_iter()
                    .map(|_| {
                        let idx: usize = value as usize % ALPHABET_LENGTH;
                        value = value / ALPHABET_LENGTH as u32;

                        self.encode_map[&idx].clone()
                    })
                    .rev()
                    .collect::<String>()
            })
            .collect()
    }

    pub fn decode(&self, data: &str) -> Vec<u8> {
        assert!(
            data.len() % 7 == 0,
            "Input data length must be a multiple of 7 chars"
        );

        let char_vec: Vec<char> = data.chars().collect();

        char_vec
            .chunks(7)
            .map(|chunks| {
                chunks.iter().fold(0u32, |acc, kar| {
                    if let Some(idx) = self.decode_map.get(kar) {
                        ALPHABET_LENGTH as u32 * acc + *idx as u32
                    } else {
                        panic!("Unsupported character in input: {:?}", kar);
                    }
                })
            })
            .flat_map(|value| value.to_be_bytes().to_vec())
            .collect()
    }
}

pub fn encode(data: &[u8]) -> String {
    Base24::new().encode(data)
}

pub fn decode(data: &str) -> Vec<u8> {
    Base24::new().decode(data)
}
