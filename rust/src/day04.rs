extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn _run() 
{
    let key = "ckczppom".as_bytes();
    let mut n = 1;

    let mut hasher = Md5::new();
    let mut part1 : i32 = -1;
    let part2 : i32;

    loop 
    {
        hasher.input(key);
        hasher.input(n.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        // "000000" is 2.5 bytes of 0
        if output[0] == 0 && output[1] == 0 && output[2] & 0xF0 == 0 {
            if part1 == -1 { 
                part1 = n; 
            }
            // "0000000" is 3 bytes of 0
            if output[2] == 0 {
                part2 = n;
                break;
            }
        }

        n += 1;
        hasher.reset();
    }

    println!("day04-1: {}", part1);
    println!("day04-2: {}", part2);
}