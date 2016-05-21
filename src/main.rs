mod base64;
mod hex;

use base64::to_base64;
use hex::hex_to_string;

// Challenge #1
const C1_HEX_STR: &'static str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const C1_BASE64_STR: &'static str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

fn main() {

    println!("Challenge #1 => {}",
             to_base64(&hex_to_string(C1_HEX_STR)) == C1_BASE64_STR);

}
