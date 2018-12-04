use std::fs::File;
use std::io::prelude::*;
use std::io::{Read, BufReader, Error, ErrorKind};
use std::collections::HashSet;


pub fn run(filename: &String) -> Result<(), Error> {
    let vec = parse_file(File::open(filename)?)?;
    part_one(&vec)?;
    part_two(&vec)?;

    Ok(())
}

fn parse_file<R: Read>(io: R) -> Result<Vec<i64>, Error> {
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

// given input list of frequency changes, output the final frequency
fn part_one(vec: &Vec<i64>) -> Result<(), Error> {
    let sum = vec.iter().fold(0, |a, &b| a + b);

    println!("Part one solution: {}", sum);
    Ok(())
}

// given input list of frequency changes, output the first frequency reached twice
fn part_two(vec: &Vec<i64>) -> Result<(), Error> {
    let mut frequencies = HashSet::new();
    let mut curr_freq = 0;

    'outer: loop {
        for i in vec {
            if frequencies.contains(&curr_freq) {
                break 'outer;
            }
            frequencies.insert({curr_freq});
            curr_freq += i;
        }
    }

    println!("Part two solution: {}", curr_freq);
    Ok(())
}