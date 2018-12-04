use std::io::{Read, BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;

pub fn run(filename: &String) -> Result<(), Error> {
    let vec = parse_file(File::open(filename)?)?;
    part_one(&vec)?;
    part_two(&vec)?;

    Ok(())
}

fn parse_file<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let buff_reader = BufReader::new(io);
    let mut v = vec![];

    for line in buff_reader.lines() {
        v.push(line?
            .trim()
            .parse()
            .map_err(|e|Error::new(ErrorKind::InvalidData,e))?);
    }

    Ok(v)
}

// calculate a checksum by counting the number of ids with exactly two of any letter and
// the number of ids with exactly three of any letter and multiplying them together
fn part_one(vec: &Vec<String>) -> Result<(), Error> {
    let mut two_count = 0;
    let mut three_count = 0;

    for str in vec {
        let mut incr_two = false;
        let mut incr_three = false;
        let mut counts: [i32; 26] = [0; 26];
        for c in str.chars() {
            let idx = (c as u32 - 'a' as u32) as usize;
            counts[idx] += 1;
        }

        for i in &counts {
            incr_two |= *i == 2;
            incr_three |= *i == 3;
        }

        two_count += 1 & incr_two as i32;
        three_count += 1 & incr_three as i32;
    }


    println!("Part one solution: {}", two_count*three_count);

    Ok(())
}

// find letters in common between two correct box ids
fn part_two(vec: &Vec<String>) -> Result<(), Error> {
    let mut common_chars = String::new();

    // id length is constant, so first find two ids with hamming distance of one.
    'outer: for outer in vec.iter() {
        for inner in vec.iter() {
            if hamming_distance(outer, inner) == 1 {
                for iter in outer.chars().zip(inner.chars()) {
                    let (i, j) = iter;
                    if i == j {
                        common_chars.push(i);
                    }
                }
                break 'outer;
            }
        }
    }

    println!("Part two solution: {}", common_chars);

    Ok(())
}

fn hamming_distance(a: &String, b: &String) -> i32 {
    let mut dist = 0;
    for iter in a.chars().zip(b.chars()) {
        let (i, j) = iter;
        if i != j {
            dist += 1;
        }
    }
    dist
}
