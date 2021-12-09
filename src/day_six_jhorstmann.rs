#![allow(warnings)]
use std::num::ParseIntError;

use anyhow::Result;

// by permission jhorstmann (jhostmann/adventofcode2021 did not have benchmarking facilities at the
// time)

pub const INPUT_PATH: &str = "inputs/day/6/input";

pub fn part_one(contents: &str) -> Result<usize> {
    Ok(part1(ages(contents)?.as_slice(), 80))
}

pub fn part_two(contents: &str) -> Result<usize> {
    Ok(part2(ages(contents)?.as_slice(), 256))
}

fn part1(ages: &[u32], days: i32) -> usize {
    let mut ages = ages.to_vec();
    let mut new_ages = Vec::with_capacity(4096);

    for _day in 0..days {
        new_ages.clear();
        for age in ages.iter() {
            if let Some(age) = age.checked_sub(1) {
                new_ages.push(age);
            } else {
                new_ages.push(6);
                new_ages.push(8);
            }
        }

        std::mem::swap(&mut ages, &mut new_ages)
    }

    ages.len()
}

fn part2(ages: &[u32], days: usize) -> usize {
    let mut histogram = [0_usize; 9];
    for age in ages.iter() {
        // per jhorstmann: unnecessary
        /*
        unsafe {
            std::intrinsics::assume((*age as usize) < histogram.len());
        }
        */
        histogram[*age as usize] += 1;
    }

    for _day in 0..days {
        let mut count = histogram[0];
        unsafe { std::ptr::copy(histogram.as_ptr().add(1), histogram.as_mut_ptr(), 8) }
        // for i in 1..histogram.len() {
        //     histogram[i - 1] = histogram[i]
        // }
        histogram[6] += count;
        histogram[8] = count;
    }

    histogram.iter().sum()
}

fn ages(data: &str) -> std::result::Result<Vec<u32>, ParseIntError> {
    data.split(",").map(|n| n.parse()).collect()
}
