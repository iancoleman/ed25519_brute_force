extern crate ed25519_dalek;
extern crate rand;
extern crate rust_sodium;
extern crate sha3;
extern crate time;

use ed25519_dalek::Keypair;
use rand::OsRng;
use rust_sodium::crypto::sign;
use sha3::Sha3_512;
use time::precise_time_ns;

fn sodium() {
   let start = precise_time_ns();
   let total_tests = 10000;
   for _ in 1..total_tests + 1 {
       sign::gen_keypair();
   }
   let duration = precise_time_ns() - start;
   println!("");
   println!("");
   println!(
       "{:?} seconds to generate {:?} sodium keys",
       (duration as f64) / 1000_000_000.0,
       total_tests
   );
   println!(
       "{:?} sodium keys per second",
       total_tests * 1000_000_000 / duration
   );
}

fn dalek() {
   let mut cspring: OsRng = OsRng::new().unwrap();
   let start = precise_time_ns();
   let total_tests = 10000;
   for _ in 1..total_tests + 1 {
       Keypair::generate::<Sha3_512>(&mut cspring);
   }
   let duration = precise_time_ns() - start;
   println!("");
   println!("");
   println!(
       "{:?} seconds to generate {:?} dalek keys",
       (duration as f64) / 1000_000_000.0,
       total_tests
   );
   println!(
       "{:?} dalek keys per second",
       total_tests * 1000_000_000 / duration
   );
}

fn main() {
   println!("Started generating keys");
   sodium();
   dalek();
}
