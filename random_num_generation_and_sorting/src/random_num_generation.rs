extern crate rand;
use rand::Rng;

pub fn generate_random_nums(){
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

pub fn generate_random_integer_with_ranges(min: i32, max: i32){
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(min, max));
}

pub fn generate_random_float_with_ranges(min: f32, max: f32){
    let mut rng = rand::thread_rng();
    println!("float: {}", rng.gen_range(min, max));
}