extern crate base64;
extern crate hamming;
extern crate hex;
extern crate itertools;
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
    pub static ref LETTER_FREQUENCIES: HashMap<u8, f32> = {
        let mut m = HashMap::new();
        m.insert(b'a', 0.752_766_000);
        m.insert(b'e', 0.709_250_000);
        m.insert(b'o', 0.517_000_000);
        m.insert(b'r', 0.496_032_000);
        m.insert(b'i', 0.469_732_000);
        m.insert(b's', 0.461_079_000);
        m.insert(b'n', 0.456_899_000);
        m.insert(b'1', 0.435_053_000);
        m.insert(b't', 0.387_388_000);
        m.insert(b'l', 0.377_728_000);
        m.insert(b'2', 0.312_312_000);
        m.insert(b'm', 0.299_913_000);
        m.insert(b'd', 0.276_401_000);
        m.insert(b'0', 0.274_381_000);
        m.insert(b'c', 0.257_276_000);
        m.insert(b'p', 0.245_578_000);
        m.insert(b'3', 0.243_339_000);
        m.insert(b'h', 0.241_319_000);
        m.insert(b'b', 0.229_145_000);
        m.insert(b'u', 0.210_191_000);
        m.insert(b'k', 0.196_828_000);
        m.insert(b'4', 0.194_265_000);
        m.insert(b'5', 0.188_577_000);
        m.insert(b'g', 0.185_331_000);
        m.insert(b'9', 0.179_558_000);
        m.insert(b'6', 0.175_647_000);
        m.insert(b'8', 0.166_225_000);
        m.insert(b'7', 0.162_100_000);
        m.insert(b'y', 0.152_483_000);
        m.insert(b'f', 0.124_760_000);
        m.insert(b'w', 0.124_492_000);
        m.insert(b'j', 0.083_667_700);
        m.insert(b'v', 0.083_362_600);
        m.insert(b'z', 0.063_255_800);
        m.insert(b'x', 0.057_330_500);
        m.insert(b'q', 0.034_611_900);
        m.insert(b'A', 0.013_046_600);
        m.insert(b'S', 0.010_813_200);
        m.insert(b'E', 0.009_708_650);
        m.insert(b'R', 0.008_476_000);
        m.insert(b'B', 0.008_067_150);
        m.insert(b'T', 0.008_012_230);
        m.insert(b'M', 0.007_823_060);
        m.insert(b'L', 0.007_755_940);
        m.insert(b'N', 0.007_481_340);
        m.insert(b'P', 0.007_371_500);
        m.insert(b'O', 0.007_292_170);
        m.insert(b'I', 0.007_090_800);
        m.insert(b'D', 0.006_980_960);
        m.insert(b'C', 0.006_608_720);
        m.insert(b'H', 0.005_443_190);
        m.insert(b'G', 0.004_973_320);
        m.insert(b'K', 0.004_607_190);
        m.insert(b'F', 0.004_173_930);
        m.insert(b'J', 0.003_630_830);
        m.insert(b'U', 0.003_502_680);
        m.insert(b'W', 0.003_203_670);
        m.insert(b'.', 0.003_167_060);
        m.insert(b'!', 0.003_069_420);
        m.insert(b'Y', 0.002_550_730);
        m.insert(b'*', 0.002_416_480);
        m.insert(b'@', 0.002_385_970);
        m.insert(b'V', 0.002_355_460);
        m.insert(b'-', 0.001_977_120);
        m.insert(b'Z', 0.001_702_520);
        m.insert(b'Q', 0.001_470_640);
        m.insert(b'X', 0.001_421_820);
        m.insert(b'_', 0.001_226_550);
        m.insert(b'$', 0.000_970_255);
        m.insert(b'#', 0.000_854_313);
        m.insert(b',', 0.000_323_418);
        m.insert(b'/', 0.000_311_214);
        m.insert(b'+', 0.000_231_885);
        m.insert(b'?', 0.000_207_476);
        m.insert(b';', 0.000_207_476);
        m.insert(b'^', 0.000_195_272);
        m.insert(b' ', 0.000_189_169);
        m.insert(b'%', 0.000_170_863);
        m.insert(b'~', 0.000_152_556);
        m.insert(b'=', 0.000_140_351);
        m.insert(b'&', 0.000_134_249);
        m.insert(b'`', 0.000_115_942);
        m.insert(b'\\', 0.000_115_942);
        m.insert(b')', 0.000_115_942);
        m.insert(b']', 0.000_109_840);
        m.insert(b'[', 0.000_109_840);
        m.insert(b':', 0.000_054_920);
        m.insert(b'<', 0.000_042_715);
        m.insert(b'(', 0.000_042_715);
        m.insert(b'>', 0.000_018_306);
        m.insert(b'"', 0.000_018_306);
        m.insert(b'|', 0.000_012_204);
        m.insert(b'{', 0.000_012_204);
        m.insert(b'\'', 0.000_012_204);
        m.insert(b'}', 0.000_006_102);
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
