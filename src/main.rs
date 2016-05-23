mod base64;
mod hex;
mod bytes;
mod lang;

use base64::to_base64;
use hex::*;
use bytes::*;

use lang::crack_xor_cypher;

// Challenge #1
const C1_HEX_STR: &'static str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const C1_BASE64_STR: &'static str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

// Challenge #2
const C2_HEX1: &'static str = "1c0111001f010100061a024b53535009181c";
const C2_HEX2: &'static str = "686974207468652062756c6c277320657965";
const C2_HEX_RES: &'static str = "746865206b696420646f6e277420706c6179";

// Challenge #3
const C3_HEX: &'static str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

fn main() {

    println!("Challenge #1 => {}",
             to_base64(&parse_hex_str(C1_HEX_STR.as_bytes())) == C1_BASE64_STR);


    println!("Challenge #2 => {}",
             to_hex_str(&xor_seqs(&parse_hex_str(C2_HEX1.as_bytes()),
                                  &parse_hex_str(C2_HEX2.as_bytes()))) == C2_HEX_RES);

    println!("Challenge #3 => {:?}",
             crack_xor_cypher(&parse_hex_str(C3_HEX.as_bytes())));

}
