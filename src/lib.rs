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
                dbg!(value);
                (0..7)
                    .into_iter()
                    .map(|_| {
                        let idx: usize = value as usize % ALPHABET_LENGTH;
                        value = value / ALPHABET_LENGTH as u32;

                        self.encode_map[&idx].clone()
                    })
                    .collect::<Vec<char>>()
                    .iter()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        // A few hard coded values
        let values: Vec<(Vec<u8>, _)> = vec![
            ("00000000", "ZZZZZZZ"),
            ("000000000000000000000000", "ZZZZZZZZZZZZZZZZZZZZZ"),
            ("00000001", "ZZZZZZA"),
            ("000000010000000100000001", "ZZZZZZAZZZZZZAZZZZZZA"),
            ("00000010", "ZZZZZZP"),
            ("00000030", "ZZZZZCZ"),
            ("88553311", "5YEATXA"),
            ("FFFFFFFF", "X5GGBH7"),
            ("FFFFFFFFFFFFFFFFFFFFFFFF", "X5GGBH7X5GGBH7X5GGBH7"),
            ("FFFFFFFFFFFFFFFFFFFFFFFF", "x5ggbh7x5ggbh7x5ggbh7"),
            ("1234567887654321", "A64KHWZ5WEPAGG"),
            ("1234567887654321", "a64khwz5wepagg"),
            ("FF0001FF001101FF01023399", "XGES63FZZ247C7ZC2ZA6G"),
            ("FF0001FF001101FF01023399", "xges63fzz247c7zc2za6g"),
            (
                "25896984125478546598563251452658",
                "2FC28KTA66WRST4XAHRRCF237S8Z",
            ),
            (
                "25896984125478546598563251452658",
                "2fc28kta66wrst4xahrrcf237s8z",
            ),
            ("00000001", "ZZZZZZA"),
            ("00000002", "ZZZZZZC"),
            ("00000004", "ZZZZZZB"),
            ("00000008", "ZZZZZZ4"),
            ("00000010", "ZZZZZZP"),
            ("00000020", "ZZZZZA4"),
            ("00000040", "ZZZZZCP"),
            ("00000080", "ZZZZZ34"),
            ("00000100", "ZZZZZHP"),
            ("00000200", "ZZZZZW4"),
            ("00000400", "ZZZZARP"),
            ("00000800", "ZZZZ2K4"),
            ("00001000", "ZZZZFCP"),
            ("00002000", "ZZZZ634"),
            ("00004000", "ZZZABHP"),
            ("00008000", "ZZZC4W4"),
            ("00010000", "ZZZB8RP"),
            ("00020000", "ZZZG5K4"),
            ("00040000", "ZZZRYCP"),
            ("00080000", "ZZAKX34"),
            ("00100000", "ZZ229HP"),
            ("00200000", "ZZEFPW4"),
            ("00400000", "ZZT7GRP"),
            ("00800000", "ZAAESK4"),
            ("01000000", "ZCCK7CP"),
            ("02000000", "ZB32E34"),
            ("04000000", "Z4HETHP"),
            ("08000000", "ZP9KZW4"),
            ("10000000", "AG8CARP"),
            ("20000000", "CSHB2K4"),
            ("40000000", "3694FCP"),
            ("80000000", "53PP634"),
        ]
        .iter()
        .map(|(str_data, b24_str)| {
            let char_vec: Vec<_> = str_data.chars().collect();

            let data: Vec<u8> = char_vec
                .chunks(2)
                .map(|chunk| {
                    let byte_str = format!("{}{}", chunk[0], chunk[1]);

                    u8::from_str_radix(&byte_str, 16)
                })
                .filter_map(|res| res.ok())
                .collect();

            (data, b24_str.to_string())
        })
        .collect();

        for (data, b24_str) in values {
            let decoded = decode(&b24_str);
            assert_eq!(decoded, data);
            assert_eq!(encode(&decoded), b24_str.to_uppercase());
        }
    }

    #[test]
    fn random_test() {
        use rand::distributions::Standard;
        use rand::{thread_rng, Rng};

        let rng = thread_rng();

        for _ in 0..100 {
            let data: Vec<u8> = rng.sample_iter(Standard).take(64).collect();
            assert_eq!(decode(&encode(&data)), data);
        }
    }
}
