use rayon::prelude::*;

pub fn euler_1(limit: u32) -> u32 {
    let mut sum = 0;
    for i in 1..limit {
        if i % 5 == 0 || i % 3 == 0 {
            sum += i;
        }
    }
    sum
}

pub fn euler_1_iter(limit: u32) -> u32 {
    (1..limit).filter(|i| i % 5 == 0 || i % 3 == 0).sum()
}

pub fn euler_1_loop(limit: u32) -> u32 {
    let mut sum = 0;
    let mut threes = 3;
    let mut fives = 5;

    while threes < limit {
        sum += threes;
        threes += 3;
    }

    while fives < limit {
        sum += fives;
        fives += 5;
        if fives >= limit {
            break;
        }
        sum += fives;
        fives += 10;
    }

    sum
}

pub fn euler_1_par_iter(limit: u32) -> u32 {
    (1..limit)
        .into_par_iter()
        .filter(|i| i % 5 == 0 || i % 3 == 0)
        .sum()
}