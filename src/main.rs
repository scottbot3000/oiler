mod euler;

use crate::euler::*;

fn main() {
    println!("{}", euler_1(1000));
    println!("{}", euler_1_iter(1000));
    println!("{}", euler_1_loop(1000));
}
