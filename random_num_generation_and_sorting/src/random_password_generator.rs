extern crate rand;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn generate_random_password(length: i32){
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .collect();

    println!("{}", rand_string);
}