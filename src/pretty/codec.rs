use once_cell::sync::Lazy;
use tailcall::tailcall;

pub trait Codec {
    fn encode(&self, number: i64) -> String;
    fn decode(&self, value: &str) -> i64;
}

#[derive(Debug, Clone)]
pub struct AlphabetCodec(Alphabet);

impl Default for AlphabetCodec {
    fn default() -> Self {
        AlphabetCodec::new(BASE_23.clone())
    }
}

impl AlphabetCodec {
    pub fn new(alphabet: Alphabet) -> Self {
        Self(alphabet)
    }
}

#[derive(Debug, Default)]
struct ResultWithIndex {
    pub result: i64,
    pub pos: usize,
}

impl ResultWithIndex {
    pub fn increment_w_result(self, result: i64) -> Self {
        Self { result, pos: self.pos + 1 }
    }
}

impl Codec for AlphabetCodec {
    fn encode(&self, number: i64) -> String {
        do_encode(&self.0, number, String::default())
    }

    fn decode(&self, value: &str) -> i64 {
        value
            .chars()
            .rev()
            .fold(ResultWithIndex::default(), |acc, c| {
                let encoded_part = self.0.index_of(c) as i64;
                let base_placement = (self.0.base as i64).pow(acc.pos as u32);
                let acc_inc = encoded_part * base_placement;
                let new_acc = acc.result + acc_inc;
                acc.increment_w_result(new_acc)
            })
            .result
    }
}

#[tailcall]
fn do_encode(alphabet: &Alphabet, number: i64, mut acc: String) -> String {
    let modulo = (number % alphabet.base as i64) as usize;
    let part = alphabet.value_of(modulo);
    acc.insert(0, part);
    if number < alphabet.base as i64 {
        acc
    } else {
        do_encode(alphabet, number / alphabet.base as i64, acc)
    }
}

#[derive(Debug, Clone)]
pub struct Alphabet {
    pub elements: String,
    pub base: usize,
}

static BASE_23: Lazy<Alphabet> = Lazy::new(|| Alphabet::new("ABCDEFGHJKLMNPQRSTUVXYZ"));

impl Alphabet {
    pub fn new(base: impl Into<String>) -> Self {
        let elements = base.into();
        let base = elements.len();
        Self { elements, base }
    }

    pub fn value_of(&self, pos: usize) -> char {
        self.elements
            .chars()
            .nth(pos)
            .expect("failed on attempted out-of-bounds access.")
    }

    pub fn index_of(&self, c: char) -> usize {
        let pos = self.elements.chars().position(|a| a == c);
        pos.expect("failed to id character in alphabet")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    static CODEC: Lazy<AlphabetCodec> = Lazy::new(|| AlphabetCodec::default());

    #[test]
    fn test_encode_value() {
        assert_eq!(CODEC.encode(23), "BA".to_string());
        assert_eq!(CODEC.encode(529), "BAA".to_string());
        assert_eq!(CODEC.encode(12167), "BAAA".to_string());
    }

    #[test]
    fn test_decode_value() {
        assert_eq!(CODEC.decode("BA"), 23);
        assert_eq!(CODEC.decode("ABA"), 23);
        assert_eq!(CODEC.decode("BAA"), 529);
        assert_eq!(CODEC.decode("BAB"), 530);
        assert_eq!(CODEC.decode("BAAA"), 12167);
        assert_eq!(CODEC.decode("HAPK"), 85477);
        assert_eq!(CODEC.decode("HPJD"), 92233);
    }
}
