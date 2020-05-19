/* AES ENCRYTION ALGORITHM IMPLEMENTED FOR ACADEMIC PURPOSES
   Audit before using in production 
 */

use std::io;
use rand::Rng;
use bytes::Bytes;
extern crate ndarray;

const ROTL8: u32 = 0xABAD1DEA;



enum KeySize{

}

enum BlockCipherMode {

}
/*
 * Generates a S-Box with average correlation properties.
 * Designed to be resistant to linerar and differntial cryptanalysis
 *
 */
fn generate_aes_sbox(sbox: &mut Vec<u8>) -> bool {
	return true;
}

struct State {
	mode: BlockCipherMode,
	address: String,
}

fn string_to_bytes(){

}

fn bytes_to_string(){

}

fn convert_to_matrix(){

}

fn key_expansion(key: u32, word: u32) {

}

fn substitute_bytes(){
	println!("Substitute key");
}

fn permutate_bytes(){
	println!("Permutate key");
}

fn mix_column(){

}

fn add_round_key(key: u32, key2: u32){

}

fn padding(strData: &str) -> String {
	let strlen      = strData.len();
	let padlen      = 128 - (strlen % 128);
	let mut padtext = String::new();
	if padlen % 2 == 0 {
		padtext = "a".repeat(padlen);
	} else {
		padtext = "b".repeat(padlen);
	}
	let returntext = format!("{}{}", strData, padtext);

	return returntext
}

fn encrypt(){

}

fn decrypt(){

}

fn main() {

    println!("Hello, Rusty!");
    let mut plaintext  = String::new();
    let mut cyphertext = String::new();
    let mut key 	   = String::new();
    println!("Please input your key.");
	io::stdin().read_line(&mut key)
		.expect("Failed to read line");
	key.pop();
	let mut key_clone = key.clone();
	
	let bytes = key.into_bytes();
	for x in bytes.iter() {
		format!("{:b}", x);
		println!("loop {}",x);
    }
    let name = padding(&key_clone);
    println!("Strings {}", name);
    println!("byte output {:?}", bytes);
    let keylength = name.len();
	println!("Key length {}",keylength);


    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
}
