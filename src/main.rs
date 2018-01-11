extern crate rust_sodium;
extern crate time;

use rust_sodium::crypto::sign;
use time::precise_time_ns;

fn main() {
    println!("Started generating keys");
    let start =  precise_time_ns();
    let total_tests = 10000;
    for _ in 1..total_tests+1 {
        sign::gen_keypair();
    }
    let duration =  precise_time_ns() - start;
    println!("");
    println!("");
    println!("{:?} seconds to generate {:?} keys", (duration as f64) / 1000_000_000.0, total_tests);
    println!("{:?} keys per second", total_tests * 1000_000_000 / duration);
}
