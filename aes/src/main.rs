/* This is a simple program to utilize AES CBC in Python 3
must run `pip install pycrypto` before running

credit to https://asecuritysite.com/symmetric/rust_aes?m=abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq

Authors: Cody Binder & Calen Olsen
Date: 2022-04-25

MIT License
Copyright (c) [2022] [Cody Binder, Calen Olsen]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.*/

use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use std::str;
use std::env;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn main() {
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");

    let mut message = String::from("Hello world!");
    let mut mykey =String::from("000102030405060708090A0B0C0D0E0F");

    let args: Vec<String> = env::args().collect();

        if args.len() >1 {
            message = args[1].clone();
        }

    if args.len() >2 {
            mykey = args[2].clone();
        }

    println!("Message: {}",message);
    println!("Key: {}",mykey);
    println!("IV: f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");

    let plaintext=message.as_bytes();
    let key = hex::decode(mykey).expect("Decoding failed");

    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();

    let pos = plaintext.len();

    let mut buffer = [0u8; 128];

    buffer[..pos].copy_from_slice(plaintext);

    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

    println!("\nCiphertext: {:?}",hex::encode(ciphertext));

    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let mut buf = ciphertext.to_vec();
    let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();

    println!("\nCiphertext: {:?}",str::from_utf8(decrypted_ciphertext).unwrap());
}