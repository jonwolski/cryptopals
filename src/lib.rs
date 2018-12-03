extern crate base64;
extern crate hex;
#[macro_use]
extern crate lazy_static;
extern crate rayon;

pub mod xor;

#[allow(dead_code)]
fn hex_to_base64(input: &str) -> Result<String, hex::FromHexError> {
    use base64::encode;
    use hex::decode;
    Ok(encode(&decode(input)?))
}

#[allow(dead_code)]
fn fixed_xor(input: &[u8], mask: &[u8]) -> Vec<u8> {
    input.iter().zip(mask).map(|(i, m)| i ^ m).collect()
}

use std::collections::HashMap;

lazy_static! {
    pub static ref LETTER_FREQUENCIES: HashMap<u8, u64> = {
        let mut m = HashMap::new();
        m.insert(b'a', 752_766_000);
        m.insert(b'e', 709_250_000);
        m.insert(b'o', 517_000_000);
        m.insert(b'r', 496_032_000);
        m.insert(b'i', 469_732_000);
        m.insert(b's', 461_079_000);
        m.insert(b'n', 456_899_000);
        m.insert(b'1', 435_053_000);
        m.insert(b't', 387_388_000);
        m.insert(b'l', 377_728_000);
        m.insert(b'2', 312_312_000);
        m.insert(b'm', 299_913_000);
        m.insert(b'd', 276_401_000);
        m.insert(b'0', 274_381_000);
        m.insert(b'c', 257_276_000);
        m.insert(b'p', 245_578_000);
        m.insert(b'3', 243_339_000);
        m.insert(b'h', 241_319_000);
        m.insert(b'b', 229_145_000);
        m.insert(b'u', 210_191_000);
        m.insert(b'k', 196_828_000);
        m.insert(b'4', 194_265_000);
        m.insert(b'5', 188_577_000);
        m.insert(b'g', 185_331_000);
        m.insert(b'9', 179_558_000);
        m.insert(b'6', 175_647_000);
        m.insert(b'8', 166_225_000);
        m.insert(b'7', 162_100_000);
        m.insert(b'y', 152_483_000);
        m.insert(b'f', 124_760_000);
        m.insert(b'w', 124_492_000);
        m.insert(b'j', 083_667_700);
        m.insert(b'v', 083_362_600);
        m.insert(b'z', 063_255_800);
        m.insert(b'x', 057_330_500);
        m.insert(b'q', 034_611_900);
        m.insert(b'A', 013_046_600);
        m.insert(b'S', 010_813_200);
        m.insert(b'E', 009_708_650);
        m.insert(b'R', 008_476_000);
        m.insert(b'B', 008_067_150);
        m.insert(b'T', 008_012_230);
        m.insert(b'M', 007_823_060);
        m.insert(b'L', 007_755_940);
        m.insert(b'N', 007_481_340);
        m.insert(b'P', 007_371_500);
        m.insert(b'O', 007_292_170);
        m.insert(b'I', 007_090_800);
        m.insert(b'D', 006_980_960);
        m.insert(b'C', 006_608_720);
        m.insert(b'H', 005_443_190);
        m.insert(b'G', 004_973_320);
        m.insert(b'K', 004_607_190);
        m.insert(b'F', 004_173_930);
        m.insert(b'J', 003_630_830);
        m.insert(b'U', 003_502_680);
        m.insert(b'W', 003_203_670);
        m.insert(b'.', 003_167_060);
        m.insert(b'!', 003_069_420);
        m.insert(b'Y', 002_550_730);
        m.insert(b'*', 002_416_480);
        m.insert(b'@', 002_385_970);
        m.insert(b'V', 002_355_460);
        m.insert(b'-', 001_977_120);
        m.insert(b'Z', 001_702_520);
        m.insert(b'Q', 001_470_640);
        m.insert(b'X', 001_421_820);
        m.insert(b'_', 001_226_550);
        m.insert(b'$', 000_970_255);
        m.insert(b'#', 000_854_313);
        m.insert(b',', 000_323_418);
        m.insert(b'/', 000_311_214);
        m.insert(b'+', 000_231_885);
        m.insert(b'?', 000_207_476);
        m.insert(b';', 000_207_476);
        m.insert(b'^', 000_195_272);
        m.insert(b' ', 000_189_169);
        m.insert(b'%', 000_170_863);
        m.insert(b'~', 000_152_556);
        m.insert(b'=', 000_140_351);
        m.insert(b'&', 000_134_249);
        m.insert(b'`', 000_115_942);
        m.insert(b'\\', 000_115_942);
        m.insert(b')', 000_115_942);
        m.insert(b']', 000_109_840);
        m.insert(b'[', 000_109_840);
        m.insert(b':', 000_054_920);
        m.insert(b'<', 000_042_715);
        m.insert(b'(', 000_042_715);
        m.insert(b'>', 000_018_306);
        m.insert(b'"', 000_018_306);
        m.insert(b'|', 000_012_204);
        m.insert(b'{', 000_012_204);
        m.insert(b'\'', 000_012_204);
        m.insert(b'}', 000_006_102);
        m
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1
    #[test]
    fn hex_to_base64_test() {
        assert_eq!(
			"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
			hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap()
			);
    }

    // Exercise 2
    #[test]
    fn fixed_xor_test() {
        assert_eq!(
            hex::decode("746865206b696420646f6e277420706c6179").unwrap(),
            fixed_xor(
                &hex::decode("1c0111001f010100061a024b53535009181c").unwrap(),
                &hex::decode("686974207468652062756c6c277320657965").unwrap()
            )
        );
    }
}
