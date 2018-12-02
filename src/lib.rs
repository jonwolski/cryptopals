extern crate base64;
extern crate hex;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_to_base64_test() {
        assert_eq!(
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
            hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap()
            );
    }

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
