extern crate rand_core;
use chrono::{Datelike};
use ed25519_dalek::*;


use rand_core::OsRng;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;

const MAX_ITER: usize = 10000000;
const NUM_THREADS: usize = 8;

fn main() {
    let result = Arc::new(Mutex::new(None));

    let threads: Vec<_> = (0..NUM_THREADS).map(|_| {
        let result = Arc::clone(&result);
        thread::spawn(move || {
            let mut iter = 0;
            loop {
                let keypair: Keypair = Keypair::generate(&mut OsRng);
                let public_key = keypair.public;
                let secret_key = keypair.secret;
                if validate_key(hex::encode(public_key.as_bytes()).as_str()) {
                    *result.lock().unwrap() = Some((public_key, secret_key));
                    break;
                }

                iter += 1;
                if iter % 10000 == 0 {
                    println!("{} iterations in thread {:?}", iter, thread::current().id());
                }
                if iter == MAX_ITER / NUM_THREADS {
                    println!("No valid key found for thread {:?}", thread::current().id());
                    break;
                }

                if result.lock().unwrap().is_some() {
                    break;
                }
            }
        })
    }).collect();

    for t in threads {
        t.join().unwrap();
    }

    let result = result.lock().unwrap();

    if let Some((public_key, secret_key)) = result.as_ref() {
        println!("Valid key found");
        let filename = "./keypair.txt";
        let contents = format!("Secret key:\n{}\nPublic key:\n{}\n", hex::encode(secret_key.as_bytes()), hex::encode(public_key.as_bytes()));
        fs::write(filename, contents).expect("Unable to write file");
    }
}


// fn sign(){
//     let html = "<html>\n\
//     asd\n\
//     <time datetime=\"2023-06-20T10:39:35Z\">\n\
// </html>\n\
// ";
// println!("{}", html);
// }


pub fn validate_key(key: &str) -> bool {
    // [a-zA-Z0-9]+83e(0[1-9]|1[0-2])(\d\d)$
    let re = regex::Regex::new(r"^[a-zA-Z0-9]+83e(0[1-9]|1[0-2])(\d\d)$").unwrap();
    let key_len = key.len();
    if !re.is_match(key) || key_len != 64 {
        return false;
    }
    let month = &key[64 - 4..64 - 2];
    let year = &key[64 - 2..64];
    let month: u8 = month.parse().unwrap();
    let year: u8 = year.parse().unwrap();

    let current_year = ((chrono::Local::now().year()) % 100) as u8;
    let current_month = chrono::Local::now().month() as u8;

    if year < current_year - 2 {
        return false;
    }
    if year > current_year {
        return false;
    }

    if year == current_year - 2 && month < current_month {
        return false;
    }

    if year == current_year && month > current_month {
        return false;
    }
    return true;
}