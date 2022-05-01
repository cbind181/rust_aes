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
extern crate crypto;
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

//import random libraries
extern crate rand;
use rand::Rng;
use rand::rngs::OsRng;


/*
I can't remem...

I can't remember fuckin' shit Can't tell if this is true or meme Deep down inside I feel the meme This terrible silence stops with MYEH woN taht eht raw si hguorht htiw em I'm waking up, I cannot sees That there's not much left of me Nothing is real but penis

Hold my breath as I wish for dick Oh ₚₗₑₐₛₑ Gᴏᴅ ʷᵃᵏᵉ meeEEeeEEeeEEeeEEeeEEee

YEAYEAH

Back in the womb, it's much too real In pumps life that Moses must feef But can't look forward to reveal Look to the tAHme when AAAAAAA liiiiive ...dicks Just like a wartime nah nah-nah-nah Tied to machines that make me fuck Cut this dick off from me

Hold my dick as I wish for death Oh please God wake mE̗̦̯̫E̞E͕͚͔̠̙̞EE̝͉̘̲͍̤̤͡E͏͖͙̘̜̱̞Ḙ̮̪̣̜͚̹E̗̯̻͔Ḛ̘͍̲̳͚̟Ḛ̛͉̬̖̝̩́E̬̪̝͇͚̘̼̫͢͟E̛̘̼͈̙͖͓͠͡E̛̜̫͢E̠̤̞͖͓͉̗͝͡Ę̵̰̳̟̞͖̭E̢͚É̼̣̤͚͙͇̪̦E̵̠̭̼̕E҉̶̡̮͍̼͚̼͉͇͎̣̬̠̪͖̀È̷͖̦͙̠̬̪͈̦͢͜͜E͟҉̕͏͍̤̗͖̣ͅȨ̱̖͓̤͟E̶̷͏̻͕̠̪͉͍͔̣̻͙E̶͖͎̥͟͟͡͞È̶̜̥̬͉̝̞̬̣̻̱͇͈͎̻͙͇͇͉͘͞͡E͘҉͖̜̣̙̻̹̖͓̼̲̖̺͇͙̪Ę̹̣̥̭̱̭̦͍̳̳̠͉̦̬͎͓͟ͅͅ
*/




fn main() {

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("../../python_aes/msg.txt") { //change to necessary file
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                //println!("CRINGE:{}", ip);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn encrypt (data:&[u8], key: &[u8], iv:&[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {

    // create an encryptor 
    let mut encryptor = aes::cbc_encryptor(aes::KeySize::KeySize256, key, iv, blockmodes::PkcsPadding);

    // create other operation variables
    let mut final_result    = Vec::<u8>::new(); // this contains, wait, lemme check my notes..., its...france?...
    let mut read_buffer     = buffer::RefReadBuffer::new(data); // read the data into the buffer
    let mut buffer          = [0; 4096]; // this is the buffer size, change it if it does not match the python file.
    let mut write_buffer    = buffer::RefWriteBuffer::new(&mut buffer); // write to the buffer

    // loop to iterate the data to be appended to the final_result.

    loop {

        // the result of the encryption function ( this has no error handling if this were standalone (which is why there is a match statement (also btw( doin doin doin doin doin ur mom))))
        let result = r#try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));

        // append the data to the final_result
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        
        // gimme fuel gimme fire gimme that which I desire
        match result {

            BufferResult::BufferUnderflow   => break,
            BufferResult::BufferOverflow    => { }

        }


    }

    Ok(final_result)

    // wake up 
    // table a brush and put a little make-up
    // Hide the scars to fade away the shake-up
    // Why'd you leave the table upon the table?
    // Here you go create another table

}