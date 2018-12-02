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
    pub static ref LETTER_FREQUENCIES: HashMap<u8, f32> = {
        let mut m = HashMap::new();
        m.insert(b'a', 7.52766);
        m.insert(b'e', 7.0925);
        m.insert(b'o', 5.17);
        m.insert(b'r', 4.96032);
        m.insert(b'i', 4.69732);
        m.insert(b's', 4.61079);
        m.insert(b'n', 4.56899);
        m.insert(b'1', 4.35053);
        m.insert(b't', 3.87388);
        m.insert(b'l', 3.77728);
        m.insert(b'2', 3.12312);
        m.insert(b'm', 2.99913);
        m.insert(b'd', 2.76401);
        m.insert(b'0', 2.74381);
        m.insert(b'c', 2.57276);
        m.insert(b'p', 2.45578);
        m.insert(b'3', 2.43339);
        m.insert(b'h', 2.41319);
        m.insert(b'b', 2.29145);
        m.insert(b'u', 2.10191);
        m.insert(b'k', 1.96828);
        m.insert(b'4', 1.94265);
        m.insert(b'5', 1.88577);
        m.insert(b'g', 1.85331);
        m.insert(b'9', 1.79558);
        m.insert(b'6', 1.75647);
        m.insert(b'8', 1.66225);
        m.insert(b'7', 1.621);
        m.insert(b'y', 1.52483);
        m.insert(b'f', 1.2476);
        m.insert(b'w', 1.24492);
        m.insert(b'j', 0.836677);
        m.insert(b'v', 0.833626);
        m.insert(b'z', 0.632558);
        m.insert(b'x', 0.573305);
        m.insert(b'q', 0.346119);
        m.insert(b'A', 0.130466);
        m.insert(b'S', 0.108132);
        m.insert(b'E', 0.0970865);
        m.insert(b'R', 0.08476);
        m.insert(b'B', 0.0806715);
        m.insert(b'T', 0.0801223);
        m.insert(b'M', 0.0782306);
        m.insert(b'L', 0.0775594);
        m.insert(b'N', 0.0748134);
        m.insert(b'P', 0.073715);
        m.insert(b'O', 0.0729217);
        m.insert(b'I', 0.070908);
        m.insert(b'D', 0.0698096);
        m.insert(b'C', 0.0660872);
        m.insert(b'H', 0.0544319);
        m.insert(b'G', 0.0497332);
        m.insert(b'K', 0.0460719);
        m.insert(b'F', 0.0417393);
        m.insert(b'J', 0.0363083);
        m.insert(b'U', 0.0350268);
        m.insert(b'W', 0.0320367);
        m.insert(b'.', 0.0316706);
        m.insert(b'!', 0.0306942);
        m.insert(b'Y', 0.0255073);
        m.insert(b'*', 0.0241648);
        m.insert(b'@', 0.0238597);
        m.insert(b'V', 0.0235546);
        m.insert(b'-', 0.0197712);
        m.insert(b'Z', 0.0170252);
        m.insert(b'Q', 0.0147064);
        m.insert(b'X', 0.0142182);
        m.insert(b'_', 0.0122655);
        m.insert(b'$', 0.00970255);
        m.insert(b'#', 0.00854313);
        m.insert(b',', 0.00323418);
        m.insert(b'/', 0.00311214);
        m.insert(b'+', 0.00231885);
        m.insert(b'?', 0.00207476);
        m.insert(b';', 0.00207476);
        m.insert(b'^', 0.00195272);
        m.insert(b' ', 0.00189169);
        m.insert(b'%', 0.00170863);
        m.insert(b'~', 0.00152556);
        m.insert(b'=', 0.00140351);
        m.insert(b'&', 0.00134249);
        m.insert(b'`', 0.00115942);
        m.insert(b'\\', 0.00115942);
        m.insert(b')', 0.00115942);
        m.insert(b']', 0.0010984);
        m.insert(b'[', 0.0010984);
        m.insert(b':', 0.000549201);
        m.insert(b'<', 0.000427156);
        m.insert(b'(', 0.000427156);
        m.insert(b'>', 0.000183067);
        m.insert(b'"', 0.000183067);
        m.insert(b'|', 0.000122045);
        m.insert(b'{', 0.000122045);
        m.insert(b'\'', 0.000122045);
        m.insert(b'}', 0.000061022);
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
