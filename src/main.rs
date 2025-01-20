// #![allow(dead_code, unused_variables)]
// there are multiple steps to implement a bloom filter in rust

// I need to read the dictonary from the disk
// then I need to wait for the word from the user
// then I need to check if the word is present in the dictonary or not

use bitvec::prelude::*;

use sha1::digest::FixedOutputReset;
use sha1::Digest;
use sha1::Sha1;
use std::env::current_dir;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Result, Write};

fn file_buf(file_path: String) -> Result<BufReader<File>> {
    let file = File::open(file_path)?;
    let buffer = BufReader::new(file);
    Ok(buffer)
}

fn get_idx<H: Digest + FixedOutputReset>(
    str: &String,
    size: usize,
    hasher: &mut H,
) -> Result<usize> {
    Digest::update(hasher, str.as_bytes());

    let result = hasher.finalize_reset(); // use finalize_reset here

    // Use the first 8 bytes of the hash to create an index
    let idx = usize::from_le_bytes(result[0..8].try_into().unwrap()) % size;

    // println!("idx {idx}");

    Ok(idx)
}

fn load_filter(buf: BufReader<File>, bit_vec: &mut BitVec, size: usize) {
    let mut hasher = Sha1::new();
    for line in buf.lines() {
        if let Ok(l) = line {
            let ll = l.trim().to_string();
            if let Ok(index) = get_idx(&ll, size, &mut hasher) {
                bit_vec.set(index, true);
            }
        }
    }
}

fn main() {
    let binding = current_dir().ok().unwrap();
    let curr_dir = binding.to_str();
    let filename = format!("{}/words.txt", curr_dir.unwrap());

    println!("filename: {}", filename);
    let buf = file_buf(filename).unwrap();

    let size = 370105;
    let mut bit_vec = bitvec![0; size];

    println!("Loading file into memory");

    load_filter(buf, &mut bit_vec, size);

    println!("Loading done");

    let mut hasher = Sha1::new();

    // println!("{:?}", bit_vec);
    loop {
        println!("......................................................");
        print!("Enter a string: ");
        stdout().flush().unwrap();
        let mut str = String::new();

        stdin().read_line(&mut str).unwrap();

        str = str.trim().to_string();

        let idx = get_idx(&str, size, &mut hasher).unwrap();
        println!("checking if the string is present in bloom filter, idx: {idx}");

        if let Some(bit) = bit_vec.get(idx).as_deref() {
            println!("Found word {} in the bloom filter: {:?}", str, bit);
        }
    }
}
