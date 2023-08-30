use core::num;
use std::{env, i32::MAX};
use sha256::{digest, try_digest};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut numbers = 0;
    let mut zeroes = 0;

    if args[1] == "-N" {
        numbers = args[2].parse().unwrap();
        zeroes = args[4].parse().unwrap();
    } else {
        numbers = args[4].parse().unwrap();
        zeroes = args[2].parse().unwrap();
    }

    let mut zeromask = String::new();
    for i in 0..zeroes {
        zeromask += "0";
    }
    let mut found = 0;
    for number in 1..<i32>::MAX {
        let hash = digest(number.to_string());
        if &hash[hash.len()-zeroes..] == zeromask {
            println!("{}, \"{}\"", number, hash);
            found+=1;
        }

        if found == numbers {
            break;
        }
    }
}
