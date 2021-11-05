use once_cell::sync::Lazy;
use tailcall::tailcall;

pub fn encode(rep: &str) -> String {
    let mut base = rep.to_string();
    base.push_str(format!("{}", checksum(rep)).as_str());
    base
}

pub fn decode(rep: &str) -> Option<&str> {
    if is_valid(rep) {
        rep.get(..(rep.len() - 1))
    } else {
        None
    }
}

pub fn is_valid(rep: &str) -> bool {
    checksum(rep) == 0
}

static MATRIX: Lazy<[[usize; 10]; 10]> = Lazy::new(|| {
    [
        [0, 3, 1, 7, 5, 9, 8, 6, 4, 2],
        [7, 0, 9, 2, 1, 5, 4, 8, 6, 3],
        [4, 2, 0, 6, 8, 7, 1, 3, 5, 9],
        [1, 7, 5, 0, 9, 8, 3, 4, 2, 6],
        [6, 1, 2, 3, 0, 4, 5, 9, 7, 8],
        [3, 6, 7, 4, 2, 0, 9, 5, 8, 1],
        [5, 8, 6, 9, 7, 2, 0, 1, 3, 4],
        [8, 9, 4, 5, 3, 6, 2, 0, 1, 7],
        [9, 4, 3, 8, 6, 1, 7, 2, 0, 5],
        [2, 5, 8, 1, 4, 3, 6, 7, 9, 0],
    ]
});

/// Calculates the checksum from the provided string
/// Params:
/// str – a string, only the numerics will be calculated
fn checksum(rep: &str) -> usize {
    do_checksum(rep.as_bytes(), 0, 0)
}

#[tailcall]
fn do_checksum(rep: &[u8], interim: usize, idx: usize) -> usize {
    if rep.len() <= idx {
        interim
    } else {
        let c = rep[idx] as char;
        let new_interim = if c.is_digit(10) { MATRIX[interim][c as usize - 48] } else { interim };
        do_checksum(rep, new_interim, idx + 1)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::pretty::damm;

    #[test]
    fn test_calculate_check_digit() {
        let actual = damm::encode("572");
        assert_eq!(&actual, "5724");
        let actual = damm::encode("43881234567");
        assert_eq!(&actual, "438812345679");
        let with_checksum = damm::encode(&format!("{}", i64::MAX));
        assert_eq!(damm::is_valid(&with_checksum), true);
    }

    #[test]
    fn test_fail_on_checking_check_digit() {
        let with_checksum = damm::encode(&format!("{}", i64::MAX));

        for i in 0..with_checksum.len() {
            let mut sb_bytes: Vec<u8> = with_checksum.as_bytes().iter().copied().collect();
            let old_char = sb_bytes[i];
            let new_char = 47 + ((old_char + 1) % 10);
            // println!(
            //     "old_char:[{}]:[{}] new_char:[{}]:[{}]",
            //     old_char as char, old_char, new_char as char, new_char
            // );
            let item = &mut sb_bytes[i];
            *item = new_char;
            let corrupted = String::from_utf8_lossy(sb_bytes.as_slice());
            // println!("orig:[{}]\ncorr:[{}]\n", with_checksum, corrupted);
            assert_eq!(damm::is_valid(corrupted.as_ref()), false);
        }
    }
}
