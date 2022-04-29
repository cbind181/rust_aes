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

//import STD libraries
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//import cryptographic libraries
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use hex_literal::hex;

//import random libraries
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

//assign randomized key

const KEY:  &str = "EX[\xc8\xd5\xbfI{\xa2$\x05(\xd5\x18\xbf\xc0\x85)\x10nc\x94\x02)j\xdf\xcb\xc4\x94\x9d(\x9e";
const IV:   [i32; 16] = [0x24; 16];

fn main() {
    
    type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
    type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("../../../python_aes/msg.txt") { //change to necessary file
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                //code logic
                println!("CRINGE:{}", ip);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}