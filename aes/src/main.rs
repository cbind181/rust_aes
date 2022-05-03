/* This is a simple program to utilize AES CBC in Rust

credit to https://github.com/DaGenix/rust-crypto/blob/master/examples/symmetriccipher.rs

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

extern crate crypto;
extern crate rand;
extern crate floating_duration;

use std::fs;
use std::time::Instant;
use floating_duration::TimeAsFloat;

use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

use rand::{ rngs::OsRng, RngCore };

fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {

    let mut encryptor = aes::cbc_encryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = r#try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));

        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

fn decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = r#try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

fn convert(x: usize) -> Result<f64, &'static str> {
    let result = x as f64;
    if result as usize != x {
        return Err("cannot convert")
    }
    Ok(result)
}

fn main() {
    let message = fs::read_to_string("/mnt/c/Users/ctbin/Desktop/test code bs/rust/aes2/src/msg.txt").expect("something went wrong");

    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];

    let mut rng = OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);

    let mut encrypt_times = Vec::new();
    let mut decrypt_times = Vec::new();

    // runs test 20 times and averages times, then prints best time
    for _i in 0..20 {
        let start = Instant::now();
        let encrypted_data = encrypt(message.as_bytes(), &key, &iv).ok().unwrap();
        let elapsed = start.elapsed();
        encrypt_times.push(elapsed.as_fractional_secs());
        println!("encrypt time: {:?}", elapsed);
        
        let start = Instant::now();
        let decrypted_data = decrypt(&encrypted_data[..], &key, &iv).ok().unwrap();
        let elapsed = start.elapsed();
        decrypt_times.push(elapsed.as_fractional_secs());
        println!("decrypt time: {:?}", elapsed);

        assert!(message.as_bytes() == &decrypted_data[..]);
    }

    let encrypt_sum: f64 = encrypt_times.iter().sum();
    let decrypt_sum: f64 = decrypt_times.iter().sum();

    let avg_encrypt: f64 = encrypt_sum / convert(encrypt_times.len()).expect("cannot convert");
    let avg_decrypt: f64 = decrypt_sum / convert(decrypt_times.len()).expect("cannot convert");

    println!("average encrypt time: {:?}s", avg_encrypt);
    println!("average decrypt time: {:?}s", avg_decrypt);

    println!("best encrypt time: {:?}s", encrypt_times.iter().fold(f64::INFINITY, |a, &b| a.min(b)));
    println!("best decrypt time: {:?}s", decrypt_times.iter().fold(f64::INFINITY, |a, &b| a.min(b)));
}